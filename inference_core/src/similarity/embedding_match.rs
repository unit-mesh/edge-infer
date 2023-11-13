use std::cmp::Ordering;
use crate::embedding::Embedding;

#[derive(Debug, Clone)]
pub struct EmbeddingMatch<Embedded: Clone + Ord> {
    score: f32,
    embedding_id: String,
    embedding: Embedding,
    embedded: Embedded,
}

impl<Embedded: Clone + Ord> EmbeddingMatch<Embedded> {
    pub(crate) fn new(score: f32, embedding_id: String, embedding: Embedding, embedded: Embedded) -> Self {
        EmbeddingMatch {
            score,
            embedding_id,
            embedding,
            embedded,
        }
    }
}

impl<Embedded: Clone + Ord> PartialOrd for EmbeddingMatch<Embedded> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl<Embedded: Clone + Ord> PartialEq for EmbeddingMatch<Embedded> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl<Embedded: Clone + Ord> Ord for EmbeddingMatch<Embedded> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.partial_cmp(&other.score).unwrap_or(Ordering::Equal)
    }
}

impl<Embedded: Clone + Ord> Eq for EmbeddingMatch<Embedded> {
}