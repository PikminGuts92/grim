#![feature(with_options)] // TODO: Move to seperate lib file
use std::env;
use std::path::Path;
use thiserror::Error;

mod grim;
use grim::io::*;

#[derive(Error, Debug)]
pub enum ArgError {
    #[error("Missing input file path")]
    NoInputPath
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    println!("Input args: {:?}", args);

    let file_path_str;

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

    let reader_result = FileReader::new(file_path);
    let mut reader_box: Box<StreamReader>;

    match reader_result {
        Ok(fr) => {
            reader_box = Box::new(fr);

            println!("Successfully opened \"{}\"", file_path_str);
        },
        Err(err) => {
            return Err(Box::new(err));
        }
    }

    let milo = MiloArchive::from_stream(&mut reader_box)?;

    Ok(())
}
