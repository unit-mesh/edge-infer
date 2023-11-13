use crate::embedding::Embedding;

pub trait Similarity {
    fn similarity_score(&self, set1: &Embedding, set2: &Embedding) -> f32;
}
