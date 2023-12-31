mod cosine_similarity;
mod relevance_score;
mod embedding_match;
mod document_match;

pub use cosine_similarity::CosineSimilarity;
pub use relevance_score::RelevanceScore;
pub use embedding_match::EmbeddingMatch;
pub use document_match::DocumentMatch;

use crate::embedding::Embedding;

pub trait Similarity: Send + Sync {
    fn similarity_score(&self, set1: &Embedding, set2: &Embedding) -> f32;
}

