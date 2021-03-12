use gltf::{Gltf, Scene};
use std::error::Error;
use std::path::Path;

pub fn open_model<T>(model_path: T) -> Result<(), Box<dyn Error>> where T: AsRef<Path>  {
    let model = Gltf::open(model_path)?;

    Ok(())
}