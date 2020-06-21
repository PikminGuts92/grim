use crate::{SystemInfo};
use crate::io::compression::*;
use crate::io::stream::{BinaryStream, MemoryStream, SeekFrom, Stream};
use crate::scene::{Object, ObjectDir, PackedObject};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::Path;
use thiserror::Error as ThisError;

const MAX_BLOCK_SIZE: usize = 0x20000;
const ADDE_PADDING: [u8; 4] = [0xAD, 0xDE, 0xAD, 0xDE];

#[derive(Copy, Clone, Debug)]
pub enum BlockType
{
    TypeA, // Block structure, no compression
    TypeB, // Block structure, zlib compression
    TypeC, // Block structure, gzip compression
    TypeD, // Block structure, zlib compression (with inflate sizes block prefixed)
}

#[derive(Debug)]
pub struct BlockInfo {
    block_type: BlockType,
    start_offset: u32,
    block_sizes: Vec<usize>,
}

impl BlockInfo {
    pub fn new() -> BlockInfo {
        BlockInfo {
            block_type: BlockType::TypeB,
            start_offset: 2064,
            block_sizes: Vec::new()
        }
    }
}

#[derive(Debug)]
pub enum MiloArchiveStructure {
    Blocked(BlockInfo),
    Uncompressed,
}

#[derive(Debug)]
pub struct MiloArchive {
    structure: MiloArchiveStructure,
    data: Vec<u8>
}

#[derive(Debug, ThisError)]
pub enum MiloBlockStructureError {
    #[error("Unsupported compression with magic of 0x{magic:X}")]
    UnsupportedCompression {
        magic: u32
    }
}

#[derive(Debug, ThisError)]
pub enum MiloUnpackError {
    #[error("Unsupported milo directory of version of {version}")]
    UnsupportedDirectoryVersion {
        version: u32
    }
}

impl MiloArchive {
    pub fn from_stream(stream: &mut Box<dyn Stream>) -> Result<MiloArchive, Box<dyn Error>> {
        let stream = stream.as_mut();
        let mut reader = BinaryStream::from_stream(stream); // Should always be little endian
        
        let mut structure: MiloArchiveStructure = MiloArchiveStructure::Uncompressed; // TODO: Handle in else case
        let mut uncompressed: Vec<u8> = Vec::new();

        if let Some(block_type) = MiloArchive::get_block_type_or_none(&mut reader)? {
            let mut block_info = BlockInfo::new();
            
            block_info.block_type = block_type;
            block_info.start_offset = reader.read_uint32()?;

            let block_count = reader.read_int32()?;
            let max_inflate_size = reader.read_int32()?;

            let mut block_sizes: Vec<i32> = vec![0; block_count as usize];

            for size in block_sizes
                .iter_mut() {
                *size = reader.read_int32()?;
            }

            // Advances to first block
            reader.seek(SeekFrom::Start(block_info.start_offset as u64))?;

            for block_size in block_sizes.iter() {
                let bytes = reader.read_bytes(*block_size as usize)?;

                let mut data = inflate_zlib_block(&bytes, max_inflate_size as usize)?;

                uncompressed.append(&mut data);
                block_info.block_sizes.push(data.len());
            }

            structure = MiloArchiveStructure::Blocked(block_info);
        } // TODO: Handle else case (should currently return error if not blocked)
        
        Ok(MiloArchive {
            structure,
            data: uncompressed
        })
    }

    fn get_block_type_or_none(reader: &mut BinaryStream) -> Result<Option<BlockType>, Box<dyn Error>> {
        let magic = reader.read_uint32()?;

        match magic {
            0xCABEDEAF => Ok(Some(BlockType::TypeA)),
            0xCBBEDEAF => Ok(Some(BlockType::TypeB)),
            0xCCBEDEAF => Ok(Some(BlockType::TypeC)),
            0xCDBEDEAF => Ok(Some(BlockType::TypeD)),
            // TODO: Assume uncompressed archive, or gzip then check version
            _ => Err(Box::new(MiloBlockStructureError::UnsupportedCompression { magic }))
        }
    }

