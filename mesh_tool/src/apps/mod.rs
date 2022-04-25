use clap::{Parser, Subcommand};
use std::error::Error;

use grim::SystemInfo;

mod model2group;
pub use self::model2group::*;

// From Cargo.toml
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) trait SubApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>>;
}

#[derive(Parser, Debug)]
#[clap(name = PKG_NAME, version = VERSION, about = "Model importer for milo games")]
struct Options {
    #[clap(subcommand)]
    commands: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    #[clap(name = "model2group", about = "Convert model to milo group")]
    Model2Group(Model2GroupApp)
}

#[derive(Debug)]
pub struct MeshTool {
    options: Options,
}

impl MeshTool {
    pub fn new() -> MeshTool {
        MeshTool {
            options: Options::parse()
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        match &mut self.options.commands {
            SubCommand::Model2Group(app) => app.process()
        }
    }
}

pub trait GameOptions {
    fn get_system_info(&self) -> SystemInfo;
}