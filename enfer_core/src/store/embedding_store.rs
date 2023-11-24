use crate::embedding::Embedding;
use crate::similarity::EmbeddingMatch;

pub trait EmbeddingStore<Embedded: Clone + Ord> {
    // Adds an embedding to the store and returns its unique identifier.
    fn add(&mut self, embedding: Embedding) -> String;

    // Adds an embedding to the store with a specified identifier.
    fn add_with_id(&mut self, id: String, embedding: Embedding);

    // Adds an embedding to the store and associates it with the provided embedded data.
    fn add_with_embedded(&mut self, embedding: Embedding, embedded: Embedded) -> String;

    // Adds a list of embeddings to the store and returns a list of unique identifiers.
    fn add_all(&mut self, embeddings: Vec<Embedding>) -> Vec<String>;

    // Adds a list of embeddings to the store and associates them with a list of embedded data.
    fn add_all_with_embedded(&mut self, embeddings: Vec<Embedding>, embedded: Vec<Embedded>) -> Vec<String>;

    // Find relevant embeddings in the store based on a reference embedding, with a maximum number of results.
    // An optional minimum score can be specified to filter results.
    fn find_relevant(
        &self,
        reference_embedding: Embedding,
        max_results: usize,
        min_score: f32,
    ) -> Vec<EmbeddingMatch<Embedded>>;
}
