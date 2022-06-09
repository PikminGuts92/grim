use grim_macros::*;
use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use thiserror::Error as ThisError;

use crate::SystemInfo;
use super::Object;

pub type MiloDirId = u32;
pub type ObjectId = u32;

pub struct DirFile {
    pub path: PathBuf, // Absolute path?
    pub root: MiloDirId,
}

/*#[milo]
pub struct ObjectDir {
    pub entries: Vec<ObjectId>,
    pub subdirs: Vec<ObjectId>,
    pub proxy_file: Option<ObjectId>,
    pub inline_subdir: bool,
    pub path_name: String, // TODO: Use index to path?
}

impl ObjectDir {
    pub fn is_proxy(&self) -> bool {
        self.proxy_file.is_some()
    }
}*/

pub struct MiloDir {
    pub object: Option<ObjectId>,
    pub objects: Vec<ObjectId>,
    pub subdirs: Vec<MiloDirId>,
    pub inline_subdirs: Vec<MiloDirId>, // Should be DirFile id?
    pub proxy_file: Option<MiloDirId>,  // ^^^
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

// MiloDirId -> MiloDir -> ObjectId -> Object

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

    pub fn load_dir<T: AsRef<Path>>(&mut self, path: T, info: Option<&SystemInfo>) -> Result<MiloDirId, LoadMiloFileError> {
        // TODO: Check if path exists
        let milo_path = path.as_ref();

        // TODO: Check if path is already loaded
        //let abs_path = std::fs::canonicalize(milo_path).unwrap();

        /*match abs_path {
            Ok(_) => { print!("Successfully creating abs path"); },
            _ => { println!("Error creating abs path :("); }
        }*/

        if let Some(file_path) = milo_path.to_str() {
            println!("Path is {}", file_path);
        }

        let mut stream: Box<dyn crate::io::Stream> = Box::new(crate::io::FileStream::from_path_as_read_open(milo_path).unwrap()); // TODO: Use ?
        let milo = crate::io::MiloArchive::from_stream(&mut stream).unwrap(); // TODO: Use ?

        let system_info = SystemInfo::guess_system_info(&milo, &milo_path);

        // Create directory
        let mut dir = DirFile {
            path: milo_path.to_owned(),
            root: 0,
        };
        self.dir_files.push(dir);


        println!("Opened milo directory!");
        Ok(0)
    }
}