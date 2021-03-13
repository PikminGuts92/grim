use crate::io::*;
use std::error::Error;

#[derive(Debug, Default)]
pub struct Anim {
    pub anim_rate: f32,
    pub unknown: u32,
}

impl Anim {
    pub fn write_to_stream(&self, writer: &mut BinaryStream) -> Result<(), Box<dyn Error>> {
        // Write version
        writer.write_int32(4)?;

        writer.write_float32(self.anim_rate)?;
        writer.write_uint32(self.unknown)?;

        Ok(())
    }
}