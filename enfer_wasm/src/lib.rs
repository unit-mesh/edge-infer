mod utils;

use std::sync::Arc;
use wasm_bindgen::prelude::*;
use inference_core::{Semantic, SemanticError};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    // fn init_semantic(model: Vec<u8>, tokenizer_data: Vec<u8>) -> Result<Arc<Semantic>, SemanticError>;
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, browser-embedding-poc!");
}
