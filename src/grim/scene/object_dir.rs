#[derive(Debug)]
pub enum MiloObject {
    Packed {
        name: String,
        r#type: String,
        data: Vec<u8>
    }
}

#[derive(Debug)]
pub struct ObjectDir {
    entries: Vec<MiloObject>
}

impl ObjectDir {
    pub fn new() -> ObjectDir {
        ObjectDir {
            entries: Vec::new()
        }
    }
}