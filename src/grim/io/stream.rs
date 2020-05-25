use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;

#[derive(Copy, Clone, Debug)]
pub enum IOEndian {
    Little,
    Big,
}

pub trait Stream {
    // Read integers
    fn read_int8(&mut self) -> Result<i8, Box<dyn Error>>;
    fn read_int16(&mut self) -> Result<i16, Box<dyn Error>>;
    fn read_int32(&mut self) -> Result<i32, Box<dyn Error>>;

    // TODO: Read floating points

    // Read strings
    fn read_prefixed_string(&mut self) -> Result<String, Box<dyn Error>>;

    // Read bytes
    fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>, Box<dyn Error>>;

    // Setters
    fn set_endian(&mut self, endian: IOEndian);

    // Getters
    fn endian(&self) -> IOEndian;
    fn position(&self) -> u64;
    fn len(&mut self) -> Result<usize, Box<dyn Error>>;

    fn seek(&mut self, offset: u64) -> Result<(), Box<dyn Error>>;
    fn seek_until(&mut self, needle: &[u8]) -> Result<Option<u64>, Box<dyn Error>>;
}

#[derive(Debug)]
pub struct FileStream {
    endian: IOEndian,
    position: u64,
    file: File,
}

impl FileStream {
    pub fn new(path: &Path) -> Result<FileStream, std::io::Error> {
        let file = File::with_options()
            .read(true)
            .write(false)
            .create(false)
            .open(path)?;

        Ok(FileStream {
            endian: IOEndian::Little, // TODO: Get from optional params
            position: 0,
            file,
        })
    }
}

impl Stream for FileStream {
    fn read_int8(&mut self) -> Result<i8, Box<dyn Error>> {
        let mut buffer: [u8; 1] = [0];
        self.file.read_exact(&mut buffer)?;
        self.position += 1;

        match self.endian {
            IOEndian::Little => Ok(i8::from_le_bytes(buffer)),
            IOEndian::Big => Ok(i8::from_be_bytes(buffer)),
        }
    }

    fn read_int16(&mut self) -> Result<i16, Box<dyn Error>> {
        let mut buffer: [u8; 2] = [0, 0];
        self.file.read_exact(&mut buffer)?;
        self.position += 2;

        match self.endian {
            IOEndian::Little => Ok(i16::from_le_bytes(buffer)),
            IOEndian::Big => Ok(i16::from_be_bytes(buffer)),
        }
    }

    fn read_int32(&mut self) -> Result<i32, Box<dyn Error>> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.file.read_exact(&mut buffer)?;
        self.position += 4;

        match self.endian {
            IOEndian::Little => Ok(i32::from_le_bytes(buffer)),
            IOEndian::Big => Ok(i32::from_be_bytes(buffer)),
        }
    }

    fn read_prefixed_string(&mut self) -> Result<String, Box<dyn Error>> {
        let length = self.read_int32()?;
        let raw_bytes = self.read_bytes(length as usize)?;
        self.position += length as u64;

        // TODO: Replace with better one (FromUtf8Error message is awful)
        Ok(String::from_utf8(raw_bytes)?)
    }

    fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer_vec = vec![0u8; length];
        self.file.read_exact(&mut buffer_vec)?;
        self.position += length as u64;

        Ok(buffer_vec)
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

    fn len(&mut self) -> Result<usize, Box<dyn Error>> {
        let start_pos = self.position();
        let size_result = self.file.seek(SeekFrom::End(0));

        self.seek(start_pos)?;

        match size_result {
            Ok(size) => {
                Ok(size as usize)
            },
            Err(err) => {
                Err(Box::new(err))
            }
        }
    }

    fn seek(&mut self, offset: u64) -> Result<(), Box<dyn Error>> {
        self.position = self.file.seek(SeekFrom::Start(offset))?;
        Ok(())
    }

    fn seek_until(&mut self, needle: &[u8]) -> Result<Option<u64>, Box<dyn Error>> {
        seek_until(self, needle)
    }
}

#[derive(Debug)]
enum MemoryData<'a> {
    Read(&'a [u8]),
    //ReadWrite(Vec<u8>), // TODO: Implement read/write stream
}

#[derive(Debug)]
pub struct MemoryStream<'a> {
    endian: IOEndian,
    position: u64,
    data: MemoryData<'a>
}

