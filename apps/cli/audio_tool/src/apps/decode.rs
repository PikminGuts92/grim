use crate::apps::SubApp;
use grim::audio::*;

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

        let _input_ext = match input_path.extension().and_then(|e| e.to_str()) {
            Some(ext) if "vgs".eq_ignore_ascii_case(ext) => ext,
            Some(ext) => {
                println!("Input audio with extension \".{ext}\" is not supported");
                return Ok(());
            },
            _ => {
                println!("Input audio not supported");
                return Ok(());
            }
        };

        let _output_ext = match ouput_path.extension().and_then(|e| e.to_str()) {
            Some(ext) if "wav".eq_ignore_ascii_case(ext) => ext,
            Some(ext) => {
                println!("Output audio with extension \".{ext}\" is not supported");
                return Ok(());
            },
            _ => {
                println!("Output audio not supported");
                return Ok(());
            }
        };

        let mut input_file = std::fs::File::open(input_path)?;
        let vgs_file = VgsFile::from_reader(&mut input_file)?;

        let channel_count = vgs_file.get_channel_count();
        let sample_rate = vgs_file.get_sample_rate();

        let channels = vgs_file.decode_samples_as_channels();
        let sample_count = channels.iter().map(|c| c.len()).max().unwrap_or_default();
        let mut interleaved_data = Vec::new();

        // TODO: Figure out more efficient way to do this
        for sample_idx in 0..sample_count {
            for sample in channels.iter().map(|c| c.get(sample_idx).map(|s| *s).unwrap_or_default()) {
                interleaved_data.push(sample);
            }
        }

        // Encode to wav
        let encoder = WavEncoder::new(&interleaved_data, channel_count as u16, sample_rate);
        encoder.encode_to_file(ouput_path).map_err(|e| Box::new(e) as Box<dyn Error>)
    }
}