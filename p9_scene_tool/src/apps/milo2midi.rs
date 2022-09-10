use crate::apps::{SubApp};
use clap::Parser;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

use grim::{Platform, SystemInfo};
use grim::io::*;
use grim::midi::MidiFile;
use grim::scene::{Object, ObjectDir, ObjectDirBase, PackedObject, Tex};
use grim::texture::{Bitmap, write_rgba_to_file};

#[derive(Parser, Debug)]
pub struct Milo2MidiApp {
    #[clap(help = "Path to input milo scene", required = true)]
    pub milo_path: String,
    #[clap(help = "Path to output MIDI file", required = true)]
    pub midi_path: String,
    #[clap(short = 'm', long, help = "Base MIDI file")]
    pub base_midi: Option<String>
}

impl SubApp for Milo2MidiApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let milo_path = PathBuf::from(&self.milo_path);
        let output_midi_path  = PathBuf::from(&self.midi_path);

        let mut mid = self.base_midi
            .as_ref()
            .and_then(|path| MidiFile::from_path(path))
            .unwrap_or_default();

        // TODO: Remove debug output
        for track in mid.tracks.iter() {
            let track_name = track.name
                .as_ref()
                .map(|n| n.as_str())
                .unwrap_or("???");

            let ev_count = track.events.len();

            println!("\"{track_name}\" : {ev_count} events");
        }

        // Open milo
        let mut stream: Box<dyn Stream> = Box::new(FileStream::from_path_as_read_open(&milo_path)?);
        let mut milo = MiloArchive::from_stream(&mut stream)?;

        // Unpack dir and entries
        let system_info = SystemInfo::guess_system_info(&milo, &milo_path);
        let mut obj_dir = milo.unpack_directory(&system_info)?;
        obj_dir.unpack_entries(&system_info)?;

        for entry in obj_dir.get_entries() {
            let name = entry.get_name();
            let obj_type = entry.get_type();

            let is_packed = entry.is_packed();

            println!("{name} | {obj_type} (packed: {is_packed})");
        }

        Ok(())
    }
}