impl<'a> MemoryStream<'a> {
    /*pub fn new() -> MemoryStream<'a> {
        MemoryStream {
            endian: IOEndian::Little,
            position: 0,
            data: MemoryData::ReadWrite(Vec::new())
        }
    }*/

    /*pub fn from_vector_as_read_write(data: &'a mut Vec<u8>) -> MemoryStream<'a> {
        MemoryStream {
            endian: IOEndian::Little,
            position: 0,
            data: MemoryData::ReadWrite(data.to_vec())
        }
    }*/

    pub fn from_slice_as_read(data: &[u8]) -> MemoryStream {
        MemoryStream {
            endian: IOEndian::Little,
            position: 0,
            data: MemoryData::Read(data)
        }
    }

    fn get_slice(&self, pos: u64, size: usize) -> &'a [u8] {
        let pos = pos as usize;

        match self.data {
            MemoryData::Read(data) => {
                &data[pos..(pos + size)]
            },
            /*MemoryData::ReadWrite(vec) => {
                &vec[pos as usize..size]
            }*/
        }
    }
}

impl<'a> Stream for MemoryStream<'a> {
    fn read_int8(&mut self) -> Result<i8, Box<dyn Error>> {
        let mut buffer: [u8; 1] = [0];
        let data = self.get_slice(self.position, 1);

        buffer.clone_from_slice(data);
        self.position += 1;

        match self.endian {
            IOEndian::Little => Ok(i8::from_le_bytes(buffer)),
            IOEndian::Big => Ok(i8::from_be_bytes(buffer)),
        }
    }

    fn read_int16(&mut self) -> Result<i16, Box<dyn Error>> {
        let mut buffer: [u8; 2] = [0, 0];
        let data = self.get_slice(self.position, 2);

        buffer.clone_from_slice(data);
        self.position += 2;

        match self.endian {
            IOEndian::Little => Ok(i16::from_le_bytes(buffer)),
            IOEndian::Big => Ok(i16::from_be_bytes(buffer)),
        }
    }

    fn read_int32(&mut self) -> Result<i32, Box<dyn Error>> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        let data = self.get_slice(self.position, 4);

        buffer.clone_from_slice(data);
        self.position += 4;

        match self.endian {
            IOEndian::Little => Ok(i32::from_le_bytes(buffer)),
            IOEndian::Big => Ok(i32::from_be_bytes(buffer)),
        }
    }

    fn read_prefixed_string(&mut self) -> Result<String, Box<dyn Error>> {
        let length = self.read_int32()?;
        let raw_bytes = self.read_bytes(length as usize)?;

        // TODO: Replace with better one (FromUtf8Error message is awful)
        Ok(String::from_utf8(raw_bytes)?)
    }

    fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer_vec = vec![0u8; length];
        let data = self.get_slice(self.position, length);

        buffer_vec.clone_from_slice(data);
        self.position += length as u64;

        Ok(buffer_vec)
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

    fn len(&mut self) -> Result<usize, Box<dyn Error>> {
        match self.data {
            MemoryData::Read(data) => {
                Ok(data.len())
            }
        }
    }

    fn seek(&mut self, offset: u64) -> Result<(), Box<dyn Error>> {
        self.position = offset;
        Ok(())
    }

    fn seek_until(&mut self, needle: &[u8]) -> Result<Option<u64>, Box<dyn Error>> {
        seek_until(self, needle)
    }
}

fn seek_until<T>(stream: &mut T, needle: &[u8]) -> Result<Option<u64>, Box<dyn Error>> where T: Stream {
    let start_pos = stream.position();
    let stream_len = stream.len()?;

    let needle_len = needle.len();
    let search_limit = stream_len - needle_len;

    let mut haystack: Vec<u8>;
    let mut next_pos: u64;
    while stream.position() <= search_limit as u64 {
        haystack = stream.read_bytes(needle_len)?;

        if haystack == needle {
            // Data found
            next_pos = stream.position() - (needle_len as u64);
            stream.seek(next_pos)?;

            return Ok(Some(next_pos - start_pos));
        } else {
            next_pos = stream.position() - ((needle_len as u64) - 1);
            stream.seek(next_pos)?;
        }
    }

    Ok(None)
}
