use crate::embedding::Embedding;
use crate::similarity::Similarity;

pub struct CosineSimilarity;

/// Cosine similarity is a measure of similarity between two non-zero vectors of an inner product space that measures the cosine of the angle between them.
/// The cosine of 0° is 1, and it is less than 1 for any other angle.
impl Similarity for CosineSimilarity {
    /// Calculates the cosine similarity between two vectors.
    fn similarity_score(&self, vector_a: &Embedding, vector_b: &Embedding) -> f32 {
        if vector_a.len() != vector_b.len() {
            panic!(
                "Length of vector a ({}) must be equal to the length of vector b ({})",
                vector_a.len(),
                vector_b.len()
            );
        }

        let dot_product: f32 = vector_a
            .iter()
            .zip(vector_b.iter())  // Use vector_b.iter() directly
            .map(|(a, b)| a * b)
            .sum();

        let norm_a: f32 = vector_a.iter().map(|x| x * x).sum();
        let norm_b: f32 = vector_b.iter().map(|x| x * x).sum();

        dot_product / (f32::sqrt(norm_a) * f32::sqrt(norm_b))
    }
}

impl CosineSimilarity {
    pub fn between(embedding: &Embedding, reference_embedding: &Embedding) -> f32 {
        CosineSimilarity.similarity_score(embedding, reference_embedding)
    }
}
