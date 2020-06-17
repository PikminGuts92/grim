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

#[derive(Clap, Debug)]
pub struct Dir2MiloApp {
    #[clap(about = "Path to input directory", required = true)]
    pub dir_path: String,
    #[clap(about = "Path to output milo scene", required = true)]
    pub milo_path: String,
}

// TODO: Get from args
const SYSTEM_INFO: SystemInfo = SystemInfo {
    version: 10,
    platform: Platform::PS2,
    endian: IOEndian::Little,
};

impl SubApp for Dir2MiloApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let dir_path = Path::new(&self.dir_path);
        let milo_path = Path::new(&self.milo_path);

        let dir_obj = ObjectDir::from_path(&dir_path, &SYSTEM_INFO)?;

        Ok(())
    }
}