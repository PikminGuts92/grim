use crate::grim::io::compression::*;
use crate::grim::io::stream::{MemoryStream, SeekFrom, Stream};
use crate::grim::scene::{Object, ObjectDir, PackedObject};
use std::fmt::{Display, Formatter};
use std::path::Path;
use thiserror::Error;

const ADDE_PADDING: [u8; 4] = [0xAD, 0xDE, 0xAD, 0xDE];

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
    structure: BlockStructure,
    data: Vec<u8>
}

#[derive(Debug, Error)]
pub enum MiloBlockStructureError {
    #[error("Unsupported compression with magic of 0x{magic:X}")]
    UnsupportedCompression {
        magic: u32
    }
}

impl MiloArchive {
    pub fn from_stream(stream: &mut Box<dyn Stream>) -> Result<MiloArchive, Box<dyn std::error::Error>> {
        let magic = MiloArchive::read_magic_and_offset(stream)?;

        let reader = stream.as_mut();

        let block_count = reader.read_int32()?;
        let max_inflate_size = reader.read_int32()?;

        let mut block_sizes: Vec<i32> = vec![0; block_count as usize];

        for size in block_sizes
            .iter_mut() {
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
        reader.seek(SeekFrom::Start(offset as u64))?;

        let mut uncompressed: Vec<u8> = Vec::new();

        for block_size in block_sizes.iter() {
            let bytes = reader.read_bytes(*block_size as usize)?;

            let mut data = inflate_zlib_block(&bytes, max_inflate_size as usize)?;
            uncompressed.append(&mut data);
        }

        Ok(MiloArchive {
            structure: magic,
            data: uncompressed
        })
    }

    fn read_magic_and_offset(stream: &mut Box<dyn Stream>) -> Result<BlockStructure, Box<dyn std::error::Error>> {
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

    pub fn get_stream<'a>(&'a self) -> Box<dyn Stream + 'a> {
        let stream = MemoryStream::from_slice_as_read(&self.data);
        Box::new(stream)
    }

    pub fn unpack_directory(&self) -> Result<ObjectDir, Box<dyn std::error::Error>> {
        // TODO: Pass platform info as args
        //   For now assume GH1 PS2

        let mut stream = self.get_stream();
        let reader = stream.as_mut();

        let version = reader.read_int32()?; // TODO: Evaluate version
        let entry_count = reader.read_int32()?;

        let mut packed_entries: Vec<PackedObject> = Vec::new();

        // Parse entry types + names
        for _ in 0..entry_count {
            let entry_type = reader.read_prefixed_string()?;
            let entry_name = reader.read_prefixed_string()?;

            packed_entries.push(PackedObject {
                name: entry_name,
                object_type: entry_type,
                data: Vec::new()
            })
        }

        if version == 10 {
            // Read external paths
            let ext_count = reader.read_int32()?;

            // TODO: Collect into struct
            for _ in 0..ext_count {
                reader.read_prefixed_string()?;
            }
        }

        // Get data for entries
        for entry_obj in packed_entries.iter_mut() {
            if let Some(size) = self.guess_entry_size(reader)? {
                // Read data and skip padding
                entry_obj.data = reader.read_bytes(size)?;
                reader.seek(SeekFrom::Current(4))?;
            } else {
                // TODO: Else throw error?
                break;
            }
        }

        Ok(ObjectDir {
            entries: packed_entries
                .into_iter()
                .map(|p| Object::Packed(p))
                .collect()
        })
    }

    fn guess_entry_size<'a>(&'a self, reader: &mut dyn Stream) -> Result<Option<usize>, Box<dyn std::error::Error>> {
        let start_pos = reader.position();
        let stream_len = reader.len()?;

        let mut magic: i32;

        loop {
            if let None = reader.seek_until(&ADDE_PADDING)? {
                // End of file reached
                reader.seek(SeekFrom::Start(start_pos))?;
                return Ok(None);
            }

            // Found padding, skip needle bytes
            reader.seek(SeekFrom::Current(4))?;

            if (reader.position() as usize) >= stream_len {
                // EOF reached
                break;
            }

            // Checks magic because ADDE padding can also be found in some Tex files as pixel data
            // This should reduce false positives
            magic = reader.read_int32()?;
            reader.seek(SeekFrom::Current(-4))?;

            if magic >= 0 && magic <= 0xFF {
                break;
            }
        }

        // Calculates size and returns to start of stream
        let entry_size = (reader.position() - (start_pos + 4)) as usize;
        reader.seek(SeekFrom::Start(start_pos))?;

        Ok(Some(entry_size))
    }
}
