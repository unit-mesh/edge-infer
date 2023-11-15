use std::sync::Arc;

pub use document::Document;
pub use document::Metadata;
pub use embedding::Embedding;
pub use embedding::Semantic;
pub use embedding::semantic::SemanticError;
pub use similarity::CosineSimilarity;
pub use similarity::DocumentMatch;
pub use similarity::EmbeddingMatch;
pub use similarity::RelevanceScore;
pub use similarity::Similarity;
pub use store::EmbeddingStore;
pub use store::InMemoryEmbeddingStore;

pub mod document;
pub mod embedding;
pub mod similarity;
pub mod store;

pub fn get_cosine_similarity() -> Arc<dyn Similarity> {
    Arc::new(CosineSimilarity {})
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

uniffi::include_scaffolding!("inference");
