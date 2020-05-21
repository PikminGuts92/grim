//use flate2::{Compress, Decompress};
use flate2::read::{ZlibDecoder, ZlibEncoder};
use flate2::{Decompress, FlushDecompress, Status};
use std::error::Error;
use std::io::Read;

pub fn inflate_zlib_block(data: &Box<[u8]>, buffer_size: usize) -> Result<Box<[u8]>, Box<dyn Error>> {
    let mut test = vec![0u8; buffer_size];
    let buffer = test.as_mut_slice();

    let mut decoder = Decompress::new(false);
    let status = decoder.decompress(data.as_ref(), buffer, FlushDecompress::Finish)?;

    match status {
        Status::StreamEnd => {
            let inflate_size = decoder.total_out();
            let mut inflated_data = vec![0u8; inflate_size as usize];
            inflated_data.clone_from_slice(&buffer[..inflate_size as usize]);

            Ok(inflated_data.into_boxed_slice())
        },
        _ => {
            Ok(vec![0u8; 0].into_boxed_slice())
        }
    }
}
