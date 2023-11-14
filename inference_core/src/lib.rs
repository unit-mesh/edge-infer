pub mod document;
pub mod embedding;
pub mod similarity;
pub mod store;

use std::sync::Arc;
// make the following types available in the generated bindings
pub use document::Document;
pub use document::Metadata;
pub use embedding::Embedding;
pub use embedding::Semantic;
pub use embedding::semantic::SemanticError;
pub use embedding::semantic::init_semantic;

pub use similarity::EmbeddingMatch;
pub use similarity::CosineSimilarity;
pub use similarity::Similarity;
pub use similarity::RelevanceScore;

pub use store::EmbeddingStore;
pub use store::InMemoryEmbeddingStore;

pub fn get_cosine_similarity() -> Arc<dyn Similarity> {
    Arc::new(CosineSimilarity {})
}

uniffi::include_scaffolding!("inference");
