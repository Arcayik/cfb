//! CLI screen recorder for the linux framebuffer.
//!
//! cfb writes frame data to a file to be compiled later. This allows the program to grab frames as
//! quickly in succession as possible. Non-time-sensitive processes are simply done later, when the
//! recording gets compiled.
//!
//! # Examples
//!
//! Record from device /dev/fb0 (this device is default):
//! ```console
//! $ cfb record capture.cap -d /dev/fb0
//! ```
//! Compile capture file to mp4 video, output must be specified:
//! ```console
//! $ cfb compile capture.cap -o video.mp4 -f mp4
//! ```
//! Format Options: 
//! [raw](`crate::compile::OutputFormat::Raw`),
//! [png](`crate::compile::OutputFormat::Png`),
//! [mp4](`crate::compile::OutputFormat::Mp4`)

mod record;
mod compile;
mod cli;

use compile::OutputFormat;
use cli::{Cli, Commands}; 
use clap::Parser;

/// Uses [clap](`clap`) as implemented in module [cli](`crate::cli`)
/// to parse arguments to pass them to the relevant function in each module.
fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();
    dbg!(&args);
    match &args.command {
        Commands::Record(arg) => {
            record::capture(arg.device.as_str(), arg.file.as_str())?;
        },
        Commands::Compile(arg) => {
            compile::compile(arg.file.as_str(), &arg.format, &arg.output)?;
        }
    };

    Ok(())
}

