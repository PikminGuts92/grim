use crate::grim::io::compression::*;
use crate::grim::io::stream::StreamReader;
use std::fmt::{Display, Formatter};
use std::path::Path;
use thiserror::Error;

#[derive(Copy, Clone, Debug)]
pub enum BlockStructure {
    TypeA(u32), // Block structure, no compression
    TypeB(u32), // Block structure, zlib compression
    TypeC(u32), // Block structure, gzip compression
    TypeD(u32), // Block structure, zlib compression (with inflate sizes block prefixed)
    Uncompressed,
}

#[derive(Debug)]
pub struct MiloArchive {

}

#[derive(Debug, Error)]
pub enum MiloBlockStructureError {
    #[error("Unsupported compression with magic of 0x{magic:X}")]
    UnsupportedCompression {
        magic: u32
    }
}

impl MiloArchive {
    pub fn from_stream(stream: &mut Box<dyn StreamReader>) -> Result<MiloArchive, Box<dyn std::error::Error>> {
        let magic = MiloArchive::read_magic_and_offset(stream)?;

        let reader = stream.as_mut();

        let block_count = reader.read_int32()?;
        let max_inflate_size = reader.read_int32()?;

        let mut block_sizes: Vec<i32> = vec![0; block_count as usize];

        for (_, size) in block_sizes
            .iter_mut()
            .enumerate() {
            *size = reader.read_int32()?;
        }

        let offset = match magic {
            BlockStructure::TypeA(offset) => offset,
            BlockStructure::TypeB(offset) => offset,
            BlockStructure::TypeC(offset) => offset,
            BlockStructure::TypeD(offset) => offset,
            BlockStructure::Uncompressed => 0,
        };

        // Advances to first block
        reader.seek(offset as u64)?;

        let mut total_size: usize = 0;

        for block_size in block_sizes.iter() {
            let bytes = reader.read_bytes(*block_size as usize)?;

            //let uncompressed = inflate_zlib_block(&bytes, max_inflate_size as usize)?;

            let uncompressed;

            match inflate_zlib_block(&bytes, max_inflate_size as usize) {
                Ok(data) => {
                    uncompressed = data;
                },
                Err(err) => {
                    println!("{:?}", err);
                    continue;
                }
            };


            println!("Uncompressed block is {} bytes in length", uncompressed.len());
            total_size += uncompressed.len();
        }

        println!("Total uncompressed size is {} bytes in length", total_size);


        Ok(MiloArchive {
            
        })
    }

    fn read_magic_and_offset(stream: &mut Box<dyn StreamReader>) -> Result<BlockStructure, Box<dyn std::error::Error>> {
        let reader = stream.as_mut();

        // TODO: Read as u32
        let magic = reader.read_int32()? as u32;
        let block_offset = reader.read_int32()? as u32;

        match magic {
            0xCABEDEAF => Ok(BlockStructure::TypeA(block_offset)),
            0xCBBEDEAF => Ok(BlockStructure::TypeB(block_offset)),
            0xCCBEDEAF => Ok(BlockStructure::TypeC(block_offset)),
            0xCDBEDEAF => Ok(BlockStructure::TypeD(block_offset)),
            _ => Err(Box::new(MiloBlockStructureError::UnsupportedCompression { magic }))
        }
    }
}
