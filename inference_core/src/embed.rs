use std::pin::Pin;
use std::sync::Arc;
use anyhow::anyhow;
use ndarray::{Axis};
use std::mem::ManuallyDrop;

use ort::{Environment, ExecutionProvider, GraphOptimizationLevel, LoggingLevel, SessionBuilder};
use crate::embedding::Embedding;

pub struct Semantic {
    model_ref: &'static [u8],

    tokenizer: Arc<tokenizers::Tokenizer>,
    session: Arc<ort::Session>,
}

impl Drop for Semantic {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut ManuallyDrop::new(self.model_ref));
        }
    }
}

impl Semantic {
    pub async fn initialize(model: Vec<u8>, tokenizer_data: Vec<u8>) -> Result<Pin<Box<Semantic>>, anyhow::Error> {
        let environment = Arc::new(
            Environment::builder()
                .with_name("Encode")
                .with_log_level(LoggingLevel::Warning)
                .with_execution_providers([ExecutionProvider::CPU(Default::default())])
                .build()?,
        );

        let threads = if let Ok(v) = std::env::var("NUM_OMP_THREADS") {
            str::parse(&v).unwrap_or(1)
        } else {
            1
        };

        let tokenizer: Arc<tokenizers::Tokenizer> = tokenizers::Tokenizer::from_bytes(tokenizer_data).map_err(|e| anyhow!("tok frombytes error: {}", e))?.into();

        let model_ref = model.leak();

        let semantic = Self {
            model_ref,
            tokenizer,
            session: SessionBuilder::new(&environment)?
                .with_optimization_level(GraphOptimizationLevel::Level3)?
                .with_intra_threads(threads)?
                .with_model_from_memory(model_ref)
                .unwrap()
                .into(),
        };

        Ok(Box::pin(semantic))
    }

    pub fn embed(&self, sequence: &str) -> anyhow::Result<Embedding> {
        let encoding = self.tokenizer.encode(sequence, true).unwrap();

        let input_ids = encoding.get_ids().iter().map(|item| *item as i64).collect::<Vec<_>>();
        let attention_mask = encoding.get_attention_mask().iter().map(|item| *item as i64).collect::<Vec<_>>();
        let token_type_ids = encoding.get_type_ids().iter().map(|item| *item as i64).collect::<Vec<_>>();

        // Run inference
        let sequence_length = input_ids.len();

        let input_ids = ndarray::CowArray::from(&input_ids)
            .into_shape((1, sequence_length))
            .unwrap()
            .into_dyn();
        let input_ids = ndarray::CowArray::from(&input_ids)
            .into_shape((1, sequence_length))
            .unwrap()
            .into_dyn();
        let input_ids = ort::Value::from_array(None, &input_ids).unwrap();

        println!("input_ids: {:?}", input_ids);

        let attention_mask = ndarray::CowArray::from(&attention_mask)
            .into_shape((1, sequence_length))
            .unwrap()
            .into_dyn();
        let attention_mask = ort::Value::from_array(None, &attention_mask).unwrap();

        println!("attention_mask: {:?}", attention_mask);

        let token_type_ids = ndarray::CowArray::from(&token_type_ids)
            .into_shape((1, sequence_length))
            .unwrap()
            .into_dyn();
        let token_type_ids = ort::Value::from_array(None, &token_type_ids).unwrap();

        println!("token_type_ids: {:?}", token_type_ids);

        let outputs = self.session
            .run(ort::inputs![input_ids, attention_mask, token_type_ids].unwrap())
            .unwrap();

        let output_tensor = outputs[0].extract_tensor::<f32>().unwrap();
        let sequence_embedding = &*output_tensor.view();
        let pooled = sequence_embedding.mean_axis(Axis(1)).unwrap();

        Ok(pooled.to_owned().as_slice().unwrap().to_vec())
    }
}
