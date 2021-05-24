use crate::io::*;
use std::error::Error;

#[derive(Debug, Default)]
pub struct Trans {
    pub mat1: Matrix,
    pub mat2: Matrix,
    pub unknown: u32,
    pub camera: String,
    pub unknown_bool: bool,
    pub transform: String,
}

#[derive(Debug)]
pub struct Matrix {
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,

    pub m21: f32,
    pub m22: f32,
    pub m23: f32,

    pub m31: f32,
    pub m32: f32,
    pub m33: f32,

    pub m41: f32,
    pub m42: f32,
    pub m43: f32,
}

impl Matrix {
    pub fn identity() -> Matrix {
        Matrix {
            m11: 1.0,
            m12: 0.0,
            m13: 0.0,

            m21: 0.0,
            m22: 1.0,
            m23: 0.0,

            m31: 0.0,
            m32: 0.0,
            m33: 1.0,

            m41: 0.0,
            m42: 0.0,
            m43: 0.0,
        }
    }
}

impl Matrix {
    fn write_to_stream(&self, writer: &mut BinaryStream) -> Result<(), Box<dyn Error>> {
        writer.write_float32(self.m11)?;
        writer.write_float32(self.m12)?;
        writer.write_float32(self.m13)?;

        writer.write_float32(self.m21)?;
        writer.write_float32(self.m22)?;
        writer.write_float32(self.m23)?;

        writer.write_float32(self.m31)?;
        writer.write_float32(self.m32)?;
        writer.write_float32(self.m33)?;

        writer.write_float32(self.m41)?;
        writer.write_float32(self.m42)?;
        writer.write_float32(self.m43)?;

        Ok(())
    }
}

impl Default for Matrix {
    fn default() -> Matrix {
        Matrix::identity()
    }
}

impl Trans {
    pub fn write_to_stream(&self, writer: &mut BinaryStream) -> Result<(), Box<dyn Error>> {
        // Write version
        writer.write_int32(9)?;

        // Write matrices
        self.mat1.write_to_stream(writer)?;
        self.mat2.write_to_stream(writer)?;

        writer.write_uint32(self.unknown)?;
        writer.write_prefixed_string(&self.camera)?;
        writer.write_uint8(self.unknown_bool as u8)?;
        writer.write_prefixed_string(&self.transform)?;

        Ok(())
    }
}