use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::SystemInfo;
use grim_traits::scene::{Color3, Matrix, MiloObject};
use std::error::Error;

pub trait ObjectReadWrite {
    fn load(&mut self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>>;
    fn save(&self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>>;
}

pub(crate) fn load_object<T: MiloObject>(obj: &mut T, reader: &mut Box<BinaryStream>, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
    // Skip revision for now
    reader.read_uint32()?;

    // Read type
    obj.set_type(reader.read_prefixed_string()?);

    // Read props
    let has_dtb = reader.read_boolean()?;
    if has_dtb {
        todo!("Props parsing not supported!");
    }

    // Read note
    if info.version >= 25 {
        obj.set_note(reader.read_prefixed_string()?);
    }

    Ok(())
}

pub (crate) fn load_color3(color: &mut Color3, reader: &mut Box<BinaryStream>) -> Result<(), Box<dyn Error>> {
    color.r = reader.read_float32()?;
    color.g = reader.read_float32()?;
    color.b = reader.read_float32()?;

    Ok(())
}

pub (crate) fn load_matrix(mat: &mut Matrix, reader: &mut Box<BinaryStream>) -> Result<(), Box<dyn Error>> {
    mat.m11 = reader.read_float32()?;
    mat.m12 = reader.read_float32()?;
    mat.m13 = reader.read_float32()?;
    mat.m14 = 0.0;

    mat.m21 = reader.read_float32()?;
    mat.m22 = reader.read_float32()?;
    mat.m23 = reader.read_float32()?;
    mat.m24 = 0.0;

    mat.m31 = reader.read_float32()?;
    mat.m32 = reader.read_float32()?;
    mat.m33 = reader.read_float32()?;
    mat.m34 = 0.0;

    mat.m41 = reader.read_float32()?;
    mat.m42 = reader.read_float32()?;
    mat.m43 = reader.read_float32()?;
    mat.m44 = 1.0;

    Ok(())
}