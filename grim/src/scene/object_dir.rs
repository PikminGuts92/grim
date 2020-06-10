use crate::scene::Object;

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
