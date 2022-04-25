//use flate2::{Compress, Decompress};

use flate2::{Compress, Compression, Decompress, FlushCompress, FlushDecompress, Status};
use std::error::Error;


pub fn inflate_zlib_block(data: &[u8], buffer: &mut [u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    if data.is_empty() {
        // Fast exit
        return Ok(Vec::new());
    }

    let mut decompressor = Decompress::new(false);
    let status = decompressor.decompress(data, buffer, FlushDecompress::Finish)?;

    match &status {
        Status::StreamEnd => {
            let inflate_size = decompressor.total_out() as usize;
            let mut inflated_data = vec![0u8; inflate_size];
            inflated_data.clone_from_slice(&buffer[..inflate_size]);

            Ok(inflated_data)
        },
        _ => {
            // TODO: Return custom error
            Ok(vec![0u8; 0])
        }
    }
}

pub fn deflate_zlib_block(data: &[u8], buffer: &mut [u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut compressor = Compress::new(Compression::best(), false);
    let status = compressor.compress(data, buffer, FlushCompress::Finish)?;

    match &status {
        Status::StreamEnd => {
            let deflate_size = compressor.total_out() as usize;
            let mut deflated_data = vec![0u8; deflate_size];
            deflated_data.clone_from_slice(&buffer[..deflate_size]);

            Ok(deflated_data)
        },
        _ => {
            // TODO: Return custom error
            Ok(vec![0u8; 0])
        }
    }
}