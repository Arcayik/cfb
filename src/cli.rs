use clap::{Args, Parser, Subcommand};

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

#[derive(Subcommand, Debug)]
pub enum Commands {
    Record(RecordArgs),
    Compile(CompileArgs),
}

#[derive(Args, Debug)]
pub struct RecordArgs {
    #[arg(default_value = "recording.cap")]
    pub file: String,

    #[arg(short, long)]
    #[arg(default_value = "/dev/fb0")]
    pub device: String,
}

#[derive(Args, Debug)]
pub struct CompileArgs {
    #[arg()]
    pub file: String,
    #[arg(short, long)]
    #[arg(default_value = "mp4")]
    #[arg(value_enum)]
    pub format: crate::OutputFormat,
}

/*
impl CompileArgs {
    // Method to convert entered string into concrete enum of choices
    pub fn get_format_enum(&self) -> OutputFormat {
        match self.format.as_str() {
            "raw" => OutputFormat::Raw,
            "png" => OutputFormat::Png,
            "mp4" => OutputFormat::Mp4,
            _ => panic!("fail"),
        }
    }
}
*/
