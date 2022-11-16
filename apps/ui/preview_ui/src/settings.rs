use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::fs::{read_to_string, self};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct GamePath {
    pub path: String,
    pub game: Game,
    pub platform: Platform,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Game {
    Unknown,
    TBRB,
    GDRB
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Platform {
    X360,
    PS3,
    Wii
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize, Resource, Serialize)]
pub struct AppSettings {
    pub show_controls: bool,
    pub game_paths: Vec<GamePath>,
    pub window_width: f32,
    pub window_height: f32,
    pub maximized: bool,
    pub show_gridlines: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            show_controls: true,
            game_paths: Vec::new(),
            window_width: 1280.0,
            window_height: 720.0,
            maximized: false,
            show_gridlines: true,
        }
    }
}

impl AppSettings {
    pub fn load_from_file<T>(json_path: T) -> Self where T: AsRef<Path> {
        let json_path = json_path.as_ref();

        if !json_path.exists() {
            return AppSettings::default();
        }

        let json_text = read_to_string(json_path);

        if let Ok(text) = json_text {
            let settings = serde_json::from_str::<AppSettings>(&text);

            if let Err(err) = &settings {
                println!("Unable to parse settings: {:?}", err);
            }

            return settings.unwrap_or_default();
        }

        AppSettings::default()
    }

    pub fn save_to_file<T>(&self, json_path: T) where T: AsRef<Path> {
        #[cfg(not(target_family = "wasm"))] {
            let json_path = json_path.as_ref();

            // TODO: Check if directory exists and check for error
            let json_text = serde_json::to_string_pretty(&self).unwrap();

            fs::write(json_path, json_text)
                .expect("Error writing settings to file");
        }
    }
}