use std::path::PathBuf;

#[derive(Debug)]
pub struct Tex {
    pub name: String,
    pub rgba: Vec<u8>,
    pub png_path: PathBuf
}