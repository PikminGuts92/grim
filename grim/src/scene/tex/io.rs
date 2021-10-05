use crate::{SystemInfo};
use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::scene::{ObjectReadWrite, save_object, Tex};
use crate::texture::Bitmap;
use thiserror::Error as ThisError;
use std::error::Error;

#[derive(Debug, ThisError)]
pub enum TexReadError {
    #[error("Tex version of {version} not supported")]
    TexVersionNotSupported {
        version: u32
    },
}

impl Tex {
    // TODO: Add from_hmx_image() function

    fn is_magic_valid(magic: u32, info: &SystemInfo) -> bool {
        match info.version {
            // GH1
            10 => match magic {
                8 => true,
                _ => false
            },
            // GH2
            24 => match magic {
                10 => true,
                _ => false
            },
            // GH2 360/TBRB/GDRB
            25 => match magic {
                10 => true,
                11 => true, // GDRB
                _ => false
            },
            _ => false
        }
    }

    pub fn from_stream(stream: &mut dyn Stream, info: &SystemInfo) -> Result<Tex, Box<dyn Error>> {
        let mut tex = Tex::new();
        tex.load(stream, info).and(Ok(tex))
    }
}

impl ObjectReadWrite for Tex {
    fn load(&mut self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        let mut reader = Box::new(BinaryStream::from_stream_with_endian(stream, info.endian));

        let magic = reader.read_uint32()?;

        // If not valid, return unsupported error
        if !Tex::is_magic_valid(magic, info) {
            return Err(Box::new(TexReadError::TexVersionNotSupported {
                version: magic
            }));
        }

        // Skip meta for now
        if magic >= 10 && info.version == 24 {
            reader.seek(SeekFrom::Current(9))?;
        } else if magic >= 10 {
            reader.seek(SeekFrom::Current(13))?;
        }

        if magic >= 11 {
            // TODO: Save boolean value
            reader.read_boolean()?;
        }

        self.width = reader.read_uint32()?;
        self.height = reader.read_uint32()?;
        self.bpp = reader.read_uint32()?;

        self.ext_path = reader.read_prefixed_string()?;
        self.index_f = reader.read_float32()?;
        self.index = reader.read_int32()?;

        self.use_ext_path = reader.read_boolean()?;

        if reader.pos() == reader.len()? as u64 {
            return Ok(());
        }

        self.bitmap = match Bitmap::from_stream(reader.as_mut(), info) {
            Ok(bitmap) => Some(bitmap),
            Err(_) => None,
        };

        Ok(())
    }

    fn save(&self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        let mut stream = Box::new(BinaryStream::from_stream_with_endian(stream, info.endian));

        // TODO: Get version from system info
        let version = 11;

        stream.write_uint32(version)?;

        save_object(self, &mut stream, info)?;

        if version >= 11 {
            // TODO: Write actual boolean value
            stream.write_boolean(false)?;
        }

        stream.write_uint32(self.width)?;
        stream.write_uint32(self.height)?;
        stream.write_uint32(self.bpp)?;

        stream.write_prefixed_string(&self.ext_path)?;
        stream.write_float32(self.index_f)?;
        stream.write_int32(self.index)?;

        stream.write_boolean(self.use_ext_path && self.bitmap.is_some())?;

        if let Some(bitmap) = &self.bitmap {
            bitmap.save(stream.as_mut(), info)?;
        }

        Ok(())
    }
}