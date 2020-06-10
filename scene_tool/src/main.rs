use std::cmp::Ordering;
use std::env;
use std::path::Path;
use thiserror::Error;

use grim::io::*;
use grim::scene::{Object, ObjectDir};

#[derive(Error, Debug)]
pub enum ArgError {
    #[error("Missing input file path")]
    NoInputPath
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    println!("Input args: {:?}", args);

    let file_path_str: &String;

    match args.get(1) {
        Some(arg) => {
            file_path_str = arg
        },
        None => {
            return Err(Box::new(ArgError::NoInputPath));
        }
    };

    println!("Opening file...");
    let file_path = Path::new(file_path_str);

    let mut reader: Box<dyn Stream> = Box::new(FileStream::from_path_as_read_open(file_path)?);
    let milo = MiloArchive::from_stream(&mut reader)?;

    let mut obj_dir = milo.unpack_directory()?;

    obj_dir.entries.sort_by(compare_entries);
    output_entries(&obj_dir);

    Ok(())
}

fn compare_entries(a : &grim::scene::Object, b: &grim::scene::Object) -> Ordering {
    // Unpack entries
    let a = match a {
        Object::Packed(obj) => obj
    };

    let b = match b {
        Object::Packed(obj) => obj
    };

    // Compare type then name
    match a.object_type.cmp(&b.object_type) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => a.name.cmp(&b.name)
    }
}

fn output_entries(milo_dir: &ObjectDir) {
    let mut obj_type: &str = &String::new();

    for obj in milo_dir.entries.iter() {
        let entry = match obj {
            Object::Packed(packed) => packed
        };

        // Print type if first in collection or new type
        if obj_type != &entry.object_type {
            println!("{}", &entry.object_type);
        }

        obj_type = &entry.object_type;

        println!("\t{}", &entry.name);
    }
}