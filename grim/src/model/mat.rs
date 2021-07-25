use crate::io::*;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Mat {
    base_mat: Vec<u8>,
    pub name: String,
    pub base_color: [f32; 4], // rgba
    pub diffuse_tex: String,
    pub normal_tex: String,
    pub specular_tex: String,
}

impl Mat {
    pub fn from_mat_file<T>(mat_path: T) -> Result<Mat, Box<dyn Error>> where T: AsRef<Path> {
        // Read file to bytes
        let mut file = File::open(mat_path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        Ok(Mat {
            base_mat: data,
            name: String::default(),
            base_color: Default::default(),
            diffuse_tex: String::default(),
            normal_tex: String::default(),
            specular_tex: String::default(),
        })
    }

    pub fn write_to_file<T>(&self, out_path: T) -> Result<(), Box<dyn Error>> where T: AsRef<Path> {
        const PART_1_1_SIZE: usize = 21;
        const PART_1_2_SIZE: usize = 68;
        const PART_2_SIZE: usize = 22;
        const PART_3_SIZE: usize = 4;

        let mut offset: usize = 0;
        let mut size_buf = [0u8; 4];

        // Read 1st mat part
        let part_1_1 = &self.base_mat[offset..(offset + PART_1_1_SIZE)];
        offset += PART_1_1_SIZE + 16; // Skip base color

        let part_1_2 = &self.base_mat[offset..(offset + PART_1_2_SIZE)];
        offset += PART_1_2_SIZE;

        // Skip diffuse tex name
        offset += self.read_as_i32(offset, &mut size_buf) as usize + 4;

        // Skip next pass mat name
        offset += self.read_as_i32(offset, &mut size_buf) as usize + 4;

        // Read 2nd mat part
        let part_2 = &self.base_mat[offset..(offset + PART_2_SIZE)];
        offset += PART_2_SIZE;

        // Skip normal tex name
        offset += self.read_as_i32(offset, &mut size_buf) as usize + 4;

        // Read 3rd mat part
        let part_3 = &self.base_mat[offset..(offset + PART_3_SIZE)];
        offset += PART_3_SIZE;

        // Skip specular tex name
        offset += self.read_as_i32(offset, &mut size_buf) as usize + 4;

        // Read 4th mat part
        let part_4 = &self.base_mat[offset..]; // Size = 105

        // Write to file
        let mut stream = FileStream::from_path_as_read_write_create(out_path.as_ref())?;
        let mut writer = BinaryStream::from_stream_with_endian(&mut stream, IOEndian::Big);

        writer.write_bytes(part_1_1)?;

        // Write base color
        writer.write_float32(self.base_color[0])?; // r
        writer.write_float32(self.base_color[1])?; // g
        writer.write_float32(self.base_color[2])?; // g
        writer.write_float32(self.base_color[3])?; // a

        writer.write_bytes(part_1_2)?;
        writer.write_prefixed_string(&self.diffuse_tex)?;
        writer.write_uint32(0)?; // Next mat pass
        writer.write_bytes(part_2)?;
        writer.write_prefixed_string(&self.normal_tex)?;
        writer.write_bytes(part_3)?;
        writer.write_prefixed_string(&self.specular_tex)?;
        writer.write_bytes(part_4)?;

        Ok(())
    }

    fn read_as_i32(&self, offset: usize, buffer: &mut [u8; 4]) -> i32 {
        let size_slice = &self.base_mat[offset..(offset + 4)];

        buffer.copy_from_slice(size_slice);
        i32::from_be_bytes(*buffer)
    }
}