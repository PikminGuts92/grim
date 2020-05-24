use crate::grim::scene::Object;

#[derive(Debug)]
pub struct ObjectDir {
    pub(crate) entries: Vec<Object>
}

impl ObjectDir {
    pub fn new() -> ObjectDir {
        ObjectDir {
            entries: Vec::new()
        }
    }
}
