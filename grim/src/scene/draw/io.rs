use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::scene::*;
use crate::SystemInfo;
use grim_traits::scene::*;
use std::error::Error;

fn is_version_supported(version: u32) -> bool {
    match version {
        1 => true,     // GH1
        3 | 4 => true, // TBRB/GDRB
        _ => false
    }
}

impl ObjectReadWrite for DrawObject {
    fn load(&mut self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        let mut reader = Box::new(BinaryStream::from_stream_with_endian(stream, info.endian));

        load_draw(self, &mut reader, info, true)
    }

    fn save(&self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        let mut writer = Box::new(BinaryStream::from_stream_with_endian(stream, info.endian));

        save_draw(self, &mut writer, info, true)
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

    if version < 3 {
        // Reads child drawables
        let draw_count = reader.read_uint32()?;
        for _ in 0..draw_count {
            // TODO: Collect into struct field
            reader.read_prefixed_string()?;
        }
    }

    load_sphere(draw.get_sphere_mut(), reader)?;

    if version >= 3 {
        draw.set_draw_order(reader.read_float32()?);
    }

    if version >= 4 {
        draw.set_override_include_in_depth_only_pass(reader.read_uint32()?.into());
    }

    Ok(())
}

pub(crate) fn save_draw<T: Draw>(draw: &T, writer: &mut Box<BinaryStream>, info: &SystemInfo, write_meta: bool)  -> Result<(), Box<dyn Error>> {
    // TODO: Get version from system info
    let version = 4;
    writer.write_uint32(version)?;

    if write_meta {
        save_object(draw, writer, info)?;
    }

    writer.write_boolean(draw.get_showing())?;
    save_sphere(draw.get_sphere(), writer)?;
    writer.write_float32(draw.get_draw_order())?;

    if version >= 4 {
        writer.write_uint32(*draw.get_override_include_in_depth_only_pass() as u32)?;
    }

    Ok(())
}