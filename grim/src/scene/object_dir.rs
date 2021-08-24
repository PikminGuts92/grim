use crate::{SystemInfo};
use crate::io::{BinaryStream, FileSearchDepth, FileStream, MemoryStream, PathFinder, SeekFrom, Stream};
use crate::scene::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;

use std::path::{Path, PathBuf};

lazy_static! {
    static ref MILO_ENTRY_REGEX: Regex = Regex::new(r"(?i)([/\\][a-z]+[/\\])[^/\\]+$").unwrap();
}

pub enum ObjectDir {
    ObjectDir(ObjectDirBase)
}

pub struct ObjectDirBase {
    pub entries: Vec<Object>,
    pub name: String,
    pub dir_type: String,
    pub sub_dirs: Vec<ObjectDir>,
}

impl ObjectDirBase {
    pub fn new() -> ObjectDirBase {
        ObjectDirBase {
            entries: Vec::new(),
            name: String::new(),
            dir_type: String::new(),
            sub_dirs: Vec::new(),
        }
    }
}

impl<'a> ObjectDir {
    pub fn from_path(path: &Path, _info: &SystemInfo) -> Result<ObjectDir, Box<dyn Error>> {
        let mut obj_dir = ObjectDirBase::new();

        let files = path.find_files_with_depth(FileSearchDepth::Limited(1))?
            .into_iter()
            .filter(|f| MILO_ENTRY_REGEX.is_match(f.to_str().unwrap()))
            .collect::<Vec<PathBuf>>();

        for file_path in files.iter() {
            // Gets file name
            let entry_name = file_path
                .file_name().unwrap()
                .to_str().unwrap()
                .to_owned();

            // Gets directory name as string and converts to pascal casing
            let entry_type = file_path
                .parent().unwrap()
                .file_name().unwrap()
                .to_str().unwrap()
                .chars()
                .enumerate()
                .map(|(i, ch)| match i {
                    0 => ch.to_ascii_uppercase(),
                    _ => ch.to_ascii_lowercase()
                })
                .collect::<String>();

            // Read data from file
            let mut stream = FileStream::from_path_as_read_open(file_path)?;
            let stream_len = stream.len()?;
            let data = stream.read_bytes(stream_len)?;

            // Add entry to collection
            obj_dir.entries.push(Object::Packed(PackedObject {
                name: entry_name,
                object_type: entry_type,
                data
            }));
        }

        Ok(ObjectDir::ObjectDir(obj_dir))
    }

    pub fn get_entries(&'a self) -> &'a Vec<Object> {
        match self {
            ObjectDir::ObjectDir(dir) => &dir.entries
        }
    }

    pub fn unpack_entries(&'a mut self, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        if let ObjectDir::ObjectDir(obj_dir) = self {
            for entry in obj_dir.entries.iter_mut() {
                if let Some(new_entry) = entry.unpack(info) {
                    *entry = new_entry;
                }
            }
        }

        Ok(())
    }
}