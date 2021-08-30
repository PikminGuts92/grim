use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::scene::*;
use crate::SystemInfo;
use grim_traits::scene::*;
use std::error::Error;

fn is_version_supported(version: u32) -> bool {
    match version {
        14 => true, // TBRB
        _ => false
    }
}

impl ObjectReadWrite for GroupObject {
    fn load(&mut self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        let mut reader = Box::new(BinaryStream::from_stream_with_endian(stream, info.endian));

        let version = reader.read_uint32()?;
        if !is_version_supported(version) {
            // TODO: Switch to custom error
            panic!("Group version \"{}\" is not supported!", version);
        }

        load_object(self, &mut reader, info)?;
        load_anim(self, &mut reader, info, false)?;
        load_trans(self, &mut reader, info, false)?;
        load_draw(self, &mut reader, info, false)?;

        let object_count = reader.read_uint32()?;
        self.objects.clear();
        for _ in 0..object_count {
            self.objects.push(reader.read_prefixed_string()?);
        }

        self.environ = reader.read_prefixed_string()?;
        self.lod_height = reader.read_float32()?;
        self.lod_width = reader.read_float32()?;

        let end_data = match version {
            13 => reader.read_bytes(4)?,
            _ => reader.read_bytes(5)?,
        };

        if end_data.iter().any(|d| *d != 0) {
            panic!("Unexpected Data: Ending bytes of Group object is {:?}", &end_data);
        }

        Ok(())
    }

    fn save(&self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}