use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub struct Mat {
    base_mat: Vec<u8>,
    pub name: String,
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
            diffuse_tex: String::default(),
            normal_tex: String::default(),
            specular_tex: String::default(),
        })
    }
}