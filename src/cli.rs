//! Parsing command-line arguments.
use clap::{Args, Parser, Subcommand};

/// Arguments passed by the command line.
#[derive(Parser, Debug)]
#[command(name = "cfb")]
#[command(author = "Blakely H. <bhaug1@ocdsb.ca>")]
#[command(version = "1.0")]
#[command(propagate_version = true)]
#[command(about = "Framebuffer screen recorder written in Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// The choice between Recording and Compiling a video.
/// Each containing their corresponding arguments.
#[derive(Subcommand, Debug)]
pub enum Commands {
    Record(RecordArgs),
    Compile(CompileArgs),
}

/// Arguments that apply to [`Commands::Record`] only.
#[derive(Args, Debug)]
pub struct RecordArgs {
    #[arg()]
    pub file: String,

    #[arg(short, long)]
    #[arg(default_value = "/dev/fb0")]
    pub device: String,
}

/// Arguments that apply to [`Commands::Compile`] only.
#[derive(Args, Debug)]
pub struct CompileArgs {
    #[arg()]
    pub file: String,
    
    #[arg(short, long)]
    #[arg(default_value = "mp4")]
    #[arg(value_enum)]
    pub format: crate::OutputFormat,

    #[arg(short)]
    pub output: String,
}

