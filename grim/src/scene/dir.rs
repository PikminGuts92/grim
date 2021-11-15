use std::path::{Path, PathBuf};

pub type ObjectId = u32;

pub struct DirFile {
    pub path: PathBuf,
    pub root: ObjectId,
}