use gltf::{Gltf, Scene};
use std::error::Error;

pub fn open_model<T>(model_path: T) -> Result<(), Box<dyn Error>> where T: AsRef<Path>  {
    let model = Gltf::open(path)?;
}