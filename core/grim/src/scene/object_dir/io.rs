use crate::io::{BinaryStream, FileStream, SeekFrom, Stream};
use crate::SystemInfo;
use std::error::Error;
use super::{ObjectDir, ObjectReadWrite};

pub(crate) fn load_object_dir_entries<T: ObjectDir>(obj_dir: &mut T, reader: &mut Box<BinaryStream>, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
    
    
    todo!()
}