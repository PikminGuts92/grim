use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::scene::*;
use crate::SystemInfo;
use grim_traits::scene::*;
use thiserror::Error as ThisError;
use std::error::Error;

#[derive(Debug, ThisError)]
pub enum CharClipReadError {
    #[error("CharClip version of {version} not supported")]
    CharClipVersionNotSupported {
        version: u32
    },
}

fn is_version_supported(version: u32) -> bool {
    match version {
        12 => true, // TBRB/GDRB
         _ => false
    }
}

pub(crate) fn load_char_clip<T: CharClip>(char_clip: &mut T, reader: &mut Box<BinaryStream>, info: &SystemInfo, read_meta: bool) -> Result<(), Box<dyn Error>> {
    let version = reader.read_uint32()?;

    // If not valid, return unsupported error
    if !is_version_supported(version) {
        // TODO: Switch to custom error
        return Err(Box::new(CharClipReadError::CharClipVersionNotSupported {
            version
        }));
    }

    if read_meta {
        load_object(char_clip, reader, info)?;
    }

    char_clip.set_start_beat(reader.read_float32()?);
    char_clip.set_end_beat(reader.read_float32()?);
    char_clip.set_beats_per_sec(reader.read_float32()?);

    char_clip.set_flags(reader.read_uint32()?);
    char_clip.set_play_flags(reader.read_uint32()?);

    char_clip.set_blend_width(reader.read_float32()?);

    if version > 3 {
        char_clip.set_range(reader.read_float32()?);
    }

    if version == 5 {
        todo!()
    } else if version > 5 {
        char_clip.set_relative(reader.read_prefixed_string()?);
    }

    /*if (version - 9) < 2 {
        todo!()
    }*/

    if version > 9 {
        char_clip.set_unknown_1(reader.read_int32()?);
    }

    if version > 11 {
        char_clip.set_do_not_decompress(reader.read_boolean()?);
    }

    let node_count = if version < 8 {
        reader.read_uint32()?
    } else {
        reader.seek(SeekFrom::Current(4))?; // Skip node size
        reader.read_uint32()?
    };

    // Read nodes
    let mut nodes = Vec::new();

    for _ in 0..node_count {
        let name = reader.read_prefixed_string()?;
        let value_count = reader.read_uint32()?;

        let mut values = Vec::new();

        for _ in 0..value_count {
            let frame = reader.read_float32()?;
            let weight = reader.read_float32()?;

            values.push(ClipNodeData {
                frame,
                weight
            });
        }

        nodes.push(ClipNode {
            name,
            values
        });
    }

    char_clip.set_nodes(nodes);

    if version < 3 {
        todo!()
    }

    if version < 7 {
        todo!()
    } else {
        // Read events
        let event_count = reader.read_uint32()?;
        let mut events = Vec::new();

        for _ in 0..event_count {
            let frame = reader.read_float32()?;
            let script = reader.read_prefixed_string()?;

            events.push(FrameEvent {
                frame,
                script
            });
        }

        char_clip.set_events(events);
    }

    Ok(())
}