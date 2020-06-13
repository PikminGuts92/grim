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

// TODO: Use for post GH1 games
#[derive(Debug)]
struct ObjectMeta {
    revision: i32,
    script_name: String,
    // script: Script,
    comments: String,
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
}
