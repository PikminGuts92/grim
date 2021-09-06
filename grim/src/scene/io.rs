use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::SystemInfo;
use grim_traits::scene::{Color3, Matrix, MiloObject, Sphere};
use std::error::Error;

pub trait ObjectReadWrite {
    fn load(&mut self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>>;
    fn save(&self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>>;
}

pub(crate) fn load_object<T: MiloObject>(obj: &mut T, reader: &mut Box<BinaryStream>, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
    if info.version < 24 {
        // Don't read metadata
        return Ok(());
    }
    
    load_object_type(obj, reader, info)?;
    load_object_rest(obj, reader, info)?;

    Ok(())
}

pub(crate) fn save_object<T: MiloObject>(obj: &T, writer: &mut Box<BinaryStream>, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
    save_object_type(obj, writer, info)?;
    save_object_rest(obj, writer, info)?;

    Ok(())
}


pub(crate) fn load_object_type<T: MiloObject>(obj: &mut T, reader: &mut Box<BinaryStream>, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
    // Skip revision for now
    reader.read_uint32()?;

    // Read type
    obj.set_type(reader.read_prefixed_string()?);

    Ok(())
}

pub(crate) fn save_object_type<T: MiloObject>(obj: &T, writer: &mut Box<BinaryStream>, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
    // TODO: Get revision from system info
    writer.write_uint32(2)?;

    // Write type
    writer.write_prefixed_string(obj.get_type())?;

    Ok(())
}

pub(crate) fn load_object_rest<T: MiloObject>(obj: &mut T, reader: &mut Box<BinaryStream>, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
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

pub(crate) fn save_object_rest<T: MiloObject>(obj: &T, writer: &mut Box<BinaryStream>, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
    // TODO: Write props
    writer.write_boolean(false)?;

    // Write note
    if info.version >= 25 {
        writer.write_prefixed_string(obj.get_note())?;
    }

    Ok(())
}

pub (crate) fn load_color3(color: &mut Color3, reader: &mut Box<BinaryStream>) -> Result<(), Box<dyn Error>> {
    color.r = reader.read_float32()?;
    color.g = reader.read_float32()?;
    color.b = reader.read_float32()?;

    Ok(())
}

pub (crate) fn save_color3(color: &Color3, writer: &mut Box<BinaryStream>) -> Result<(), Box<dyn Error>> {
    writer.write_float32(color.r)?;
    writer.write_float32(color.g)?;
    writer.write_float32(color.b)?;

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

pub (crate) fn save_matrix(mat: &Matrix, writer: &mut Box<BinaryStream>) -> Result<(), Box<dyn Error>> {
    writer.write_float32(mat.m11)?;
    writer.write_float32(mat.m12)?;
    writer.write_float32(mat.m13)?;

    writer.write_float32(mat.m21)?;
    writer.write_float32(mat.m22)?;
    writer.write_float32(mat.m23)?;

    writer.write_float32(mat.m31)?;
    writer.write_float32(mat.m32)?;
    writer.write_float32(mat.m33)?;

    writer.write_float32(mat.m41)?;
    writer.write_float32(mat.m42)?;
    writer.write_float32(mat.m43)?;

    Ok(())
}

pub (crate) fn load_sphere(sphere: &mut Sphere, reader: &mut Box<BinaryStream>) -> Result<(), Box<dyn Error>> {
    sphere.x = reader.read_float32()?;
    sphere.y = reader.read_float32()?;
    sphere.z = reader.read_float32()?;
    sphere.r = reader.read_float32()?;

    Ok(())
}

pub (crate) fn save_sphere(sphere: &Sphere, writer: &mut Box<BinaryStream>) -> Result<(), Box<dyn Error>> {
    writer.write_float32(sphere.x)?;
    writer.write_float32(sphere.y)?;
    writer.write_float32(sphere.z)?;
    writer.write_float32(sphere.r)?;

    Ok(())
}