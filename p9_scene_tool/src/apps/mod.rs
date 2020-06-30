use clap::{App, Arg, Clap};
use std::error::Error;

mod milo2midi;
pub use self::milo2midi::*;

// From Cargo.toml
const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub(crate) trait SubApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>>;
}

#[derive(Clap, Debug)]
#[clap(name = PKG_NAME, version = VERSION, about = "Use this tool for modding scenes from milo engine based games (project 9)")]
struct Options {
    #[clap(subcommand)]
    commands: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(name = "milo2midi", about = "Creates MIDI from milo scene")]
    Milo2Midi(Milo2MidiApp)
}

#[derive(Debug)]
pub struct P9SceneTool {
    options: Options,
}

impl P9SceneTool {
    pub fn new() -> P9SceneTool {
        P9SceneTool {
            options: Options::parse()
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        match &mut self.options.commands {
            SubCommand::Milo2Midi(app) => app.process(),
        }
    }
}