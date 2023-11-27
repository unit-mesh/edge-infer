use std::io::{Cursor, Read};

use async_stream::stream;
use tokenizer::tokenizer_client::TokenizerClient;
use tokenizer::EncodeRequest;
use tokenizer::EncodeReply;

pub mod tokenizer {
    tonic::include_proto!("tokenizer");
}

const model: &[u8] = include_bytes!("../../../model/model.onnx");
const tok: &[u8] = include_bytes!("../../../model/tokenizer.json");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TokenizerClient::connect("http://[::1]:50051").await?;


    let mut cursor = Cursor::new(tok);
    let mut buf = [0u8; 1024 * 8];

    let response = client.set_tokenizer_json(stream! {
        while let Ok(n) = cursor.read(&mut buf) {
            if n == 0 {
                break;
            }

            yield tokenizer::TokenizerJson {
                json: buf[..n].to_vec(),
            };
        }
    }).await?;

    println!("tokenizer RESPONSE={:?}", response);

    let mut cursor = Cursor::new(model);
    let response = client.set_model(stream! {
        while let Ok(n) = cursor.read(&mut buf) {
            if n == 0 {
                break;
            }

            yield tokenizer::Model {
                model: buf[..n].to_vec(),
            };
        }
    }).await?;
    println!("model RESPONSE={:?}", response);

    let response =  client.init_model(()).await ;

    let request = tonic::Request::new(EncodeRequest {
        text: "Tonic".into(),
    });

    let response = client.encode(request).await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}
