#[derive(Debug)]
pub enum Object {
    Packed(PackedObject)
}

#[derive(Debug)]
pub struct PackedObject {
    pub name: String,
    pub object_type: String,
    pub data: Vec<u8>
}