pub mod embed;
pub mod memory_store;
pub mod document;
pub mod embedding;
pub mod embedding_store;
mod embedding_match;
mod similarity;
mod relevance_score;
mod cosine_similarity;

fn hello_name(name: String) -> String {
    format!("Hello {}", name)
}

uniffi::include_scaffolding!("inference");
