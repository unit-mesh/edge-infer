use inference_core::Semantic;


const MODEL: &[u8] = include_bytes!("../../../model/model.onnx");
const TOK: &[u8] = include_bytes!("../../../model/tokenizer.json");

#[tokio::main]
async fn main() {
    let semantic =
        Semantic::initialize(MODEL.to_vec(), TOK.to_vec())
            .await.unwrap();

    let encoding = semantic.embed("hello, world!").unwrap();

    println!("encoding: {:?}", encoding);
}
