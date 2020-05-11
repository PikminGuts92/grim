use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;

#[derive(Copy, Clone, Debug)]
pub enum IOEndian {
    Little,
    Big
}

pub trait StreamReader {
    // Read integers
    fn read_int8(&mut self) -> Result<i8, std::io::Error>;
    fn read_int16(&mut self) -> Result<i16, std::io::Error>;
    fn read_int32(&mut self) -> Result<i32, std::io::Error>;

    // TODO: Read floating points

    // Read strings
    fn read_prefixed_string(&mut self) -> Result<String, Box<std::error::Error>>;

    // Read bytes
    fn read_bytes(&mut self, length: usize) -> Result<Box<[u8]>, std::io::Error>;

    // Setters
    fn set_endian(&mut self, endian: IOEndian);

    // Getters
    fn endian(&self) -> IOEndian;
    fn position(&self) -> u64;

    fn seek(&mut self, offset: u64) -> Result<(), std::io::Error>;
}

#[derive(Debug)]
pub struct FileReader {
    endian: IOEndian,
    position: u64,
    file: File
}

impl FileReader {
    pub fn new(path: &Path) -> Result<FileReader, std::io::Error> {
        let file = File::open(path)?;

        Ok(FileReader {
            endian: IOEndian::Little, // TODO: Get from optional params
            position: 0,
            file
        })
    }
}

impl StreamReader for  FileReader{
    fn read_int8(&mut self) -> Result<i8, std::io::Error> {
        let mut buffer: [u8; 1] = [0];
        self.file.read_exact(&mut buffer)?;

        match self.endian {
            IOEndian::Little => Ok(i8::from_le_bytes(buffer)),
            IOEndian::Big => Ok(i8::from_be_bytes(buffer))
        }
    }

    fn read_int16(&mut self) -> Result<i16, std::io::Error> {
        let mut buffer: [u8; 2] = [0, 0];
        self.file.read_exact(&mut buffer)?;

        match self.endian {
            IOEndian::Little => Ok(i16::from_le_bytes(buffer)),
            IOEndian::Big => Ok(i16::from_be_bytes(buffer))
        }
    }

    fn read_int32(&mut self) -> Result<i32, std::io::Error> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.file.read_exact(&mut buffer)?;

        match self.endian {
            IOEndian::Little => Ok(i32::from_le_bytes(buffer)),
            IOEndian::Big => Ok(i32::from_be_bytes(buffer))
        }
    }

    fn read_prefixed_string(&mut self) -> Result<String, Box<std::error::Error>> {
        let length = self.read_int32()?;
        let mut raw_bytes = self.read_bytes(length as usize)?;

        // TODO: Replace with better one (FromUtf8Error message is awful)
        Ok(String::from_utf8(raw_bytes.as_mut().to_vec())?)
    }

    fn read_bytes(&mut self, length: usize) -> Result<Box<[u8]>, std::io::Error> {
        let mut buffer_vec = vec![0u8; length];
        self.file.read_exact(&mut buffer_vec)?;

        Ok(buffer_vec.into_boxed_slice())
    }

    fn set_endian(&mut self, endian: IOEndian) {
        self.endian = endian
    }

    fn endian(&self) -> IOEndian {
        self.endian
    }

    fn position(&self) -> u64 {
        self.position
    }

    fn seek(&mut self, offset: u64) -> Result<(), std::io::Error> {
        self.position = self.file.seek(SeekFrom::Start(offset))?;
        Ok(())
    }
}