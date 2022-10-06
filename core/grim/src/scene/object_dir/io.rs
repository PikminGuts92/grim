use crate::io::{BinaryStream, FileStream, SeekFrom, Stream};
use crate::SystemInfo;
use std::error::Error;
use super::{ObjectDir, ObjectReadWrite};

pub(crate) fn load_object_dir_entries<T: ObjectDir>(obj_dir: &mut T, reader: &mut Box<BinaryStream>, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
    // Read and verify version
    let version = reader.read_uint32()?;
    if info.version != version {
        //return Err(Box::new(MiloUnpackError::UnsupportedDirectoryVersion { version }));
        panic!("Unsupported directory version") // TODO: Use actual error
    }

    let mut dir_class;
    let dir_name;

    if version >= 24 {
        // Read object dir name + type
        dir_class = reader.read_prefixed_string()?;
        dir_name = reader.read_prefixed_string()?;

        reader.seek(SeekFrom::Current(8))?; // Skip string table counts

        // Update class name
        // TODO: (Refactor)
        //ObjectDir::fix_class_name(version, &mut dir_type);
    } else {
        dir_class = String::new();
        dir_name = String::new();
    }

    let entry_count = reader.read_int32()?;
    //let mut packed_entries: Vec<PackedObject> = Vec::new();

    // Parse entry classes + names
    for _ in 0..entry_count {
        let mut entry_class = reader.read_prefixed_string()?;
        let entry_name = reader.read_prefixed_string()?;

        // Update class name
        // TODO: (Refactor)
        //ObjectDir::fix_class_name(version, &mut entry_type);

        /*packed_entries.push(PackedObject {
            name: entry_name,
            object_type: entry_class,
            data: Vec::new()
        })*/
    }
    
    todo!()
}

/*pub(crate) struct DirLoader<'a> {
    //pub environment: &'a mut 
}*/

pub(crate) fn load_dir_recurse() {

}