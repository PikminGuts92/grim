use crate::{SystemInfo};
use crate::io::MemoryStream;
use crate::scene::*;

#[derive(Debug)]
pub enum Object {
    Tex(Tex),
    Packed(PackedObject),
}

#[derive(Debug)]
pub struct PackedObject {
    pub name: String,
    pub object_type: String,
    pub data: Vec<u8>
}

impl Object {
    pub fn get_name(&self) -> &str {
        match self {
            Object::Tex(tex) => &tex.name,
            Object::Packed(packed) => &packed.name,
        }
    }

    pub fn get_type(&self) -> &str {
        match self {
            Object::Tex(_) => "Tex",
            Object::Packed(packed) => &packed.object_type,
        }
    }

    pub fn unpack(&self, info: &SystemInfo) -> Option<Object> {
        match self {
            Object::Packed(packed) => {
                let mut stream = MemoryStream::from_slice_as_read(&packed.data[..]);

                match &packed.object_type[..] {
                    "Tex" => {
                        match Tex::from_stream(&mut stream, info) {
                            Ok(mut tex) => {
                                tex.name = packed.name.to_owned();
                                Some(Object::Tex(tex))
                            },
                            Err(_) => None,
                        }
                    },
                    _ => None
                }
            },
            _ => None
        }
    }
}