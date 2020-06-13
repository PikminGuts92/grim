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

            let new_object = match object {
                Object::Packed(packed) => {
                    let mut stream = MemoryStream::from_slice_as_read(&packed.data[..]);

                    match &packed.object_type[..] {
                        "Tex" => {
                            match Tex::from_stream(&mut stream, info) {
                                Ok(tex) => Object::Tex(tex),
                                Err(_) => Object::Packed(packed),
                            }
                        },
                        _ => Object::Packed(packed)
                    }
                },
                _ => object
            };

            new_entries.push(new_object);
        }

        // Assign new entries
        self.entries = new_entries;
    }
}