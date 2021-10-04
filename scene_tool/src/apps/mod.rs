use clap::{Clap};
use std::error::Error;

use grim::SystemInfo;

mod dir2milo;
mod milo2dir;
mod savemilo;
pub use self::dir2milo::*;
pub use self::milo2dir::*;
pub use self::savemilo::*;

// From Cargo.toml
const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) trait SubApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>>;
}

#[derive(Clap, Debug)]
#[clap(name = PKG_NAME, version = VERSION, about = "Use this tool for modding scenes from milo engine based games")]
struct Options {
    #[clap(subcommand)]
    commands: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(name = "dir2milo", about = "Creates milo scene from input directory")]
    Dir2Milo(Dir2MiloApp),
    #[clap(name = "milo2dir", about = "Extracts content of milo scene to directory")]
    Milo2Dir(Milo2DirApp),
    #[clap(name = "savemilo", about = "Save milo")]
    SaveMilo(SaveMiloApp),
}

#[derive(Debug)]
pub struct SceneTool {
    options: Options,
}

impl SceneTool {
    pub fn new() -> SceneTool {
        SceneTool {
            options: Options::parse()
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        match &mut self.options.commands {
            SubCommand::Dir2Milo(app) => app.process(),
            SubCommand::Milo2Dir(app) => app.process(),
            SubCommand::SaveMilo(app) => app.process(),
        }
    }
}

pub trait GameOptions {
    fn get_system_info(&self) -> SystemInfo;
}