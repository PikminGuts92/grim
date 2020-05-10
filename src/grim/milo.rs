use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;

pub trait StreamReader {
    fn read_int8(&mut self) -> Result<i8, std::io::Error>;
    fn read_int16(&mut self) -> Result<i16, std::io::Error>;
    fn read_int32(&mut self) -> Result<i32, std::io::Error>;
    fn position(&self) -> u64;
    fn seek(&mut self, offset: u64) -> Result<(), std::io::Error>;
}

#[derive(Debug)]
pub struct FileReader {
    position: u64,
    file: File
}

impl FileReader {
    pub fn new(path: &Path) -> Result<FileReader, std::io::Error> {
        let file = File::open(path)?;

        Ok(FileReader {
            position: 0,
            file
        })
    }
}

impl StreamReader for  FileReader{
    fn read_int8(&mut self) -> Result<i8, std::io::Error> {
        //self.file.re

        Ok(0)
    }

    fn read_int16(&mut self) -> Result<i16, std::io::Error> {
        Ok(0)
    }

    fn read_int32(&mut self) -> Result<i32, std::io::Error> {
        Ok(0)
    }

    fn position(&self) -> u64 {
        self.position
    }

    fn seek(&mut self, offset: u64) -> Result<(), std::io::Error> {
        self.position = self.file.seek(SeekFrom::Start(offset))?;
        Ok(())
    }
}