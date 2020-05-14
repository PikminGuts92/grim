use std::env;
use std::path::Path;

mod grim;
use grim::io::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Input args: {:?}", args);

    let file_path_str;

    match args.get(1) {
        Some(arg) => {
            file_path_str = arg
        },
        None => {
            println!("Missing input file path");
            return;
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
            println!("{:?}", err);
            return;
        }
    }

    let milo = MiloArchive::from_stream(&mut reader_box);
}
