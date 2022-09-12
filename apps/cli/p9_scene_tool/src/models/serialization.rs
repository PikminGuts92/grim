use serde::{Deserialize, Serialize, Serializer, ser::{SerializeStruct, SerializeSeq}};
use serde::de::{self, MapAccess, value::StringDeserializer, Visitor};
use std::fmt;
use super::*;

struct P9SongVisitor;

impl<'de> Visitor<'de> for P9SongVisitor {
    type Value = P9Song;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct P9Song")
    }

    fn visit_map<V: MapAccess<'de>>(self, mut map: V) -> Result<P9Song, V::Error> {
        let mut name: Option<String> = None;
        let mut format: Option<String> = None;
        let mut preferences: Option<SongPreferences> = None;
        let mut lyric_configs: Option<Vec<LyricConfig>> = None;

        while let Some(key) = map.next_key()? {
            match key {
                "Name" | "name" => name = map.next_value()?,
                "Format" | "format" => format = map.next_value()?,
                "Preferences" | "preferences" => {
                    // TODO: How to handle if "format" is deserialized after preferences?
                    let prefs = match format.as_deref() {
                        Some("GDRB") => map
                            .next_value::<GDRBSongPreferences>()
                            .map(|gdrb_prefs| SongPreferences::GDRB(gdrb_prefs))?,
                        _ => map.next_value::<TBRBSongPreferences>()
                            .map(|tbrb_prefs| SongPreferences::TBRB(tbrb_prefs))?,
                    };

                    preferences = Some(prefs);
                },
                "LyricConfigs" | "lyric_configs" => lyric_configs = map.next_value()?,
                _ => continue,
            }
        }

        Ok(P9Song {
            name: name.unwrap_or_default(),
            preferences: preferences.unwrap_or_default(),
            lyric_configurations: lyric_configs,
            ..Default::default()
        })
    }
}

impl<'de> Deserialize<'de> for P9Song {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        const FIELDS: [&str; 8] = [
            "Name", "name",
            "Format", "format",
            "Preferences", "preferences",
            "LyricConfigs", "lyric_configs"
        ];

        deserializer.deserialize_struct("P9Song", &FIELDS, P9SongVisitor)
    }
}

impl Serialize for P9Song {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut song = serializer.serialize_struct("P9Song", 4)?;
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
