use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::scene::*;
use crate::SystemInfo;
use grim_traits::scene::*;
use std::error::Error;

fn is_version_supported(version: u32) -> bool {
    match version {
        0..=8 => false,
        9 => true, // GH2 TODO: Update to "9.." once adopted in rust stable
        _ => true
    }
}

impl ObjectReadWrite for TransObject {
    fn load(&mut self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        let mut reader = Box::new(BinaryStream::from_stream_with_endian(stream, info.endian));

        load_trans(self, &mut reader, info, true)?;
        Ok(())
    }

    fn save(&self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

pub(crate) fn load_trans<T: Trans>(trans: &mut T, reader: &mut Box<BinaryStream>, info: &SystemInfo, read_meta: bool)  -> Result<(), Box<dyn Error>> {
    let version = reader.read_uint32()?;
    if !is_version_supported(version) {
        // TODO: Switch to custom error
        panic!("Trans version \"{}\" is not supported!", version);
    }

    if read_meta {
        load_object(trans, reader, info)?;
    }

    load_matrix(trans.get_local_xfm_mut(), reader)?;
    load_matrix(trans.get_world_xfm_mut(), reader)?;

    trans.set_constraint(reader.read_uint32()?.into());
    trans.set_target(reader.read_prefixed_string()?);

    trans.set_preserve_scale(reader.read_boolean()?);
    trans.set_parent(reader.read_prefixed_string()?);

    Ok(())
}