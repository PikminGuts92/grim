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

pub(crate) fn load_char_bones_samples(char_bones_samples: &mut CharBonesSamples, reader: &mut Box<BinaryStream>, info: &SystemInfo, read_meta: bool) -> Result<(), Box<dyn Error>> {
    let version = reader.read_uint32()?;

    // If not valid, return unsupported error
    if !is_version_supported(version) {
        return Err(Box::new(CharBonesSamplesReadError::CharBonesSamplesNotSupported {
            version
        }));
    }

    Ok(())
}