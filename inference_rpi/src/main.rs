use rust_gpiozero::*;

use inference_core::embed::Semantic;

const model: &[u8] = include_bytes!("../../model/model.onnx");
const tok: &[u8] = include_bytes!("../../model/tokenizer.json");

fn main() {
    Semantic::initialize(model.to_vec(), tok.to_vec()).unwrap();
    // Create a new LED attached to Pin 17
    let mut led = LED::new(17);
    led.blink(1.0, 1.0);
    led.wait();
}
