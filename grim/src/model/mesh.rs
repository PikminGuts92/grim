use russimp::scene::Scene;
use std::error::Error;

pub fn open_model(model_path: &str) -> Result<(), Box<dyn Error>> {
    let model = Scene::from_file(file_path, flags);
}