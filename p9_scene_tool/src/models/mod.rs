mod gdrb;
mod tbrb;

pub use gdrb::*;
pub use tbrb::*;
use serde::{Deserialize, Serialize, Serializer, ser::{SerializeStruct, SerializeSeq}};
use serde::de::{value::StringDeserializer, Visitor};

#[derive(Default)]
pub struct P9Song {
    pub name: String,
    pub preferences: SongPreferences,
    pub lyric_configurations: Option<Vec<LyricConfig>>,
}

pub enum SongPreferences {
    TBRB(TBRBSongPreferences),
    GDRB(GDRBSongPreferences)
}

impl Default for SongPreferences {
    fn default() -> Self {
        SongPreferences::TBRB(TBRBSongPreferences::default())
    }
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

impl<'de> Deserialize<'de> for P9Song {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        todo!()
    }
}

impl Serialize for P9Song {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut song = serializer.serialize_struct("P9Song", 1)?;
        let game_format = match self.preferences {
            SongPreferences::TBRB(_) => "TBRB",
            SongPreferences::GDRB(_) => "GDRB"
        };

        song.serialize_field("name", &self.name)?;
        song.serialize_field("format", game_format)?;

        match &self.preferences {
            SongPreferences::TBRB(tbrb_prefs) => {
                song.serialize_field("preferences", tbrb_prefs)?;
            },
            SongPreferences::GDRB(gdrb_prefs) => {
                song.serialize_field("preferences", gdrb_prefs)?;
            }
        }

        if let Some(lyric_configs) = &self.lyric_configurations {
            song.serialize_field("lyric_configs", lyric_configs)?;
        }

        song.end()
    }
}
