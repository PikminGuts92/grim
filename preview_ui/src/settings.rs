use serde::{Deserialize, Serialize};
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

#[derive(Debug, Deserialize, Serialize)]
pub struct AppSettings {
    pub show_controls: bool,
    pub show_side_panel: bool,
    pub game_paths: Vec<GamePath>,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            show_controls: true,
            show_side_panel: true,
            game_paths: Vec::new(),
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
            return settings.unwrap_or_default();
        }

        AppSettings::default()
    }

    pub fn save_to_file<T>(&self, json_path: T) where T: AsRef<Path> {
        let json_path = json_path.as_ref();

        // TODO: Check if directory exists and check for error
        let json_text = serde_json::to_string_pretty(&self).unwrap();

        fs::write(json_path, json_text)
            .expect("Error writing settings to file");
    }
}