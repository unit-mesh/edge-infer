use std::sync::Arc;

pub use document::Document;
pub use document::Metadata;
pub use embedding::Embedding;
pub use embedding::Semantic;
pub use embedding::semantic::SemanticError;
pub use similarity::CosineSimilarity;
pub use similarity::DocumentMatch;
pub use similarity::EmbeddingMatch;
pub use similarity::RelevanceScore;
pub use similarity::Similarity;
pub use store::EmbeddingStore;
pub use store::InMemoryEmbeddingStore;

pub mod document;
pub mod embedding;
pub mod similarity;
pub mod store;

pub fn get_cosine_similarity() -> Arc<dyn Similarity> {
    Arc::new(CosineSimilarity {})
}

pub fn init_semantic(model: Vec<u8>, tokenizer_data: Vec<u8>) -> Result<Arc<Semantic>, SemanticError> {
    let result = Semantic::init_semantic(model, tokenizer_data)?;
    Ok(Arc::new(result))
}

pub fn init_semantic_with_path(model_path: &str, tokenizer_path: &str) -> Result<Arc<Semantic>, SemanticError> {
    let model = std::fs::read(model_path).map_err(|_| SemanticError::InitModelReadError)?;
    let tokenizer_data = std::fs::read(tokenizer_path).map_err(|_| SemanticError::InitTokenizerReadError)?;

    let result = Semantic::init_semantic(model, tokenizer_data)?;
    Ok(Arc::new(result))
}

uniffi::include_scaffolding!("inference");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_semantic() {
        let model = std::fs::read("../model/model.onnx").unwrap();
        let tokenizer_data = std::fs::read("../model/tokenizer.json").unwrap();

        let semantic = init_semantic(model, tokenizer_data).unwrap();
        let embedding = semantic.embed("hello world").unwrap();
        assert_eq!(embedding.len(), 128);
    }

    #[test]
    fn should_find_relevant() {
        let model = std::fs::read("../model/model.onnx").unwrap();
        let tokenizer_data = std::fs::read("../model/tokenizer.json").unwrap();

        let semantic = init_semantic(model, tokenizer_data).unwrap();
        let hello = "hello world";
        let pure_text_hello = semantic.embed(hello).unwrap();
        let code_hello_text = "print('hello world')";
        let code_hello = semantic.embed(code_hello_text).unwrap();

        let embedding_store = InMemoryEmbeddingStore::new();
        embedding_store.add(hello.to_string(), pure_text_hello.clone(), Document::from(hello.to_string()));
        embedding_store.add(code_hello_text.to_string(), code_hello.clone(), Document::from(code_hello_text.to_string()));

        let vec = embedding_store.find_relevant(pure_text_hello, 1, 0.0);
        assert_eq!(vec.len(), 1);
    }
}