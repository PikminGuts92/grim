mod gdrb;
mod tbrb;
mod serialization;

pub use gdrb::*;
pub use tbrb::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct P9Song {
    pub name: String,
    pub preferences: SongPreferences,
    pub lyric_configurations: Option<Vec<LyricConfig>>,
}

#[derive(Debug)]
pub enum SongPreferences {
    TBRB(TBRBSongPreferences),
    GDRB(GDRBSongPreferences)
}

impl Default for SongPreferences {
    fn default() -> Self {
        SongPreferences::TBRB(TBRBSongPreferences::default())
    }
}

impl SongPreferences {
    pub fn is_tbrb(&self) -> bool {
        match self {
            SongPreferences::TBRB(_) => true,
            _ => false,
        }
    }

    pub fn is_gdrb(&self) -> bool {
        match self {
            SongPreferences::GDRB(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LyricConfig {
    #[serde(alias = "Name")] pub name: String,
    #[serde(alias = "Lyrics")] pub lyrics: Vec<LyricEvent>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LyricEvent {
    #[serde(alias = "Pos", rename = "pos")] pub position: [f32; 3],
    #[serde(alias = "Rot", rename = "rot")] pub rotation: [f32; 4],
    #[serde(alias = "Scale")] pub scale: [f32; 3],
}