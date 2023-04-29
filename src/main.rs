mod record;
mod compile;
use compile::OutputFormat;

mod cli;
use crate::cli::{Cli, Commands}; 
use clap::Parser;

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();
    dbg!(&args);
    match &args.command {
        Commands::Record(arg) => {
            record::capture(arg.device.as_str(), arg.file.as_str())?;
        },
        Commands::Compile(arg) => {
            compile::compile(arg.file.as_str(), &arg.format)?;
        }
    };

    Ok(())
}


