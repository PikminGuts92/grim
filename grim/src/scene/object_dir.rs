use crate::{SystemInfo};
use crate::io::{BinaryStream, FileSearchDepth, FileStream, MemoryStream, PathFinder, SeekFrom, Stream};
use crate::scene::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref MILO_ENTRY_REGEX: Regex = Regex::new(r"(?i)([/\\][a-z]+[/\\])[^/\\]+$").unwrap();
}

#[derive(Debug)]
pub struct ObjectDir {
    pub entries: Vec<Object>
}

impl ObjectDir {
    pub fn new() -> ObjectDir {
        ObjectDir {
            entries: Vec::new()
        }
    }
}

impl ObjectDir {
    pub fn unpack_entries(&mut self, info: &SystemInfo) {
        let mut new_entries = Vec::<Object>::new();

        while self.entries.len() > 0 {
            let object = self.entries.remove(0);

            let new_object = match object.unpack(info) {
                Some(obj) => obj,
                None => object
            };

            new_entries.push(new_object);
        }

        // Assign new entries
        self.entries = new_entries;
    }

    pub fn from_path(path: &Path, info: &SystemInfo) -> Result<ObjectDir, Box<dyn Error>> {
        let mut obj_dir = ObjectDir::new();

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

        Ok(obj_dir)
    }
}