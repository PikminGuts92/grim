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
            let obj = self.entries.remove(0);

            if let Object::Packed(packed) = obj {
                match &packed.object_type[..] {
                    "Tex" => {
                        let mut stream = MemoryStream::from_slice_as_read(&packed.data[..]);

                        match Tex::from_stream(&mut stream, info) {
                            Ok(tex) => {
                                new_entries.push(Object::Tex(tex));
                            },
                            Err(_) => {
                                new_entries.push(Object::Packed(packed));
                            }
                        }
                    },
                    _ => {
                        new_entries.push(Object::Packed(packed));
                    }
                }
            } else {
                new_entries.push(obj);
            }
        }

        // Assign new entries
        self.entries = new_entries;
    }
}