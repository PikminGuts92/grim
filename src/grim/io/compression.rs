//use flate2::{Compress, Decompress};
use flate2::read::{ZlibDecoder, ZlibEncoder};
use std::error::Error;
use std::io::Read;

fn inflate_zlib_block(data: &Box<[u8]>, buffer_size: usize) -> Result<Box<[u8]>, Box<dyn Error>> {
    // Create decoder
    let mut decoder = match buffer_size {
        0 => ZlibDecoder::new(data.as_ref()),
        _ => {
            let buffer = vec![0u8; buffer_size];
            ZlibDecoder::new_with_buf(data.as_ref(), buffer)
        }
    };

    // Inflate block
    let mut inflated_data: Vec<u8> = Vec::new();
    decoder.read_to_end(&mut inflated_data)?;
    Ok(inflated_data.into_boxed_slice())
}