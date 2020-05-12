//use flate2::{Compress, Decompress};
use flate2::read::{ZlibDecoder, ZlibEncoder};
use std::error::Error;

fn inflate_zlib_block(data: &Box<[u8]>, buffer_size: usize) -> Result<Box<[u8]>, Box<dyn Error>> {
    let mut buffer = vec![0u8; buffer_size];
    let mut decoder = ZlibDecoder::new(&*data.as_ref());

    //let mut test = *buffer.as_mut_slice();
    //decoder.read(buffer.as_slice());


    //ZlibEncoder::new(r: R, level: crate::Compression)
    let mut dat: Vec<u8> = Vec::new();

    let mut compressed = ZlibDecoder::new(&*dat);

    Ok(dat.into_boxed_slice())
}