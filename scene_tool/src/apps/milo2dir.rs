use crate::apps::{SubApp};
use clap::{App, Arg, Clap};
use std::cmp::Ordering;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

use grim::{Platform, SystemInfo};
use grim::io::*;
use grim::scene::{Object, ObjectDir, PackedObject, Tex};
use grim::texture::{Bitmap, write_rgba_to_file};

// TODO: Use this error somewhere or refactor
#[derive(Debug, Error)]
pub enum ArgError {
    #[error("Missing input file path")]
    NoInputPath
}

#[derive(Debug, Error)]
pub enum TexExtractionError {
    #[error("Texture doesn't contain en embedded bitmap")]
    TextureContainsNoBitmap
}

// TODO: Get from args
const SYSTEM_INFO: SystemInfo = SystemInfo {
    version: 10,
    platform: Platform::PS2,
    endian: IOEndian::Little,
};

#[derive(Clap, Debug)]
pub struct Milo2DirApp {
    #[clap(about = "Path to input milo scene", required = true)]
    pub milo_path: String,
    #[clap(about = "Path to output directory", required = true)]
    pub dir_path: String,
    #[clap(long, about = "Automatically convert textures to PNG")]
    pub convert_textures: bool
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

        let mut obj_dir = milo.unpack_directory(&SYSTEM_INFO)?;
        //obj_dir.unpack_entries(&SYSTEM_INFO);

        //obj_dir.entries.sort_by(compare_entries_by_name);
        extract_contents(&obj_dir, dir_path, self.convert_textures, &SYSTEM_INFO)?;

        Ok(())
    }
}

fn extract_contents(milo_dir: &ObjectDir, output_path: &Path, convert_texures: bool, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
    for obj in milo_dir.entries.iter() {
        let entry_type = obj.get_type();

        let entry_dir = Path::join(output_path, entry_type);
        if !entry_dir.exists() {
            // Not found, create directory
            fs::create_dir_all(&entry_dir)?;
        }

        // First try parsing object
        if convert_texures {
            if let Some(unpacked) = obj.unpack(info) {
                match &unpacked {
                    Object::Tex(tex) => {
                        if let Some(_) = tex.bitmap {
                            if let Ok(_) = extract_tex_object(tex, &entry_dir, info) {
                                continue;
                            }
                        }
                    },
                    _ => {
                        continue; // Shouldn't be reached
                    }
                }
            }
        }
        
        // Just write raw bytes if can't convert or not selected
        if let Object::Packed(packed) = obj {
            if let Err(_) = extract_packed_object(packed, &entry_dir) {
                println!("There was an error extracting {}", obj.get_name());
            }
        }
    }

    Ok(())
}

fn extract_packed_object(packed: &PackedObject, entry_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    let entry_name = &packed.name;
    let entry_path = Path::join(&entry_dir, entry_name);

    let mut stream = FileStream::from_path_as_read_write_create(&entry_path)?;
    stream.write_bytes(packed.data.as_slice())?;

    if let Some(name) = entry_path.to_str() {
        println!("Wrote {}", name);
    }

    Ok(())
}

fn extract_tex_object(tex: &Tex, entry_dir: &PathBuf, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
    // TODO : Refactor hacky way of getting file name with png extension
    let entry_name = match Path::new(&tex.name).file_stem() {
        Some(name) => match name.to_str() {
            Some(name_str) => format!("{}.png", name_str),
            None => tex.name.to_owned()
        },
        None => tex.name.to_owned()
    };

    let entry_path = Path::join(&entry_dir, &entry_name);

    let bitmap = match &tex.bitmap {
        Some(bitmap) => bitmap,
        None => {
            return Err(Box::new(TexExtractionError::TextureContainsNoBitmap))
        }
    };

    let rgba = bitmap.unpack_rgba(info)?;
    write_rgba_to_file(bitmap.width as u32, bitmap.height as u32, &rgba[..], &entry_path)?;

    if let Some(name) = entry_path.to_str() {
        println!("Wrote {}", name);
    }

    Ok(())
}

fn compare_entries_by_name(a : &grim::scene::Object, b: &grim::scene::Object) -> Ordering {
    // Get entry types
    let a_type = a.get_type();
    let b_type = b.get_type();

    // First compare type
    match a_type.cmp(b_type) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => {
            // Get entry names
            let a_name = a.get_name();
            let b_name = b.get_name();
            
            // Then compare name
            a_name.cmp(b_name)
        }
    }
}
