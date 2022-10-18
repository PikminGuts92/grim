use clap::{Parser, Subcommand};
use std::error::Error;

mod milo2midi;
mod newproject;
mod project2milo;
use self::milo2midi::*;
use self::newproject::*;
use self::project2milo::*;

// From Cargo.toml
pub const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
// pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const VERSION: &'static str = get_prerelease_version();

const fn get_prerelease_version() -> &'static str {
    const BUILD_DATE: &str = build_time::build_time_local!("%Y%m%d");
    const_format::formatcp!("prealpha-{}", BUILD_DATE)
}

pub(crate) trait SubApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>>;
}

#[derive(Parser, Debug)]
#[command(name = PKG_NAME, version = VERSION, about = "Use this tool for modding scenes from milo engine based games (project 9)")]
struct Options {
    #[command(subcommand)]
    commands: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    #[command(name = "milo2midi", about = "Creates MIDI from milo scene")]
    Milo2Midi(Milo2MidiApp),
    #[command(name = "newproj", about = "Create new song project from scratch")]
    NewProject(NewProjectApp),
    #[command(name = "proj2milo", about = "Build song milo archive(s) from input project")]
    Project2Milo(Project2MiloApp)
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
            SubCommand::NewProject(app) => app.process(),
            SubCommand::Project2Milo(app) => app.process()
        }
    }
}