use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use thiserror::Error as ThisError;

use crate::SystemInfo;
use super::Object;

pub type MiloDirId = u32;
pub type ObjectId = u32;

pub struct DirFile {
    pub path: PathBuf,
    pub root: MiloDirId,
}

pub struct MiloDir {
    pub object: Option<ObjectId>,
    pub objects: Vec<ObjectId>,
    pub subdirs: Vec<MiloDirId>,
    pub inline_subdirs: Vec<MiloDirId>, // Should be DirFile id?
    pub proxy_file: Option<MiloDirId>,  // ^^^
}

impl MiloDir {
    pub fn is_proxy(&self) -> bool {
        self.proxy_file.is_some()
    }
}

#[derive(Default)]
pub struct MiloEnvironment {
    dir_files: Vec<DirFile>,
    object_dirs: HashMap<MiloDirId, MiloDir>,
    objects: HashMap<ObjectId, Object>,
    next_id: u32,
}

#[derive(Debug, ThisError)]
pub enum LoadMiloFileError {
    #[error("File not found: {path}")]
    MiloFileNotFound {
        path: PathBuf
    },
}

impl MiloEnvironment {
    pub fn new() -> MiloEnvironment {
        MiloEnvironment::default()
    }

    fn get_next_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        id
    }

    pub fn get_object(&self, id: ObjectId) -> &Object {
        // TODO: Return option or result?
        self.objects.get(&id).unwrap()
    }

    pub fn get_object_mut(&mut self, id: ObjectId) -> &mut Object {
        // TODO: Return option or result?
        self.objects.get_mut(&id).unwrap()
    }

    pub fn get_object_dir(&self, id: MiloDirId) -> &MiloDir {
        // TODO: Return option or result?
        self.object_dirs.get(&id).unwrap()
    }

    pub fn get_object_dir_mut(&mut self, id: MiloDirId) -> &mut MiloDir {
        // TODO: Return option or result?
        self.object_dirs.get_mut(&id).unwrap()
    }

    pub fn load_dir<T: AsRef<Path>>(&mut self, path: T, info: Option<&SystemInfo>) -> Result<(), LoadMiloFileError> {
        Ok(())
    }
}