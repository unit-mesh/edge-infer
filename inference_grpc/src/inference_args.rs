use clap::{Parser, arg};

/// Full line inference program arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct InferenceArgs {
    /// The port the server is listening on, if not specified the server will choose the port
    #[arg(short, long)]
    port: Option<String>,

    /// Path to the model file [required]
    #[arg(short, long, required = true)]
    model: String,

    /// Model type, available values: [onnx, llama] [required]
    #[arg(short, long, required = true)]
    model_type: String,
}