use crate::apps::{SubApp};
use crate::models::*;
use clap::Parser;
use serde::Serialize;
use std::error::Error;
use std::fs::{copy, create_dir_all, read, remove_dir_all, write};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Parser, Debug)]
pub struct NewProjectApp {
    #[clap(help = "Path to output project directory", required = true)]
    pub dir_path: String,
    #[clap(short, long, help = "Shortname of song (ex. \"temporarysec\")", required = true)]
    pub name: String
}

impl SubApp for NewProjectApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let ouput_dir = PathBuf::from(&self.dir_path);
        if !ouput_dir.exists() {
            // Create dir
            create_dir_all(&ouput_dir).unwrap();
        }

        //let song_meta_path = ouput_dir.join("song.json");

        let song = create_p9_song(&self.name);
        //let song_json = serde_json::to_string_pretty(&song)?;
        let song_json = crate::formatter::to_string(&song)?;

        let song_json_path = ouput_dir.join("song.json");

        //serde_json::ser::PrettyFormatter::new()

        write(song_json_path, song_json).unwrap();
        println!("Wrote \"song.json\"");

        let output_dir_str = ouput_dir.as_path().to_str().unwrap(); // Ugh why so hacky?

        println!("Successfully created project in \"{output_dir_str}\"");
        Ok(())
    }
}

fn create_p9_song(name: &str) -> P9Song {
    P9Song {
        name: name.to_owned(),
        preferences: SongPreferences {
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
        },
        lyric_configurations: vec![
            LyricConfig {
                name: String::from("config_1"),
                lyrics: vec![
                    LyricEvent {
                        position: [0., 0., 0.],
                        rotation: [0., 0., 0., 0.],
                        scale: [1., 1., 1.]
                    }
                ]
            }
        ]
    }
}