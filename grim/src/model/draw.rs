use crate::io::*;
use std::error::Error;

#[derive(Debug)]
pub struct Draw {
    pub showing: bool,
    pub bounding: (f32, f32, f32, f32),
    pub unknown: u32,
}

impl Draw {
    pub fn write_to_stream(&self, writer: &mut BinaryStream) -> Result<(), Box<dyn Error>> {
        // Write version
        writer.write_int32(3)?;

        writer.write_uint8(self.showing as u8)?;

        // Write bounding sphere
        let (x, y, z, r) = self.bounding;
        writer.write_float32(x)?;
        writer.write_float32(y)?;
        writer.write_float32(z)?;
        writer.write_float32(r)?;

        writer.write_uint32(self.unknown)?;

        Ok(())
    }
}

impl Default for Draw {
    fn default() -> Draw {
        Draw {
            showing: true,
            bounding: (0.0, 0.0, 0.0, 0.0),
            unknown: 0,
        }
    }
}