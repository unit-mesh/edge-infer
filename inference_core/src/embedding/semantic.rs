use std::fmt::{Display, Formatter};
use std::mem::ManuallyDrop;
use std::pin::Pin;
use std::sync::Arc;

use ndarray::Axis;
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

pub fn init_semantic(model: Vec<u8>, tokenizer_data: Vec<u8>) -> Result<Arc<Semantic>, SemanticError> {
    let result = Semantic::init_semantic(model, tokenizer_data)?;
    Ok(Arc::new(result))
}

pub fn init_semantic_with_path(model_path: &str, tokenizer_path: &str) -> Result<Arc<Semantic>, SemanticError> {
    let model = std::fs::read(model_path).map_err(|_| SemanticError::InitModelReadError)?;
    let tokenizer_data = std::fs::read(tokenizer_path).map_err(|_| SemanticError::InitTokenizerReadError)?;

    let result = Semantic::init_semantic(model, tokenizer_data)?;
    Ok(Arc::new(result))
}

impl Semantic {
    pub async fn initialize(model: Vec<u8>, tokenizer_data: Vec<u8>) -> Result<Pin<Box<Semantic>>, SemanticError> {
        let semantic = Self::init_semantic(model, tokenizer_data)?;

        Ok(Box::pin(semantic))
    }

    pub fn init_semantic(model: Vec<u8>, tokenizer_data: Vec<u8>) -> Result<Semantic, SemanticError> {
        let environment = Arc::new(
            Environment::builder()
                .with_name("Encode")
                .with_log_level(LoggingLevel::Warning)
                .with_execution_providers([ExecutionProvider::CPU(Default::default())])
                .build().map_err(|e| SemanticError::InitBuildOrtEnv)?,
        );

        let threads = if let Ok(v) = std::env::var("NUM_OMP_THREADS") {
            str::parse(&v).unwrap_or(1)
        } else {
            1
        };

        let tokenizer: Arc<tokenizers::Tokenizer> = tokenizers::Tokenizer::from_bytes(tokenizer_data)
            .map_err(|e| SemanticError::TokenizeEncodeByteError)?.into();

        let model_ref = model.leak();

        let semantic = Self {
            model_ref,
            tokenizer,
            session: SessionBuilder::new(&environment).map_err(|e| SemanticError::InitSessionBuilder)?
                .with_optimization_level(GraphOptimizationLevel::Level3).map_err(|e| SemanticError::InitSessionOptimization)?
                .with_intra_threads(threads).map_err(|e| SemanticError::InitSessionThreads)?
                .with_model_from_memory(model_ref)
                .unwrap()
                .into(),
        };
        Ok(semantic)
    }

    pub fn embed(&self, sequence: &str) -> Result<Embedding, SemanticError> {
        let encoding = self.tokenizer.encode(sequence, true)
            .map_err(|_| SemanticError::TokenizeEncodeError)?;

        let input_ids = encoding.get_ids().iter().map(|item| *item as i64).collect::<Vec<_>>();
        let attention_mask = encoding.get_attention_mask().iter().map(|item| *item as i64).collect::<Vec<_>>();
        let token_type_ids = encoding.get_type_ids().iter().map(|item| *item as i64).collect::<Vec<_>>();

        // Run inference
        let sequence_length = input_ids.len();

        let input_ids = ndarray::CowArray::from(&input_ids)
            .into_shape((1, sequence_length))
            .map_err(|_| SemanticError::ShapeError)?
            .into_dyn();

        let input_ids = ndarray::CowArray::from(&input_ids)
            .into_shape((1, sequence_length))
            .map_err(|_| SemanticError::ShapeError)?
            .into_dyn();
        let input_ids = ort::Value::from_array(None, &input_ids).unwrap();

        let attention_mask = ndarray::CowArray::from(&attention_mask)
            .into_shape((1, sequence_length))
            .map_err(|_| SemanticError::ShapeError)?
            .into_dyn();
        let attention_mask = ort::Value::from_array(None, &attention_mask).unwrap();

        let token_type_ids = ndarray::CowArray::from(&token_type_ids)
            .into_shape((1, sequence_length))
            .map_err(|_| SemanticError::ShapeError)?
            .into_dyn();
        let token_type_ids = ort::Value::from_array(None, &token_type_ids).unwrap();

        println!("token_type_ids: {:?}", token_type_ids);

        let outputs = self.session
            .run(ort::inputs![input_ids, attention_mask, token_type_ids].unwrap())
            .unwrap();

        let output_tensor = outputs[0].extract_tensor::<f32>().unwrap();
        let sequence_embedding = &*output_tensor.view();
        let pooled = sequence_embedding.mean_axis(Axis(1)).unwrap();

        Ok(Embedding(pooled.to_owned().as_slice().unwrap().to_vec()))
    }
}

type Result<T, E = SemanticError> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum SemanticError {
    TokenizeEncodeError,
    TokenizeEncodeByteError,
    ShapeError,
    InitSessionBuilder,
    InitSessionOptimization,
    InitBuildOrtEnv,
    InitSessionThreads,
    InitModelReadError,
    InitTokenizerReadError
}

impl Display for SemanticError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SemanticError::TokenizeEncodeError => write!(f, "TokenizeEncodeError"),
            SemanticError::ShapeError => write!(f, "ShapeError"),
            SemanticError::TokenizeEncodeByteError => write!(f, "TokenizeEncodeByteError"),
            SemanticError::InitSessionBuilder => write!(f, "InitSessionBuilder"),
            SemanticError::InitSessionOptimization => write!(f, "InitSessionOptimization"),
            SemanticError::InitSessionThreads => write!(f, "InitSessionThreads"),
            SemanticError::InitBuildOrtEnv => write!(f, "InitBuildOrtEnv"),
            SemanticError::InitModelReadError => write!(f, "InitModelReadError"),
            SemanticError::InitTokenizerReadError => write!(f, "InitTokenizerReadError"),
        }
    }
}