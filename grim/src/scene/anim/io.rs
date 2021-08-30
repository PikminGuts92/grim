use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::scene::*;
use crate::SystemInfo;
use grim_traits::scene::*;
use std::error::Error;

fn is_version_supported(version: u32) -> bool {
    match version {
        4 => true, // TBRB
        _ => false
    }
}

impl ObjectReadWrite for AnimObject {
    fn load(&mut self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        let mut reader = Box::new(BinaryStream::from_stream_with_endian(stream, info.endian));

        load_anim(self, &mut reader, info, true)?;
        Ok(())
    }

    fn save(&self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

pub(crate) fn load_anim<T: Anim>(anim: &mut T, reader: &mut Box<BinaryStream>, info: &SystemInfo, read_meta: bool)  -> Result<(), Box<dyn Error>> {
    let version = reader.read_uint32()?;
    if !is_version_supported(version) {
        // TODO: Switch to custom error
        panic!("Anim version \"{}\" is not supported!", version);
    }

    if read_meta {
        load_object(anim, reader, info)?;
    }

    anim.set_frame(reader.read_float32()?);
    anim.set_rate(reader.read_uint32()?.into());

    Ok(())
}