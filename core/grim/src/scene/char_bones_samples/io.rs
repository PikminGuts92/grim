use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::scene::*;
use crate::SystemInfo;
use grim_traits::scene::*;
use thiserror::Error as ThisError;
use std::error::Error;

#[derive(Debug, ThisError)]
pub enum CharBonesSamplesReadError {
    #[error("CharBonesSamples version of {version} not supported")]
    CharBonesSamplesNotSupported {
        version: u32
    },
}

fn is_version_supported(version: u32) -> bool {
    match version {
        16 => true, // TBRB/GDRB
         _ => false
    }
}

pub(crate) fn load_char_bones_samples(char_bones_samples: &mut CharBonesSamples, reader: &mut Box<BinaryStream>, _info: &SystemInfo) -> Result<(), Box<dyn Error>> {
    let version = reader.read_uint32()?;

    // If not valid, return unsupported error
    if !is_version_supported(version) {
        return Err(Box::new(CharBonesSamplesReadError::CharBonesSamplesNotSupported {
            version
        }));
    }

    let (bones, sample_count) = load_char_bones_samples_header(char_bones_samples, reader, Some(version))?;
    load_char_bones_samples_data(char_bones_samples, reader, Some(version), bones, sample_count)?;

    Ok(())
}

pub(crate) fn load_char_bones_samples_header(char_bones_samples: &mut CharBonesSamples, reader: &mut Box<BinaryStream>, version: Option<u32>) -> Result<(Vec<(String, f32)>, u32), Box<dyn Error>> {
    let count_size = match version {
        Some(v) if v > 15 => 7,
        //_ => 10
        _ => todo!()
    };

    let bone_count = reader.read_uint32()?;
    let mut bones = Vec::new();

    for _ in 0..bone_count {
        let name = reader.read_prefixed_string()?;
        let weight = reader.read_float32()?;

        bones.push((name, weight));
    }

    for i in 0..count_size {
        char_bones_samples.counts[i] = reader.read_uint32()?;
    }

    char_bones_samples.compression = reader.read_uint32()?;
    let sample_count = reader.read_uint32()?;

    // Read frames
    char_bones_samples.frames = match version {
        Some(v) if v > 11 => {
            // Read frames
            let frame_count = reader.read_uint32()?;
            let mut frames = Vec::new();

            for _ in 0..frame_count {
                let frame = reader.read_float32()?;
                frames.push(frame);
            }

            frames
        },
        _ => Vec::new()
    };

    Ok((bones, sample_count))
}

pub(crate) fn load_char_bones_samples_data(char_bones_samples: &mut CharBonesSamples, reader: &mut Box<BinaryStream>, _version: Option<u32>, bones: Vec<(String, f32)>, sample_count: u32) -> Result<(), Box<dyn Error>> {
    /*if let Some(v) && v == 4 {

    }*/

    // Calculate sample frame size
    // TODO: Support 10 counts
    /*let mut size = 0;
    let mut curr_num = 0;

    for i in 0..char_bones_samples.counts.len() {
        let type_size = char_bones_samples.get_type_size(i as u32);
    }*/

    let sample_size: u32 = bones
        .iter()
        .filter_map(|(s, _)| match CharBonesSamples::get_type_of(s) {
            i @ 0..=6 => Some(char_bones_samples.get_type_size(i)),
            _ => None
        })
        .sum();

    // Read samples
    let mut samples = Vec::new();
    for _ in 0..sample_count {
        let data = reader.read_bytes(sample_size as usize)?;
        samples.push(data.into_boxed_slice());
    }

    char_bones_samples.samples = EncodedSamples::Compressed(
        bones.into_iter().map(|(s, w)| CharBone { symbol: s, weight: w }).collect(),
        samples
    );

    Ok(())
}