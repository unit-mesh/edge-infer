use clap::{Parser, arg};

/// Full line inference program arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct InferenceArgs {
    /// The port the server is listening on, if not specified the server will choose the port
    #[arg(short, long)]
    pub port: Option<String>,
}