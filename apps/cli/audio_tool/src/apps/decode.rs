use crate::apps::SubApp;
use clap::Parser;

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
pub struct DecoderApp {
    #[arg(help = "Path to input audio (.vgs)", required = true)]
    pub input_path: String,
    #[arg(help = "Path to output audio", required = true)]
    pub output_path: String,
}

impl SubApp for DecoderApp {
    fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let input_path = Path::new(&self.input_path);
        let ouput_path = Path::new(&self.output_path);

        Ok(())
    }
}