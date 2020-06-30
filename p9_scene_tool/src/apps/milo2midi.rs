use crate::apps::{SubApp};
use clap::{App, Arg, Clap};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

use grim::{Platform, SystemInfo};
use grim::io::*;
use grim::scene::{Object, ObjectDir, PackedObject, Tex};
use grim::texture::{Bitmap, write_rgba_to_file};

#[derive(Clap, Debug)]
pub struct Milo2MidiApp {
    #[clap(about = "Path to input milo scene", required = true)]
    pub milo_path: String,
    #[clap(about = "Path to output MIDI file", required = true)]
    pub midi_path: String,
    #[clap(long, about = "Base MIDI file")]
    pub base_midi: String
}

impl SubApp for Milo2MidiApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {

        Ok(())
    }
}