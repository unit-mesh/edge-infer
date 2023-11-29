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
    /// Creates a new document from a string value.
    pub fn from(string_value: String) -> Self {
        Self {
            id: "".to_string(),
            metadata: Metadata::new(),
            text: string_value,
            vector: Embedding(vec![]),
        }
    }

    /// Creates a new document from a string value and metadata.
    /// Example:
    /// ```rust
    /// use inference_core::{Document, Metadata};
    ///
    /// let mut metadata = Metadata::new();
    /// metadata.metadata.insert("title".to_string(), "Hello world!".to_string());
    /// let document = Document::from_with_metadata("Hello world!".to_string(), metadata);
    /// assert_eq!(document.metadata.metadata.get("title").unwrap(), "Hello world!");
    /// ```
    pub fn from_with_metadata(string_value: String, metadata: Metadata) -> Self {
        Self {
            id: "".to_string(),
            metadata,
            text: string_value,
            vector: Embedding(vec![]),
        }
    }
}