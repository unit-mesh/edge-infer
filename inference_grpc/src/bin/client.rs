use tokenizer::tokenizer_client::TokenizerClient;
use tokenizer::EncodeRequest;

pub mod tokenizer {
    tonic::include_proto!("tokenizer");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TokenizerClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(EncodeRequest {
        text: "Tonic".into(),
    });

    let response = client.encode(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
