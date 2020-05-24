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

    let mut reader: Box<dyn Stream> = Box::new(FileStream::new(file_path)?);
    let milo = MiloArchive::from_stream(&mut reader)?;

    let obj_dir = milo.unpack_directory()?;

    Ok(())
}
