use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::scene::*;
use crate::SystemInfo;
use grim_traits::scene::*;
use thiserror::Error as ThisError;
use std::error::Error;

#[derive(Debug, ThisError)]
pub enum CharClipSamplesReadError {
    #[error("CharClipSamples version of {version} not supported")]
    CharClipSamplesNotSupported {
        version: u32
    },
}

fn is_version_supported(version: u32) -> bool {
    match version {
        16 => true, // TBRB/GDRB
         _ => false
    }
}

impl ObjectReadWrite for CharClipSamples {
    fn load(&mut self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        let mut reader = Box::new(BinaryStream::from_stream_with_endian(stream, info.endian));

        let version = reader.read_uint32()?;

        // If not valid, return unsupported error
        if !is_version_supported(version) {
            return Err(Box::new(CharClipSamplesReadError::CharClipSamplesNotSupported {
                version
            }));
        }

        // Metadata is written for CharClip instead for some reason
        load_char_clip(self, &mut reader, info, true)?;

        if version >= 16 {
            self.some_bool = reader.read_boolean()?;
        }

        Ok(())
    }

    fn save(&self, _stream: &mut dyn Stream, _info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}