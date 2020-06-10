use crate::apps::{SubApp};
use clap::{App, Arg, Clap};
use std::cmp::Ordering;
use std::error::Error;
use std::fs;
use std::path::Path;
use thiserror::Error;

use grim::io::*;
use grim::scene::{Object, ObjectDir};

// TODO: Use this error somewhere or refactor
#[derive(Error, Debug)]
pub enum ArgError {
    #[error("Missing input file path")]
    NoInputPath
}

#[derive(Clap, Debug)]
pub struct Milo2DirApp {
    #[clap(about = "Path to input milo scene", required = true)]
    pub milo_path: String,
    #[clap(about = "Path to output directory", required = true)]
    pub dir_path: String
}

impl SubApp for Milo2DirApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let milo_path = Path::new(&self.milo_path);
        let dir_path = Path::new(&self.dir_path);

        if let Some(file_name) = milo_path.file_name() {
            let file_name = match  file_name.to_str() {
                Some(name) => name,
                None => "file"
            };

            println!("Opening {}", file_name);
        }

        let mut stream: Box<dyn Stream> = Box::new(FileStream::from_path_as_read_open(milo_path)?);
        let milo = MiloArchive::from_stream(&mut stream)?;

        let mut obj_dir = milo.unpack_directory()?;
        obj_dir.entries.sort_by(compare_entries_by_name);
        extract_contents(&obj_dir, dir_path)?;

        Ok(())
    }
}

fn extract_contents(milo_dir: &ObjectDir, output_path: &Path) -> Result<(), Box<dyn Error>> {
    for obj in milo_dir.entries.iter() {
        let entry = match obj {
            Object::Packed(packed) => packed
        };

        let entry_dir = Path::join(output_path, &entry.object_type);
        if !entry_dir.exists() {
            // Not found, create directory
            fs::create_dir_all(&entry_dir)?;
        }

        let entry_path = Path::join(&entry_dir, &entry.name);
        
        let mut stream = FileStream::from_path_as_read_write_create(&entry_path)?;
        stream.write_bytes(entry.data.as_slice())?;

        if let Some(name) = entry_path.to_str() {
            println!("Wrote {}", name);
        }
    }

    Ok(())
}

fn compare_entries_by_name(a : &grim::scene::Object, b: &grim::scene::Object) -> Ordering {
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
