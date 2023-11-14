use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
use crate::{Document, DocumentMatch};

use crate::embedding::Embedding;
use crate::similarity::{CosineSimilarity, RelevanceScore};

#[derive(Debug, Clone)]
pub struct Entry {
    id: String,
    embedding: Embedding,
    embedded: Document,
}

impl Entry {
    fn new(id: String, embedding: Embedding, embedded: Document) -> Self {
        Entry { id, embedding, embedded }
    }
}

#[derive(Debug, Clone)]
pub struct InMemoryEmbeddingStore {
    entries: Arc<Mutex<Vec<Entry>>>,
}

impl InMemoryEmbeddingStore {
    pub fn new() -> Self {
        InMemoryEmbeddingStore { entries: Arc::new(Mutex::new(vec![])) }
    }

    pub fn add(&self, id: String, embedding: Embedding, document: Document) -> String {
        let entry = Entry::new(id.clone(), embedding, document);
        self.entries.lock().unwrap().push(entry);
        id
    }

    pub fn add_all(&self, embeddings: Vec<Embedding>, embedded: Vec<Document>) -> Vec<String> {
        assert_eq!(embeddings.len(), embedded.len(), "The list of embeddings and embedded must have the same size");

        embeddings
            .into_iter()
            .zip(embedded)
            .map(|(embedding, embedded)| self.add(uuid::Uuid::new_v4().to_string(), embedding, embedded))
            .collect()
    }

    pub fn find_relevant(&self, reference_embedding: Embedding, max_results: usize, min_score: f32) -> Vec<DocumentMatch> {
        let mut matches = BinaryHeap::new();

        for entry in self.entries.lock().unwrap().iter() {
            let cosine_similarity = CosineSimilarity::between(&entry.embedding, &reference_embedding);
            let score = RelevanceScore::from_cosine_similarity(cosine_similarity);

            if score >= min_score {
                matches.push(DocumentMatch::new(score, entry.id.clone(), entry.embedding.clone(), entry.embedded.clone()));

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
