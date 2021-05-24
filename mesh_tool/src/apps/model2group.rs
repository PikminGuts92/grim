use crate::apps::{SubApp};
use clap::{Clap};

use std::error::Error;
use std::path::{Path};


use grim::{Platform, SystemInfo};
use grim::model::*;
use grim::io::*;
use grim::scene::{Object, ObjectDir, PackedObject, Tex};


#[derive(Clap, Debug)]
pub struct Model2GroupApp {
    #[clap(about = "Path to input model file (.gltf)", required = true)]
    pub model_path: String,
    #[clap(about = "Path to output directory", required = true)]
    pub output_path: String,
    #[clap(short, long, about = "Path to base material file (.mat)", required = true)]
    pub mat_path: String,
}

// TODO: Get from args
const SYSTEM_INFO: SystemInfo = SystemInfo {
    version: 25,
    platform: Platform::PS3,
    endian: IOEndian::Big,
};

impl SubApp for Model2GroupApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let asset_man = open_model(&self.model_path, &self.mat_path)?;
        asset_man.dump_to_directory(&self.output_path)?;

        Ok(())
    }
}