use std::collections::BinaryHeap;

use crate::embedding::Embedding;
use crate::similarity::{CosineSimilarity, EmbeddingMatch, RelevanceScore};

#[derive(Clone)]
struct Entry<Embedded: Clone + Ord> {
    id: String,
    embedding: Embedding,
    embedded: Option<Embedded>,
}

impl<Embedded: Clone + Ord> Entry<Embedded> {
    fn new(id: String, embedding: Embedding, embedded: Option<Embedded>) -> Self {
        Entry { id, embedding, embedded }
    }
}

pub struct InMemoryEmbeddingStore<Embedded: Clone + Ord> {
    entries: Vec<Entry<Embedded>>,
}

// Implement methods for InMemoryEmbeddingStore
impl<Embedded: Clone + Ord> InMemoryEmbeddingStore<Embedded> {
    fn new() -> Self {
        InMemoryEmbeddingStore { entries: Vec::new() }
    }

    fn add(&mut self, embedding: Embedding) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        self.add_with_id(id.clone(), embedding);
        id
    }

    fn add_with_id(&mut self, id: String, embedding: Embedding) {
        self.add_with_embedded(id, embedding, None);
    }

    fn add_with_embedded(&mut self, id: String, embedding: Embedding, embedded: Option<Embedded>) -> String {
        let entry = Entry::new(id.clone(), embedding, embedded);
        self.entries.push(entry);
        id
    }

    fn add_all(&mut self, embeddings: Vec<Embedding>) -> Vec<String> {
        embeddings
            .into_iter()
            .map(|embedding| self.add(embedding))
            .collect()
    }

    fn add_all_with_embedded(&mut self, embeddings: Vec<Embedding>, embedded: Vec<Embedded>) -> Vec<String> {
        assert_eq!(embeddings.len(), embedded.len(), "The list of embeddings and embedded must have the same size");

        embeddings
            .into_iter()
            .zip(embedded)
            .map(|(embedding, embedded)| self.add_with_embedded(uuid::Uuid::new_v4().to_string(), embedding, Some(embedded)))
            .collect()
    }

    fn find_relevant(&self, reference_embedding: Embedding, max_results: usize, min_score: f32) -> Vec<EmbeddingMatch<Embedded>> {
        let mut matches = BinaryHeap::new();

        for entry in &self.entries {
            let cosine_similarity = CosineSimilarity::between(&entry.embedding, &reference_embedding);
            let score = RelevanceScore::from_cosine_similarity(cosine_similarity);

            if score >= min_score {
                matches.push(EmbeddingMatch::new(score, entry.id.clone(), entry.embedding.clone(), entry.embedded.clone().unwrap()));

                if matches.len() > max_results {
                    matches.pop();
                }
            }
        }

        let mut result: Vec<_> = matches.into_sorted_vec();
        result.reverse();
        result
    }
}
