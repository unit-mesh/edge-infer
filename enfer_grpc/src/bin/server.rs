use std::pin::Pin;
use tokio::sync::Mutex;
use clap::Parser;
use tonic::{transport::Server, Request, Response, Status, Streaming};
use tonic::codegen::tokio_stream::StreamExt;

use tokenizer::tokenizer_server::{Tokenizer, TokenizerServer};
use tokenizer::{EncodeRequest, EncodeReply, GeneralResponse, Model, TokenizerJson};

use inference_core::embedding::Semantic;
use inference_grpc::inference_args::InferenceArgs;

pub mod tokenizer {
    tonic::include_proto!("tokenizer");
}

#[derive(Default)]
pub struct MyTokenizer {
    sema: Mutex<Option<Pin<Box<Semantic>>>>,

    tokenzier: Mutex<Vec<u8>>,
    model: Mutex<Vec<u8>>,
}

#[tonic::async_trait]
impl Tokenizer for MyTokenizer {
    async fn set_tokenizer_json(&self, reqeust: Request<Streaming<TokenizerJson>>) -> Result<Response<GeneralResponse>, Status> {
        let mut t = self.tokenzier.lock().await;
        t.clear();

        let mut stream = reqeust.into_inner();
        while let Some(json) = stream.next().await {
            let json = match json {
                Ok(j) => j,

                Err(e) => return Ok(Response::new(GeneralResponse{
                    success: false,
                    error: format!("json error: {}", e).into(),
                })),
            };
            t.extend(json.json);
        }


        Ok(Response::new(GeneralResponse{
            success: true,
            error: None,
        }))
    }

    async fn set_model(&self, reqeust: Request<Streaming<Model>>) -> Result<Response<GeneralResponse>, Status> {
        let mut t = self.model.lock().await;
        t.clear();

        let mut stream = reqeust.into_inner();
        while let Some(model) = stream.next().await {
            let model = match model {
                Ok(j) => j,

                Err(e) => return Ok(Response::new(GeneralResponse{
                    success: false,
                    error: format!("model error: {}", e).into(),
                })),
            };
            t.extend(model.model);
        }


        Ok(Response::new(GeneralResponse{
            success: true,
            error: None,
        }))

    }

    async fn init_model(&self, _: tonic::Request<()>) -> Result<Response<GeneralResponse>, Status> {
        let model = self.model.lock().await;
        let tokenizer = self.tokenzier.lock().await;


        let sema = match Semantic::initialize(model.clone(), tokenizer.clone()).await {
            Ok(t) => t,
            Err(e) => return Ok(Response::new(GeneralResponse{
                success: false,
                error: format!("sma init failed: {}", e).into(),
            })),
        };

        {
            let mut s = self.sema.lock().await;
            s.replace(sema);
        }

        Ok(Response::new(GeneralResponse{
            success: true,
            error: None,
        }))

    }

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
    let args = InferenceArgs::parse();
    let addr = format!("[::1]:{}", args.port.unwrap_or_else(|| "50051".to_string())).parse().unwrap();
    let greeter = MyTokenizer::default();

    println!("GreeterServer listening on {}", addr);


    Server::builder()
        .add_service(TokenizerServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
