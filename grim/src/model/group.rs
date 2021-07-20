use crate::io::*;
use crate::model::{Anim, Draw, Trans};
use std::error::Error;
use std::path::Path;

#[derive(Debug)]
pub struct Group {
    pub name: String,
    pub objects: Vec<String>,
}

impl Group {
    pub fn write_to_file<T>(&self, out_path: T) -> Result<(), Box<dyn Error>> where T: AsRef<Path> {
        // Write to file
        let mut stream = FileStream::from_path_as_read_write_create(out_path.as_ref())?;
        let mut writer = BinaryStream::from_stream_with_endian(&mut stream, IOEndian::Big);

        // Write version
        writer.write_int32(14)?;

        // Write meta
        // TODO: Use struct
        writer.write_uint32(2)?; // Revision - VERY important
        writer.write_bytes(&[0u8; 9])?;

        // Write anim
        let anim = Anim::default();
        anim.write_to_stream(&mut writer)?;

        // Write trans
        let mut trans = Trans::default();
        trans.transform = self.name.to_owned();
        trans.write_to_stream(&mut writer)?;

        // Write draw
        let draw = Draw::default();
        draw.write_to_stream(&mut writer)?;

        // Write objects
        writer.write_uint32(self.objects.len() as u32)?;

        for ob in &self.objects {
            writer.write_prefixed_string(ob)?;
        }

        // Write env
        writer.write_prefixed_string("")?;

        // Write lod
        writer.write_float32(0.0)?; // Width
        writer.write_float32(0.0)?; // Height

        // Write unknown data
        writer.write_bytes(&[0u8; 5])?;

        Ok(())
    }
}