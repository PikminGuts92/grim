use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::scene::*;
use crate::SystemInfo;
use crate::texture::Bitmap;
use grim_traits::scene::*;
use std::error::Error;

fn is_version_supported(version: u32) -> bool {
    match version {
        1 => true, // GH2 360
        _ => false
    }
}

impl ObjectReadWrite for CubeTexObject {
    fn load(&mut self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        let mut stream = Box::new(BinaryStream::from_stream_with_endian(stream, info.endian));

        let version = stream.read_uint32()?;
        if !is_version_supported(version) {
            // TODO: Switch to custom error
            panic!("CubeTex version \"{}\" is not supported!", version);
        }

        load_object(self, &mut stream, info)?;

        self.some_num_1 = stream.read_uint32()?;
        self.some_num_2 = stream.read_uint32()?;

        self.right_ext_path = stream.read_prefixed_string()?;
        self.left_ext_path = stream.read_prefixed_string()?;
        self.top_ext_path = stream.read_prefixed_string()?;
        self.bottom_ext_path = stream.read_prefixed_string()?;
        self.front_ext_path = stream.read_prefixed_string()?;
        self.back_ext_path = stream.read_prefixed_string()?;

        self.some_bool = stream.read_boolean()?;

        if stream.pos() == stream.len()? as u64 {
            return Ok(());
        }
        self.right = Bitmap::from_stream(stream.as_mut(), info).ok();

        if stream.pos() == stream.len()? as u64 {
            return Ok(());
        }
        self.left = Bitmap::from_stream(stream.as_mut(), info).ok();

        if stream.pos() == stream.len()? as u64 {
            return Ok(());
        }
        self.top = Bitmap::from_stream(stream.as_mut(), info).ok();

        if stream.pos() == stream.len()? as u64 {
            return Ok(());
        }
        self.bottom = Bitmap::from_stream(stream.as_mut(), info).ok();

        if stream.pos() == stream.len()? as u64 {
            return Ok(());
        }
        self.front = Bitmap::from_stream(stream.as_mut(), info).ok();

        if stream.pos() == stream.len()? as u64 {
            return Ok(());
        }
        self.back = Bitmap::from_stream(stream.as_mut(), info).ok();

        Ok(())
    }

    fn save(&self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        let mut stream = Box::new(BinaryStream::from_stream_with_endian(stream, info.endian));

        // TODO: Get version from system info
        let version = 1;

        stream.write_uint32(version)?;

        save_object(self, &mut stream, info)?;

        stream.write_uint32(self.some_num_1)?;
        stream.write_uint32(self.some_num_2)?;

        stream.write_prefixed_string(&self.right_ext_path)?;
        stream.write_prefixed_string(&self.left_ext_path)?;
        stream.write_prefixed_string(&self.top_ext_path)?;
        stream.write_prefixed_string(&self.bottom_ext_path)?;
        stream.write_prefixed_string(&self.front_ext_path)?;
        stream.write_prefixed_string(&self.back_ext_path)?;

        stream.write_boolean(self.some_bool)?;

        let textures = [
            &self.right,
            &self.left,
            &self.top,
            &self.bottom,
            &self.front,
            &self.back,
        ];

        for tex in textures {
            if let Some(bitmap) = tex {
                bitmap.save(stream.as_mut(), info)?;
            }
        }

        Ok(())
    }
}