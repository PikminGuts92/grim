use crate::{SystemInfo};
use crate::io::{BinaryStream, MemoryStream, SeekFrom, Stream};
use crate::scene::*;

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
}