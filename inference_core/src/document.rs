use std::collections::HashMap;

use crate::embedding::Embedding;

#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub metadata: Metadata,
    pub text: String,
    pub vector: Embedding,
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub metadata: HashMap<String, String>,
}

impl Metadata {
    pub fn new() -> Self {
        Self {
            metadata: HashMap::new(),
        }
    }
}


impl Document {
    pub fn from(string_value: String) -> Self {
        Self {
            id: "".to_string(),
            metadata: Metadata::new(),
            text: string_value,
            vector: Embedding(vec![]),
        }
    }

    pub fn from_with_metadata(string_value: String, metadata: Metadata) -> Self {
        Self {
            id: "".to_string(),
            metadata,
            text: string_value,
            vector: Embedding(vec![]),
        }
    }
}