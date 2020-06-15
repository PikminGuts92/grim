use crate::{Platform, SystemInfo};
use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::scene::Tex;
use crate::texture::Bitmap;
use std::error::Error;

impl Tex {
    // TODO: Add from_hmx_image() function

    fn is_magic_valid(magic: i32, info: &SystemInfo) -> bool {
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
            // GH2 360
            25 => match magic {
                10 => true,
                _ => false
            },
            _ => false
        }
    }

    pub fn from_stream(stream: &mut dyn Stream, info: &SystemInfo) -> Result<Tex, Box<dyn Error>> {
        let mut tex = Tex::new();
        let mut reader = BinaryStream::from_stream_with_endian(stream, info.endian);

        let magic = reader.read_int32()?;
        let is_magic_valid = Tex::is_magic_valid(magic, info);
        // TODO: If not valid, return unsupported error

        // Skip meta for now
        if magic >= 10 && info.version == 24 {
            reader.seek(SeekFrom::Current(9))?;
        } else if magic >= 10 {
            reader.seek(SeekFrom::Current(13))?;
        }

        tex.width = reader.read_uint32()?;
        tex.height = reader.read_uint32()?;
        tex.bpp = reader.read_uint32()?;

        tex.ext_path = reader.read_prefixed_string()?;
        tex.index_f = reader.read_float32()?;
        tex.index = reader.read_int32()?;

        tex.use_ext_path = reader.read_boolean()?;

        if reader.pos() == reader.len()? as u64 {
            return Ok(tex);
        }

        tex.bitmap = match Bitmap::from_stream(&mut reader, info) {
            Ok(bitmap) => Some(bitmap),
            Err(_) => None,
        };

        Ok(tex)
    }
}