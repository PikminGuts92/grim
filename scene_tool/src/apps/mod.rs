use clap::{App, Arg, Clap};
use std::error::Error;

mod dir2milo;
mod milo2dir;
pub use self::milo2dir::*;
pub use self::dir2milo::*;

// From Cargo.toml
const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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
            SubCommand::Milo2Dir(app) => app.process()
        }
    }
}