use std::collections::HashMap;
use crate::embedding::Embedding;

struct Document {
    id: String,
    metadata: Metadata,
    text: String,
    vector: Embedding,
}

struct Metadata {
    metadata: HashMap<String, String>,
}

impl Document {
    fn from(string_value: String) -> Self {
        Self {
            id: "".to_string(),
            metadata: Metadata::new(),
            text: string_value,
            vector: vec![],
        }
    }

    fn from_with_metadata(string_value: String, metadata: Metadata) -> Self {
        Self {
            id: "".to_string(),
            metadata,
            text: string_value,
            vector: vec![],
        }
    }
}