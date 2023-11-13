pub struct RelevanceScore;

impl RelevanceScore {
    pub fn from_cosine_similarity(cosine_similarity: f32) -> f32 {
        (cosine_similarity + 1.0) / 2.0
    }
}
