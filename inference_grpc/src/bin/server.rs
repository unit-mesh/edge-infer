use std::path::Path;
use tonic::{transport::Server, Request, Response, Status};

use tokenizer::tokenizer_server::{Tokenizer, TokenizerServer};
use tokenizer::{EncodeRequest, EncodeReply};

use inference_core::embed::Semantic;

pub mod tokenizer {
    tonic::include_proto!("tokenizer");
}

#[derive(Default)]
pub struct MyTokenizer {}

#[tonic::async_trait]
impl Tokenizer for MyTokenizer {
    async fn encode(
        &self,
        request: Request<EncodeRequest>,
    ) -> Result<Response<EncodeReply>, Status> {
        println!("Got a request from {:?}", request.remote_addr());

        let reply = tokenizer::EncodeReply {
            text: format!("Hello {}!", request.into_inner().text),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyTokenizer::default();

    println!("GreeterServer listening on {}", addr);

    // Semantic::initialize(&Path::new(env!("CARGO_MANIFEST_DIR")).join("model")).await.unwrap();

    Server::builder()
        .add_service(TokenizerServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
