use serde::{Deserialize, Serialize, Serializer, ser::SerializeSeq};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct P9Song {
    #[serde(alias = "Name")] pub name: String,
    #[serde(alias = "Game")] pub game: P9Game,
    #[serde(alias = "Preferences")] pub preferences: SongPreferences,
    #[serde(alias = "LyricConfigs", rename = "lyric_configs", skip_serializing_if = "Option::is_none")] pub lyric_configurations: Option<Vec<LyricConfig>>,
}

#[derive(Serialize, Deserialize)]
pub enum P9Game {
    TBRB,
    GDRB
}

impl Default for P9Game {
    fn default() -> Self {
        P9Game::TBRB
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct SongPreferences {
    #[serde(alias = "Venue")] pub venue: String,
    #[serde(alias = "MiniVenues")] pub mini_venues: Vec<String>,
    #[serde(alias = "Scenes")] pub scenes: Vec<String>,

    #[serde(alias = "DreamscapeOutfit")] pub dreamscape_outfit: String,
    #[serde(alias = "StudioOutfit")] pub studio_outfit: String,

    #[serde(alias = "GeorgeInstruments")] pub george_instruments: Vec<String>,
    #[serde(alias = "JohnInstruments")] pub john_instruments: Vec<String>,
    #[serde(alias = "PaulInstruments")] pub paul_instruments: Vec<String>,
    #[serde(alias = "RingoInstruments")] pub ringo_instruments: Vec<String>,

    #[serde(alias = "Tempo")] pub tempo: String,
    #[serde(alias = "SongClips")] pub song_clips: String,
    #[serde(alias = "DreamscapeFont")] pub dreamscape_font: String,

    #[serde(alias = "GeorgeAmp")] pub george_amp: String,
    #[serde(alias = "JohnAmp")] pub john_amp: String,
    #[serde(alias = "PaulAmp")] pub paul_amp: String,
    #[serde(alias = "Mixer")] pub mixer: String,
    #[serde(alias = "DreamscapeCamera")] pub dreamscape_camera: String,

    #[serde(alias = "LyricPart")] pub lyric_part: String,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LyricConfig {
    #[serde(alias = "Name")] pub name: String,
    #[serde(alias = "Lyrics")] pub lyrics: Vec<LyricEvent>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct LyricEvent {
    #[serde(alias = "Pos", rename = "pos")] pub position: [f32; 3],
    #[serde(alias = "Rot", rename = "rot")] pub rotation: [f32; 4],
    #[serde(alias = "Scale")] pub scale: [f32; 3],
}