    pub fn get_stream<'a>(&'a self) -> Box<dyn Stream + 'a> {
        let stream = MemoryStream::from_slice_as_read(&self.data);
        Box::new(stream)
    }

    pub fn unpack_directory(&self, info: &SystemInfo) -> Result<ObjectDir, Box<dyn Error>> {
        let mut stream = self.get_stream();
        let stream = stream.as_mut();
        let mut reader = BinaryStream::from_stream_with_endian(stream, info.endian);

        // Read and verify version
        let version = reader.read_uint32()?;
        if info.version != version {
            return Err(Box::new(MiloUnpackError::UnsupportedDirectoryVersion { version }));
        }

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
            if let Some(size) = self.guess_entry_size(&mut reader)? {
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

    fn guess_entry_size<'a>(&'a self, reader: &mut BinaryStream) -> Result<Option<usize>, Box<dyn Error>> {
        let start_pos = reader.pos();
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

            if (reader.pos() as usize) >= stream_len {
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
        let entry_size = (reader.pos() - (start_pos + 4)) as usize;
        reader.seek(SeekFrom::Start(start_pos))?;

        Ok(Some(entry_size))
    }

    pub fn from_object_dir(obj_dir: &ObjectDir, info: &SystemInfo) -> Result<MiloArchive, Box<dyn Error>> {
        // Create stream
        let mut data = Vec::<u8>::new();
        let mut stream = MemoryStream::from_vector_as_read_write(&mut data);
        let mut writer = BinaryStream::from_stream(&mut stream);

        writer.write_uint32(info.version)?;
        writer.write_uint32(obj_dir.entries.len() as u32)?;

        // Write types + names
        for entry in obj_dir.entries.iter() {
            let obj_type = entry.get_type();
            let obj_name = entry.get_name();

            writer.write_prefixed_string(obj_type)?;
            writer.write_prefixed_string(obj_name)?;
        }

        if info.version == 10 {
            // TODO: Determine external dependencies or get from directory property
            writer.write_uint32(0)?;
        }

        let mut block_sizes = Vec::new();
        let mut current_size = writer.len()?;

        // Write data for entries
        for entry in obj_dir.entries.iter() {
            // Get packed entry
            let data = match entry {
                Object::Packed(packed) => &packed.data,
                _ => {
                    // TODO: Handle this better
                    continue;
                }
            };

            // Write to stream
            writer.write_bytes(&data[..])?;
            writer.write_bytes(&ADDE_PADDING)?;

            // Update block size
            current_size += data.len();


            if current_size >= MAX_BLOCK_SIZE {
                block_sizes.push(current_size);
                current_size = 0;
            }
        }

        if current_size > 0 {
            block_sizes.push(current_size);
        }

        Ok(MiloArchive {
            structure: MiloArchiveStructure::Blocked(BlockInfo {
                block_type: BlockType::TypeB,
                start_offset: 2064,
                block_sizes
            }),
            data
        })
    }

    pub fn write_to_stream(&self, stream: &mut dyn Stream) -> Result<(), Box<dyn Error>> {
        let mut writer = BinaryStream::from_stream(stream);

        match &self.structure {
            MiloArchiveStructure::Blocked(info) => {
                // Get and write magic
                let magic: u32 = match &info.block_type {
                    BlockType::TypeA => 0xCABEDEAF,
                    BlockType::TypeB => 0xCBBEDEAF,
                    BlockType::TypeC => 0xCCBEDEAF,
                    BlockType::TypeD => 0xCDBEDEAF,
                };

                // Get max uncompressed size
                let max_block_size = match info.block_sizes.iter().max() {
                    Some(max) => *max,
                    None => 0
                };

                // Write infos
                writer.write_uint32(magic)?;
                writer.write_uint32(info.start_offset)?;
                writer.write_uint32(info.block_sizes.len() as u32)?;
                writer.write_uint32(max_block_size as u32)?;

                // TODO: Implement proper seek with insertion of empty bytes
                // Write empty bytes for now
                writer.write_bytes(&vec![0u8; (info.start_offset - 16) as usize][..])?;


                // Iterate over blocks and compress data
            },
            MiloArchiveStructure::Uncompressed  => {
                // Write uncompressed data
                writer.write_bytes(&self.data[..])?;
            }
        }

        Ok(())
    }
}
