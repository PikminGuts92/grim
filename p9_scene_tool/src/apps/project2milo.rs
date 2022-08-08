use crate::apps::{SubApp};
use crate::models::*;
use clap::Parser;
use grim::midi::{MidiFile, MidiTrack};
use serde::Deserialize;
use serde_json::Deserializer;
use std::error::Error;
use std::fs::{copy, create_dir_all, File, read, remove_dir_all, write};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Parser, Debug)]
pub struct Project2MiloApp {
    #[clap(name = "dirPath", help = "Path to input project directory", required = true)]
    pub input_path: String,
    #[clap(name = "miloPath", help = "Path to output milo archive", required = true)]
    pub output_path: String,
    #[clap(short, long, help = "Enable to leave output milo archive(s) uncompressed", required = false)]
    pub uncompressed: bool
}

impl SubApp for Project2MiloApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let input_dir = PathBuf::from(&self.input_path);
        if !input_dir.exists() {
            // TODO: Throw proper error
            panic!("Input directory doesn't exist")
        }

        let song_json_path = input_dir.join("song.json");
        let song_json = read(song_json_path)?;
        let song = serde_json::from_slice::<P9Song>(song_json.as_slice())?;

        dbg!(song);

        Ok(())
    }
}
