use std::cmp::Ordering;
use crate::{Document, Embedding};

#[derive(Debug, Clone)]
pub struct DocumentMatch {
    pub score: f32,
    pub embedding_id: String,
    pub embedding: Embedding,
    pub embedded: Document,
}

impl DocumentMatch {
    pub fn new(score: f32, embedding_id: String, embedding: Embedding, embedded: Document) -> Self {
        DocumentMatch {
            score,
            embedding_id,
            embedding,
            embedded,
        }
    }
}

impl PartialOrd for DocumentMatch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl PartialEq for DocumentMatch {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Ord for DocumentMatch {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.partial_cmp(&other.score).unwrap_or(Ordering::Equal)
    }
}

impl Eq for DocumentMatch {
}
