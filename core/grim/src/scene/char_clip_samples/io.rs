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

        if version < 13 {
            // Read headers first
            let (full_bones, full_sample_count) = load_char_bones_samples_header(&mut self.full, &mut reader, None)?;
            let (one_bones, one_sample_count) = load_char_bones_samples_header(&mut self.one, &mut reader, None)?;

            if version > 7 {
                // Read duplicate serialized data Probably milo bug
                // TODO: Write specific function that just skips data instead of read
                let mut cbs = CharBonesSamples::default();
                load_char_bones_samples_header(&mut cbs, &mut reader, None)?;
            }

            // Then read data
            load_char_bones_samples_data(&mut self.full, &mut reader, None, full_bones, full_sample_count)?;
            load_char_bones_samples_data(&mut self.one, &mut reader, None, one_bones, one_sample_count)?;
        } else {
            load_char_bones_samples(&mut self.full, &mut reader, info)?;
            load_char_bones_samples(&mut self.one, &mut reader, info)?;
        }

        if version > 14 {
            // Load bones
            let bone_count = reader.read_uint32()?;

            // TODO: Do something with extra bones values
            for _ in 0..bone_count {
                let _weight = reader.read_float32()?;
                let _name = reader.read_prefixed_string()?;
            }
        }

        Ok(())
    }

    fn save(&self, _stream: &mut dyn Stream, _info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}