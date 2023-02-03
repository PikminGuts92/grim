use crate::apps::{SubApp};
use crate::models::*;
use clap::Parser;
use grim::midi::{MidiFile, MidiTrack};
use std::error::Error;
use std::fs::{copy, create_dir_all, read, remove_dir_all, write};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Parser, Debug)]
pub struct NewProjectApp {
    #[arg(help = "Path to output project directory", required = true)]
    pub output_path: String,
    #[arg(short, long, help = "Shortname of song (ex. \"temporarysec\")", required = true)]
    pub name: String,
    #[arg(short, long, help = "Use GDRB format", required = false)]
    pub gdrb: bool
}

impl SubApp for NewProjectApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let ouput_dir = PathBuf::from(&self.output_path);
        if !ouput_dir.exists() {
            // Create dir
            create_dir_all(&ouput_dir).unwrap();
        }

        // Create extra folder
        let extra_dir = ouput_dir.join("extra");
        if !extra_dir.exists() {
            create_dir_all(&extra_dir).unwrap();
            //write(extra_dir.join("EXTRA_MILO_RELATED_FILES_HERE"), "").unwrap();
            println!("Created extras directory");
        }

        // Create lipsync folder
        let lipsync_dir = ouput_dir.join("lipsync");
        if !lipsync_dir.exists() {
            create_dir_all(&lipsync_dir).unwrap();
            //write(lipsync_dir.join("LIPSYNC_HERE"), "").unwrap();
            println!("Created lipsync directory");
        }

        // Write midi file
        let midi_path = ouput_dir.join("venue.mid");
        create_default_mid(&midi_path, self.gdrb)?;

        // Write json file
        let song = create_p9_song(&self.name, self.gdrb);
        //let song_json = serde_json::to_string_pretty(&song)?;
        let song_json = crate::formatter::to_string(&song)?;
        let song_json_path = ouput_dir.join("song.json");

        write(song_json_path, song_json).unwrap();
        println!("Wrote \"song.json\"");

        let output_dir_str = ouput_dir.as_path().to_str().unwrap_or("???"); // Ugh why so hacky?
        let game_format = if self.gdrb { "GDRB" } else { "TBRB" };

        println!("Successfully created project for {game_format} in \"{output_dir_str}\"");
        Ok(())
    }
}

fn create_p9_song(name: &str, gdrb: bool) -> P9Song {
    let preferences = if gdrb {
        SongPreferences::GDRB(GDRBSongPreferences {
            venue: String::from("dookie"),
            mike_instruments: vec![
                String::from("bass_g3")
            ],
            billie_instruments: vec![
                String::from("guitar_blue01")
            ],
            tre_instruments: vec![
                String::from("drum_dw")
            ],
            tempo: String::from("medium"),
            song_clips: String::from("none"),
            normal_outfit: String::from("dookie"),
            bonus_outfit: String::from("dookie"),
            drum_set: String::from("drum_dw"),
            era: String::from("early"),
            cam_directory: String::from(""),
            media_directory: String::from(""),
            song_intro_cam: String::from(""),
            win_cam: String::from(""),
        })
    } else {
        SongPreferences::TBRB(TBRBSongPreferences {
            venue: String::from("dreamscape"),
            mini_venues: vec![
                String::from("abbeyroad01default")
            ],
            scenes: Vec::new(),
            dreamscape_outfit: String::from("sixtyeight"),
            studio_outfit: String::from("sixtyeight_hdp"),
            george_instruments: vec![
                String::from("guitar_rh_gibson_lespaul_red")
            ],
            john_instruments: vec![
                String::from("guitar_rh_epi65casino_strip")
            ],
            paul_instruments: vec![
                String::from("bass_lh_ricken_4001s_stripped")
            ],
            ringo_instruments: vec![
                String::from("drum_dream04")
            ],
            tempo: String::from("medium"),
            song_clips: String::from("none"),
            dreamscape_font: String::from("none"),
            george_amp: String::from("none"),
            john_amp: String::from("none"),
            paul_amp: String::from("none"),
            mixer: String::from("none"),
            dreamscape_camera: String::from("kP9DreamSlow"),
            lyric_part: String::from("PART HARM1")
        })
    };

    P9Song {
        name: name.to_owned(),
        preferences,
        ..P9Song::default()
    }
}

fn create_default_mid(mid_path: &Path, gdrb: bool) -> Result<(), std::io::Error> {
    const TBRB_TRACK_NAMES: [&str; 5] = [
        "PAUL",
        "JOHN",
        "GEORGE",
        "RINGO",
        "VENUE"
    ];

    const GDRB_TRACK_NAMES: [&str; 4] = [
        "BILLIE",
        "MIKE",
        "TRE",
        "VENUE GDRB"
    ];

    let mut midi = MidiFile::default();

    // Create basic tempo track
    // Nothing to do?

    // Add other tracks
    let track_names = if gdrb { GDRB_TRACK_NAMES.as_slice() } else { TBRB_TRACK_NAMES.as_slice() };
    for track_name in track_names {
        midi.tracks.push(MidiTrack {
            name: Some(track_name.to_string()),
            events: Vec::new()
        });
    }

    midi.write_to_file(mid_path);
    println!("Wrote \"venue.mid\"");
    Ok(())
}