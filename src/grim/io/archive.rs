use crate::grim::io::stream::StreamReader;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::Path;

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

#[derive(Debug, Clone)]
struct MiloBlockStructureError {
    message: String,
}

impl Display for MiloBlockStructureError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "")//self.message.as_str())
    }
}

impl Error for MiloBlockStructureError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl MiloBlockStructureError {
    fn new(msg: String) -> MiloBlockStructureError {
        MiloBlockStructureError {
            message: msg,
        }
    }
}


impl MiloArchive {
    pub fn from_stream(stream: &mut Box<dyn StreamReader>) -> Result<MiloArchive, Box<dyn Error>> {
        let magic = MiloArchive::read_magic_and_offset(stream)?;



        Ok(MiloArchive {
            
        })
    }

    fn read_magic_and_offset(stream: &mut Box<dyn StreamReader>) -> Result<BlockStructure, Box<dyn Error>> {
        let reader = stream.as_mut();

        // TODO: Read as u32
        let magic = reader.read_int32()? as u32;
        let block_offset = reader.read_int32()? as u32;

        match magic {
            0xCABEDEAF => Ok(BlockStructure::TypeA(block_offset)),
            0xCBBEDEAF => Ok(BlockStructure::TypeB(block_offset)),
            0xCCBEDEAF => Ok(BlockStructure::TypeC(block_offset)),
            0xCDBEDEAF => Ok(BlockStructure::TypeD(block_offset)),
            _ => Err(Box::new(MiloBlockStructureError::new(String::from("Unsupported magic"))))
        }
    }
}
