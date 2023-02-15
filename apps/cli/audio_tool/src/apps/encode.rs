use crate::apps::SubApp;
use grim::{Platform, SystemInfo};
use grim::audio::*;
use grim::io::{FileStream, IOEndian};
use grim::scene::{ObjectReadWrite, SampleData, SynthSample};

use clap::Parser;
use std::error::Error;
use std::io::{Read, Seek, Write};
use std::fs;
use std::path::{Path, PathBuf};

enum FileType {
    Vgs,
    SynthSample(u32, IOEndian)
}

#[derive(Parser, Debug)]
pub struct EncoderApp {
    #[arg(help = "Path to input audio (.wav)", required = true)]
    pub input_path: String,
    #[arg(help = "Path to output audio (.vgs)", required = true)]
    pub output_path: String,
    #[arg(short = 's', long, help = "Sample rate (Default: Use sample rate from input audio)")]
    pub sample_rate: Option<u32>,
}

impl SubApp for EncoderApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let input_path = Path::new(&self.input_path);
        let output_path = Path::new(&self.output_path);

        todo!()
        //Ok(())
    }
}
