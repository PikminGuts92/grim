use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::scene::*;
use crate::SystemInfo;
use grim_traits::scene::*;
use std::error::Error;

fn is_version_supported(version: u32) -> bool {
    match version {
        3..=4 => true, // TBRB/GDRB
        _ => false
    }
}

impl ObjectReadWrite for DrawObject {
    fn load(&mut self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        let mut reader = Box::new(BinaryStream::from_stream_with_endian(stream, info.endian));

        load_draw(self, &mut reader, info, true)?;
        Ok(())
    }

    fn save(&self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

pub(crate) fn load_draw<T: Draw>(draw: &mut T, reader: &mut Box<BinaryStream>, info: &SystemInfo, read_meta: bool)  -> Result<(), Box<dyn Error>> {
    let version = reader.read_uint32()?;
    if !is_version_supported(version) {
        // TODO: Switch to custom error
        panic!("Draw version \"{}\" is not supported!", version);
    }

    if read_meta {
        load_object(draw, reader, info)?;
    }

    draw.set_showing(reader.read_boolean()?);
    load_sphere(draw.get_sphere_mut(), reader)?;
    draw.set_draw_order(reader.read_float32()?);

    if version >= 4{
        draw.set_override_include_in_depth_only_pass(reader.read_uint32()?.into());
    }

    Ok(())
}