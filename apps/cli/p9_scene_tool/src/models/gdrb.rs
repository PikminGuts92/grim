use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct GDRBSongPreferences {
    pub venue: String,

    pub mike_instruments: Vec<String>,   // George
    pub billie_instruments: Vec<String>, // John
    pub tre_instruments: Vec<String>,    // Ringo

    pub tempo: String,
    pub song_clips: String,

    // New for GDRB
    pub normal_outfit: String,
    pub bonus_outfit: String,
    pub drum_set: String,

    pub era: String,

    // TODO: Investigate if wanted/needed
    #[serde(skip_serializing)] pub cam_directory: String,
    #[serde(skip_serializing)] pub media_directory: String,

    pub song_intro_cam: String,
    pub win_cam: String,
}