use inference_core::embed::Semantic;

const model: &[u8] = include_bytes!("../../model/model.onnx");
const tok: &[u8] = include_bytes!("../../model/tokenizer.json");

fn main() {
    Semantic::initialize(model.to_vec(), tok.to_vec()).unwrap();
}
