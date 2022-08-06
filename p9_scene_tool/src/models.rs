use serde::{Deserialize, Serialize, Serializer, ser::SerializeSeq};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct P9Song {
    pub name: String,
    pub preferences: SongPreferences,
    #[serde(rename = "lyricConfigs")] pub lyric_configurations: Vec<LyricConfig>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SongPreferences {
    pub venue: String,
    pub mini_venues: Vec<String>,
    pub scenes: Vec<String>,

    pub dreamscape_outfit: String,
    pub studio_outfit: String,

    pub george_instruments: Vec<String>,
    pub john_instruments: Vec<String>,
    pub paul_instruments: Vec<String>,
    pub ringo_instruments: Vec<String>,

    pub tempo: String,
    pub song_clips: String,
    pub dreamscape_font: String,

    pub george_amp: String,
    pub john_amp: String,
    pub paul_amp: String,
    pub mixer: String,
    pub dreamscape_camera: String,

    pub lyric_part: String,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LyricConfig {
    pub name: String,
    pub lyrics: Vec<LyricEvent>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LyricEvent {
    #[serde(rename = "pos")] pub position: [f32; 3],
    #[serde(rename = "rot")] pub rotation: [f32; 4],
    pub scale: [f32; 3],
}