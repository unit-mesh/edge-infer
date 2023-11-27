mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    // fn init_semantic(model: Vec<u8>, tokenizer_data: Vec<u8>) -> Result<Arc<Semantic>, SemanticError>;
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, browser-embedding-poc!");
}
