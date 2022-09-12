use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TBRBSongPreferences {
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