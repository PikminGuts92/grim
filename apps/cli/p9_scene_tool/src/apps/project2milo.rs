use crate::apps::{SubApp};
use crate::models::*;
use clap::Parser;
use grim::io::*;
use grim::midi::{MidiFile, MidiTrack};
use grim::scene::{Object, PackedObject};
use log::{debug, error, info, warn};
use serde::Deserialize;
use serde_json::Deserializer;
use std::error::Error;
use std::fs::{copy, create_dir_all, File, read, remove_dir_all, write};
use std::io::Read;
use std::path::{Path, PathBuf};
use thiserror::Error;

// TODO: Rename to something like 'compile' or 'build'
#[derive(Parser, Debug)]
pub struct Project2MiloApp {
    #[clap(name = "dir_path", help = "Path to input project directory", required = true)]
    pub input_path: String,
    #[clap(name = "output_path", help = "Path to build output", required = true)]
    pub output_path: String,
    #[clap(short, long, help = "Enable to leave output milo archive(s) uncompressed", required = false)]
    pub uncompressed: bool
}

impl SubApp for Project2MiloApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let input_dir = PathBuf::from(&self.input_path);
        if !input_dir.exists() {
            // TODO: Throw proper error
            error!("Input directory {:?} doesn't exist", input_dir);
            return Ok(())
        }

        // Open song file
        let song_json_path = input_dir.join("song.json");
        let song_json = read(song_json_path)?;
        let song = serde_json::from_slice::<P9Song>(song_json.as_slice())?;

        //dbg!(&song);

        // Open midi
        let mid_path = input_dir.join("venue.mid");
        let _mid = MidiFile::from_path(mid_path).unwrap();

        // Get lipsync file(s)
        let _lipsyncs = get_lipsync(input_dir.join("lipsync").as_path(), song.preferences.is_gdrb());

        Ok(())
    }
}

fn get_lipsync(lipsync_dir: &Path, is_gdrb: bool) -> Vec<Object> {
    const GDRB_LIPSYNC_NAMES: [&str; 4] = [
        "song.lipsync",
        "billiejoe.lipsync",
        "mikedirnt.lipsync",
        "trecool.lipsync"
    ];

    const TBRB_LIPSYNC_NAMES: [&str; 4] = [
        "george.lipsync",
        "john.lipsync",
        "paul.lipsync",
        "ringo.lipsync"
    ];

    let lipsyncs = lipsync_dir
        .find_files_with_depth(FileSearchDepth::Immediate)
        .unwrap_or_default()
        .into_iter()
        .filter(|lip| lip
            .file_name()
            .and_then(|f| f.to_str())
            .map(|p| p.ends_with(".lipsync"))
            .unwrap_or_default())
        .collect::<Vec<_>>();

    if lipsyncs.is_empty() {
        warn!("No lipsync files found in {:?}", lipsync_dir);
        return Vec::new();
    }

    // Validate lipsync file names
    let lipsync_names = if is_gdrb { &GDRB_LIPSYNC_NAMES } else { &TBRB_LIPSYNC_NAMES };

    for lipsync_file in lipsyncs.iter() {
        let file_name = lipsync_file.file_name().and_then(|f| f.to_str()).unwrap();

        info!("Found \"{}\"", &file_name);

        let mut is_valid = false;

        for name in lipsync_names.iter() {
            if file_name.eq(*name) {
                is_valid = true;
                break;
            }
        }

        if !is_valid {
            warn!("Lipsync with file name \"{file_name}\" is invalid. Expected: {:?}", lipsync_names);
        }
    }

    // Get byte data for lipsync files
    lipsyncs
        .iter()
        .map(|lip_path| {
            let mut buffer = Vec::new();

            let mut file = File::open(lip_path).expect(format!("Can't open {:?}", lip_path).as_str());
            file.read_to_end(&mut buffer).expect(format!("Can't read data from {:?}", lip_path).as_str());

            let file_name = lip_path.file_name().and_then(|f| f.to_str()).unwrap();

            Object::Packed(PackedObject {
                name: file_name.to_string(),
                object_type: String::from("CharLipSync"),
                data: buffer,
            })
        })
        .collect()
}
