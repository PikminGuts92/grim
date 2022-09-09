use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::scene::*;
use crate::SystemInfo;
use grim_traits::scene::*;
use std::error::Error;

fn is_version_supported(version: u32) -> bool {
    match version {
        11 | 12 => true,
        _ => false
    }
}

impl ObjectReadWrite for PropAnim {
    fn load(&mut self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        let mut reader = Box::new(BinaryStream::from_stream_with_endian(stream, info.endian));

        let version = reader.read_uint32()?;
        if !is_version_supported(version) {
            // TODO: Switch to custom error
            panic!("PropAnim version \"{}\" is not supported!", version);
        }

        load_object(self, &mut reader, info)?;
        load_anim(self, &mut reader, info, false)?;

        if version >= 12 {
            self.unknown_toggle = reader.read_boolean()?;
        }

        // Reset prop keys
        self.keys.clear();

        let prop_keys_count = reader.read_uint32()?;
        for _ in 0..prop_keys_count {
            
        }

        todo!() //Ok(())
    }

    fn save(&self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        let mut writer = Box::new(BinaryStream::from_stream_with_endian(stream, info.endian));

        // TODO: Get version from system info
        let version = 11;

        writer.write_uint32(version)?;

        save_object(self, &mut writer, info)?;
        save_anim(self, &mut writer, info, false)?;

        if version >= 12 {
            writer.write_boolean(self.unknown_toggle)?;
        }

        todo!() //Ok(())
    }
}