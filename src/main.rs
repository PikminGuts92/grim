use std::env;
use std::path::Path;

mod grim;
use grim::*;

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

    let reader_result = grim::FileReader::new(file_path);
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

    let seek_res = reader_box.as_mut().seek(4);
    match seek_res {
        Ok(_) => {
            println!("Successfully seeked file");

            let pos = reader_box.as_mut().position();
            println!("Current position: {}", pos);
        },
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    }

    let block_offset_res = reader_box.as_mut().read_int32();
    match block_offset_res {
        Ok(block_offset) => {
            println!("Block offset: {}", block_offset);
        },
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    }

    let seek_res = reader_box.as_mut().seek(4);
    match seek_res {
        Ok(_) => {
            println!("Successfully seeked file");

            let str_value_res = reader_box.as_mut().read_prefixed_string();

            match str_value_res {
                Ok(str_value) => {
                    println!("String value: {}", str_value);
                },
                Err(err) => {
                    println!("{:?}", err.as_ref());
                }
            }
        },
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    }
}
