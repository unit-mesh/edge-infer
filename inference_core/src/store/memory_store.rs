use std::collections::BinaryHeap;
use crate::{Document, DocumentMatch};

use crate::embedding::Embedding;
use crate::similarity::{CosineSimilarity, RelevanceScore};

#[derive(Debug, Clone)]
pub struct Entry {
    id: String,
    embedding: Embedding,
    embedded: Option<Document>,
}

impl Entry {
    fn new(id: String, embedding: Embedding, embedded: Option<Document>) -> Self {
        Entry { id, embedding, embedded }
    }
}

pub struct InMemoryEmbeddingStore {
    entries: Vec<Entry>,
}

impl InMemoryEmbeddingStore {
    pub fn new() -> Self {
        InMemoryEmbeddingStore { entries: Vec::new() }
    }

    pub fn add(&mut self, embedding: Embedding) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        self.add_with_id(id.clone(), embedding);
        id
    }

    pub fn add_with_id(&mut self, id: String, embedding: Embedding) -> String {
        let entry = Entry::new(id.clone(), embedding, None);
        self.entries.push(entry);
        id
    }

    pub fn add_with_embedded(&mut self, id: String, embedding: Embedding, embedded: Document) -> String {
        let entry = Entry::new(id.clone(), embedding, Some(embedded));
        self.entries.push(entry);
        id
    }

    pub fn add_all(&mut self, embeddings: Vec<Embedding>) -> Vec<String> {
        embeddings
            .into_iter()
            .map(|embedding| self.add(embedding))
            .collect()
    }

    pub fn add_all_with_embedded(&mut self, embeddings: Vec<Embedding>, embedded: Vec<Document>) -> Vec<String> {
        assert_eq!(embeddings.len(), embedded.len(), "The list of embeddings and embedded must have the same size");

        embeddings
            .into_iter()
            .zip(embedded)
            .map(|(embedding, embedded)| self.add_with_embedded(uuid::Uuid::new_v4().to_string(), embedding, embedded))
            .collect()
    }

    pub fn find_relevant(&self, reference_embedding: Embedding, max_results: usize, min_score: f32) -> Vec<DocumentMatch> {
        let mut matches = BinaryHeap::new();

        for entry in &self.entries {
            let cosine_similarity = CosineSimilarity::between(&entry.embedding, &reference_embedding);
            let score = RelevanceScore::from_cosine_similarity(cosine_similarity);

            if score >= min_score {
                matches.push(DocumentMatch::new(score, entry.id.clone(), entry.embedding.clone(), entry.embedded.clone().unwrap()));

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
