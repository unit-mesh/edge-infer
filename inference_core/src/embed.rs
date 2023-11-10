use std::pin::Pin;
use std::sync::Arc;
use anyhow::anyhow;
use ndarray::{Axis};

use ort::{Environment, ExecutionProvider, GraphOptimizationLevel, LoggingLevel, SessionBuilder};

pub struct Semantic {
    #[allow(dead_code)]
    model: Vec<u8>,

    tokenizer: Arc<tokenizers::Tokenizer>,
    session: Arc<ort::Session>,
}

pub type Embedding = Vec<f32>;

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

        let data_ref: &[u8] = unsafe { &*(model.as_slice() as *const [u8]) };

        let semantic = Self {
            model,
            tokenizer,
            session: SessionBuilder::new(&environment)?
                .with_optimization_level(GraphOptimizationLevel::Level3)?
                .with_intra_threads(threads)?
                .with_model_from_memory(data_ref)
                .unwrap()
                .into(),
        };

        Ok(Box::pin(semantic))
    }

    pub fn embed(&self, sequence: &str) -> anyhow::Result<Embedding> {
        let mut encodings = self.tokenizer.encode_batch(vec![sequence], true).unwrap();

        // let length = tokenizer_output.get_ids().len();

        let (input_ids, attention_mask, token_type_ids) = encodings.iter_mut().fold(
            (Vec::new(), Vec::new(), Vec::new()),
            |(mut input_ids, mut attention_mask, mut token_type_ids), encoding| {
                encoding.pad(
                    512,
                    u32::MAX,
                    u32::MAX,
                    "[PAD]",
                    tokenizers::PaddingDirection::Right,
                );
                encoding.truncate(512, 0, tokenizers::TruncationDirection::Right);
                input_ids.extend(encoding.get_ids().iter().map(|item| *item as i64));
                attention_mask.extend(
                    encoding
                        .get_attention_mask()
                        .iter()
                        .map(|item| *item as i64),
                );
                token_type_ids.extend(encoding.get_type_ids().iter().map(|item| *item as i64));
                (input_ids, attention_mask, token_type_ids)
            },
        );

        // Run inference
        let batch_size = encodings.len();
        let sequence_length = 512;

        let input_ids = ndarray::CowArray::from(&input_ids)
            .into_shape((batch_size, sequence_length))
            .unwrap()
            .into_dyn();
        let input_ids = ndarray::CowArray::from(&input_ids)
            .into_shape((batch_size, sequence_length))
            .unwrap()
            .into_dyn();
        let input_ids = ort::Value::from_array(None, &input_ids).unwrap();

        print!("input_ids: {:?}", input_ids);

        let attention_mask = ndarray::CowArray::from(&attention_mask)
            .into_shape((batch_size, sequence_length))
            .unwrap()
            .into_dyn();
        let attention_mask = ort::Value::from_array(None, &attention_mask).unwrap();

        print!("attention_mask: {:?}", attention_mask);

        let token_type_ids = ndarray::CowArray::from(&token_type_ids)
            .into_shape((batch_size, sequence_length))
            .unwrap()
            .into_dyn();
        let token_type_ids = ort::Value::from_array(None, &token_type_ids).unwrap();

        print!("token_type_ids: {:?}", token_type_ids);

        let outputs = self.session.run(ort::inputs![input_ids, attention_mask, token_type_ids].unwrap()).unwrap();

        let output_tensor = outputs[0].extract_tensor::<f32>().unwrap();
        let sequence_embedding = &*output_tensor.view();
        let pooled = sequence_embedding.mean_axis(Axis(1)).unwrap();

        Ok(pooled.to_owned().as_slice().unwrap().to_vec())
    }
}
