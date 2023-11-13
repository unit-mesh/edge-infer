pub mod embed;
pub mod memory_store;
pub mod document;
pub mod embedding;
pub mod embedding_store;
pub mod embedding_match;
pub mod similarity;
pub mod relevance_score;
pub mod cosine_similarity;

fn hello_name(name: String) -> String {
    format!("Hello {}", name)
}

uniffi::include_scaffolding!("inference");
