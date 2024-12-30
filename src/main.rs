use clap::Parser;
use ollama_rs::{
    Ollama,
    generation::completion::request::GenerationRequest,
};
use core::error::Error;
use tokio_stream::StreamExt;
use std::{io::{stdout, Write}, path::PathBuf};

#[derive(Copy, Clone, clap::ValueEnum)]
enum Mode {
    Pixel,
    Ratio,
}

#[derive(clap::Parser)]
enum Command {
    /// Generate an image.
    Generate {
        /// A file path where the output image will be saved.
        #[arg(short, long, default_value = "llama3.2-vision")]
        model: String,
        prompt: String,
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    /// Refines an existing image.
    Refine { file: PathBuf, prompt: String },
    Extend {
        file: PathBuf,
        mode: Mode,
        left: f64,
        right: f64,
        up: f64,
        down: f64,
    },
    Zoom {
        file: PathBuf,
        mode: Mode,
        x: f64,
        y: f64,
        r#in: f64,
        out: f64,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    match Command::parse() {
        Command::Generate { model, prompt, .. } => {
            let mut out = stdout();
            let mut stream = Ollama::new("http://ollama", 11434)
                .generate_stream(GenerationRequest::new(
                    model,
                    prompt,
                ))
                .await?;
            
            while let Some(Ok(chunk)) = stream.next().await {
                for response in chunk {
                    out.write_all(response.response.as_bytes())?;
                }
                out.flush()?;
            }
        }
        Command::Refine { .. } => println!("refine yaooo!"),
        Command::Extend { .. } => println!("extend yaooo!"),
        Command::Zoom { .. } => println!("zoom yaooo!"),
    }
    Ok(())
}