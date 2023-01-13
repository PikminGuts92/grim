use crate::apps::{SubApp};
use clap::Parser;

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use grim::{Platform, SystemInfo};
use grim::model::*;
use grim::io::*;

#[derive(Parser, Debug)]
pub struct Milo2GltfApp {
    #[arg(help = "Path to input milo scene", required = true)]
    pub milo_path: String,
    #[arg(help = "Path to output directory", required = true)]
    pub output_path: String,
    #[arg(short = 'n' , long, help = "Gltf base file name")]
    pub name: Option<String>,
}

impl SubApp for Milo2GltfApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        //let asset_man = open_model(&self.model_path, SYSTEM_INFO)?;
        //asset_man.dump_to_directory(&self.output_path)

        let milo_path = Path::new(&self.milo_path);
        let dir_path = Path::new(&self.output_path);

        if let Some(file_name) = milo_path.file_name() {
            let file_name = file_name.to_str().unwrap_or("file");

            println!("Opening {}", file_name);
        }

        let mut stream: Box<dyn Stream> = Box::new(FileStream::from_path_as_read_open(milo_path)?);
        let milo = MiloArchive::from_stream(&mut stream)?;

        // Guess system info
        let system_info = SystemInfo::guess_system_info(&milo, &milo_path);
        let mut obj_dir = milo.unpack_directory(&system_info)?;
        obj_dir.unpack_entries(&system_info)?;

        let bones = find_bones(&obj_dir);

        let bone_count = calc_total_bone_count(&bones);
        println!("Found {} bones\n", bone_count);

        //print_bones(&bones, 0);

        export_object_dir_to_gltf(&obj_dir, &dir_path);

        Ok(())
    }
}

fn print_bones(bones: &Vec<BoneNode>, depth: usize) {
    for bone in bones {
        println!("{}{}", "  ".repeat(depth), bone.object.get_name());

        if !bone.children.is_empty() {
            print_bones(&bone.children, depth + 1);
        }
    }
}

fn calc_total_bone_count(bones: &Vec<BoneNode>) -> usize {
    bones
        .iter()
        .map(|b| calc_total_bone_count(&b.children))
        .sum::<usize>() + bones.len()
}