use crate::apps::{SubApp};
use clap::Parser;
use grim::dta::DataArray;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

use grim::{Platform, SystemInfo};
use grim::io::*;
use grim::midi::{MidiEvent, MidiTextType, MidiFile, MidiText, MidiTrack};
use grim::scene::{Object, ObjectDir, ObjectDirBase, PackedObject, PropAnim, PropKeysEvents, Tex, AnimRate};
use grim::texture::{Bitmap, write_rgba_to_file};

#[derive(Parser, Debug)]
pub struct Milo2MidiApp {
    #[arg(help = "Path to input milo scene", required = true)]
    pub milo_path: String,
    #[arg(help = "Path to output MIDI file", required = true)]
    pub midi_path: String,
    #[arg(short = 'm', long, help = "Base MIDI file")]
    pub base_midi: Option<String>
}

impl SubApp for Milo2MidiApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let milo_path = PathBuf::from(&self.milo_path);
        let output_midi_path  = PathBuf::from(&self.midi_path);

        let mut mid = self.base_midi
            .as_ref()
            .and_then(|path| MidiFile::from_path(path))
            .unwrap_or_default();

        // TODO: Remove debug output
        for track in mid.tracks.iter() {
            let track_name = track.name
                .as_ref()
                .map(|n| n.as_str())
                .unwrap_or("???");

            let ev_count = track.events.len();

            println!("\"{track_name}\" : {ev_count} events");
        }

        // Open milo
        let mut stream = FileStream::from_path_as_read_open(&milo_path)?;
        let milo = MiloArchive::from_stream(&mut stream)?;

        // Unpack dir and entries
        let system_info = SystemInfo::guess_system_info(&milo, &milo_path);
        let mut obj_dir = milo.unpack_directory(&system_info)?;
        obj_dir.unpack_entries(&system_info)?;

        for entry in obj_dir.get_entries() {
            let name = entry.get_name();
            let obj_type = entry.get_type();

            let is_packed = entry.is_packed();

            println!("{name} | {obj_type} (packed: {is_packed})");

            if let Object::PropAnim(prop_anim) = entry {
                let extra_tracks = process_prop_anim(prop_anim, &mid);
                mid.add_tracks_with_realtime_positions(extra_tracks, false);
            }
        }

        // Save output midi file
        let midi_dir = output_midi_path.parent().unwrap();
        if !midi_dir.exists() {
            // Not found, create directory
            fs::create_dir_all(&midi_dir)?;
        }

        mid.write_to_file(output_midi_path);

        Ok(())
    }
}

fn process_prop_anim(prop_anim: &PropAnim, _base_mid: &MidiFile) -> Vec<MidiTrack> {
    // TODO: Pre-parse tempo track for faster realtime to tick pos calculation

    const GDRB_CHARACTERS: [(&str, &str); 3] = [
        ("BILLIE", "BILLIEJOE"),
        ("MIKE", "MIKEDIRNT"),
        ("TRE", "TRECOOL"),
    ];

    // Create tracks
    let mut mapped_tracks = GDRB_CHARACTERS
        .iter()
        .map(|(c_short, c_long)| (format!("_{}", c_long.to_lowercase()), MidiTrack {
            name: Some(c_short.to_string()),
            events: Vec::new()
        }))
        .collect::<HashMap<_, _>>();

    let mut venue_track = MidiTrack {
        name: Some(String::from("VENUE GDRB")),
        events: Vec::new()
    };

    let fps = match prop_anim.rate {
        AnimRate::k30_fps | AnimRate::k30_fps_ui | AnimRate::k30_fps_tutorial => 30.,
        _ => panic!("Unsupported anim rate of {:?}", prop_anim.rate)
    };

    let track_keys = mapped_tracks.keys().map(|k| k.to_string()).collect::<Vec<_>>();

    for prop_keys in prop_anim.keys.iter() {
        let _target = prop_keys.target.as_str(); // Don't care for now

        // Assume single symbol for now (most common for TBRB/GDRB song anims)
        let property = prop_keys
            .property
            .first()
            .and_then(|node| match node {
                DataArray::Symbol(s) => s.as_utf8(),
                _ => None,
            });

        if property.is_none() {
            continue;
        }

        let mut property = (unsafe { property.unwrap_unchecked() }).to_string();
        let mut track = &mut venue_track; // Use venue track by default

        for track_key in track_keys.iter() {
            if property.contains(track_key) {
                // Update property name and use dedicated character track
                property = property.replace(track_key, "");
                track = unsafe { mapped_tracks.get_mut(track_key).unwrap_unchecked() };

                break;
            }
        }

        // Map events to display vecs
        let events_as_display: Vec<(f32, Vec<&dyn Display>)> = match &prop_keys.events {
            PropKeysEvents::Float(events) => events
                .iter()
                .map(|ev| (ev.pos, vec![
                    &ev.value as &dyn Display
                ]))
                .collect(),
            PropKeysEvents::Color(events) => events
                .iter()
                .map(|ev| (ev.pos, vec![
                    &ev.value.r as &dyn Display,
                    &ev.value.g as &dyn Display,
                    &ev.value.b as &dyn Display,
                    &ev.value.a as &dyn Display
                ]))
                .collect(),
            PropKeysEvents::Object(events) => events
                .iter()
                .map(|ev| (ev.pos, vec![
                    &ev.text1 as &dyn Display,
                    &ev.text2 as &dyn Display
                ]))
                .collect(),
            PropKeysEvents::Bool(events) => events
                .iter()
                .map(|ev| (ev.pos, vec![
                    if ev.value { &"TRUE" } else { &"FALSE" } as &dyn Display
                ]))
                .collect(),
            PropKeysEvents::Quat(events) => events
                .iter()
                .map(|ev| (ev.pos, vec![
                    &ev.value.x as &dyn Display,
                    &ev.value.y as &dyn Display,
                    &ev.value.z as &dyn Display,
                    &ev.value.w as &dyn Display
                ]))
                .collect(),
            PropKeysEvents::Vector3(events) => events
                .iter()
                .map(|ev| (ev.pos, vec![
                    &ev.value.x as &dyn Display,
                    &ev.value.y as &dyn Display,
                    &ev.value.z as &dyn Display
                ]))
                .collect(),
            PropKeysEvents::Symbol(events) => events
                .iter()
                .map(|ev| (ev.pos, vec![
                    &ev.text as &dyn Display
                ]))
                .collect()
        };

        for (pos, values) in events_as_display {
            let realtime_pos = (pos as f64 / fps) * 1000.; // Convert from frame pos to realtime (ms)

            // Joins values into single string
            // TODO: Look at making this more efficient
            let values_formatted = values
                .iter()
                .map(|v| v.to_string())
                .filter(|v| !v.is_empty())
                .collect::<Vec<_>>()
                .join(" ");

            let text = format!("[{property} ({values_formatted})]");

            // Add event
            track.events.push(MidiEvent::Meta(MidiText {
                pos: 0, // Calculated elsewhere
                pos_realtime: Some(realtime_pos),
                text: MidiTextType::Event(text.into_bytes().into_boxed_slice())
            }))
        }
    }

    let mut new_tracks = Vec::new();
    for (_, char_long) in GDRB_CHARACTERS.iter() {
        let key = format!("_{}", char_long.to_lowercase());

        let track = mapped_tracks.remove(key.as_str()).unwrap();
        new_tracks.push(track);
    }

    new_tracks.push(venue_track);
    new_tracks
}