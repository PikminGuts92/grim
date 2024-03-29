mod decode;
#[cfg(feature = "encode")] mod encode;

use clap::{Parser, Subcommand};
use decode::*;
#[cfg(feature = "encode")] use encode::*;
use std::error::Error;

// From Cargo.toml
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) trait SubApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>>;
}

#[derive(Parser, Debug)]
#[command(name = PKG_NAME, version = VERSION, about = "Audio tool for milo games")]
struct Options {
    #[command(subcommand)]
    commands: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    #[command(name = "decode", about = "Decode audio file")]
    Decode(DecoderApp),
    #[cfg(feature = "encode")]
    #[command(name = "encode", about = "Encode audio file")]
    Encode(EncoderApp)
}

#[derive(Debug)]
pub struct AudioTool {
    options: Options,
}

impl AudioTool {
    pub fn new() -> AudioTool {
        AudioTool {
            options: Options::parse()
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        match &mut self.options.commands {
            SubCommand::Decode(app) => app.process(),
            #[cfg(feature = "encode")] SubCommand::Encode(app) => app.process()
        }
    }
}