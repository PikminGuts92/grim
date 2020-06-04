use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
pub use std::io::SeekFrom;
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

    // Write integers
    fn write_int8(&mut self, value: i8) -> Result<(), Box<dyn Error>>;

    // Setters
    fn set_endian(&mut self, endian: IOEndian);

    // Getters
    fn endian(&self) -> IOEndian;
    fn position(&self) -> u64;
    fn can_write(&self) -> bool;
    fn len(&mut self) -> Result<usize, Box<dyn Error>>;

    fn seek(&mut self, pos: SeekFrom) -> Result<(), Box<dyn Error>>;
    fn seek_until(&mut self, needle: &[u8]) -> Result<Option<usize>, Box<dyn Error>>;
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

    fn write_int8(&mut self, value: i8) -> Result<(), Box<dyn Error>> {
        panic!("Not implmented yet") // TODO: Create custom error when can't write
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

    fn can_write(&self) -> bool {
        false // TODO: Add support for writing to file
    }

    fn len(&mut self) -> Result<usize, Box<dyn Error>> {
        let start_pos = self.position();
        let size_result = self.file.seek(SeekFrom::End(0));

        self.seek(SeekFrom::Start(start_pos))?;

        match size_result {
            Ok(size) => {
                Ok(size as usize)
            },
            Err(err) => {
                Err(Box::new(err))
            }
        }
    }

    fn seek(&mut self, pos: SeekFrom) -> Result<(), Box<dyn Error>> {
        self.position = self.file.seek(pos)?;
        Ok(())
    }

    fn seek_until(&mut self, needle: &[u8]) -> Result<Option<usize>, Box<dyn Error>> {
        seek_until(self, needle)
    }
}

#[derive(Debug)]
enum MemoryData<'a> {
    Read(&'a [u8]),
    ReadWrite(&'a mut Vec<u8>), // TODO: Implement read/write stream
    ReadWriteOwned(Vec<u8>),
}

#[derive(Debug)]
pub struct MemoryStream<'a> {
    endian: IOEndian,
    position: u64,
    data: MemoryData<'a>
}


impl<'a> MemoryStream<'a> {
    pub fn new() -> MemoryStream<'a> {
        MemoryStream {
            endian: IOEndian::Little,
            position: 0,
            data: MemoryData::ReadWriteOwned(Vec::new())
        }
    }

    pub fn from_vector_as_read_write(data: &'a mut Vec<u8>) -> MemoryStream<'a> {
        MemoryStream {
            endian: IOEndian::Little,
            position: 0,
            data: MemoryData::ReadWrite(data)
        }
    }

    pub fn from_slice_as_read(data: &[u8]) -> MemoryStream {
        MemoryStream {
            endian: IOEndian::Little,
            position: 0,
            data: MemoryData::Read(data)
        }
    }

    fn get_slice(&'a self, pos: u64, size: usize) -> &'a [u8] {
        let pos = pos as usize;

        match &self.data {
            MemoryData::Read(data) => {
                &(*data)[pos..(pos + size)]
            },
            MemoryData::ReadWrite(vec) => {
                &(*vec)[pos..(pos + size)]
            },
            MemoryData::ReadWriteOwned(vec) => {
                &vec[pos..(pos + size)]
            }
        }
    }
}

impl<'a> Stream for MemoryStream<'a> {
    fn read_int8(&mut self) -> Result<i8, Box<dyn Error>> {
        let mut buffer: [u8; 1] = [0];
        let data = self.get_slice(self.position, buffer.len());

        buffer.clone_from_slice(data);
        self.position += buffer.len() as u64;

        match self.endian {
            IOEndian::Little => Ok(i8::from_le_bytes(buffer)),
            IOEndian::Big => Ok(i8::from_be_bytes(buffer)),
        }
    }

    fn read_int16(&mut self) -> Result<i16, Box<dyn Error>> {
        let mut buffer: [u8; 2] = [0, 0];
        let data = self.get_slice(self.position, buffer.len());

        buffer.clone_from_slice(data);
        self.position += buffer.len() as u64;

        match self.endian {
            IOEndian::Little => Ok(i16::from_le_bytes(buffer)),
            IOEndian::Big => Ok(i16::from_be_bytes(buffer)),
        }
    }

    fn read_int32(&mut self) -> Result<i32, Box<dyn Error>> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        let data = self.get_slice(self.position, buffer.len());

        buffer.clone_from_slice(data);
        self.position += buffer.len() as u64;

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

    fn write_int8(&mut self, value: i8) -> Result<(), Box<dyn Error>> {
        if !self.can_write() {
            panic!("Not implmented yet") // TODO: Create custom error when can't write
        }

        /*let &mut vec_data = match &mut self.data {
            /*MemoryData::Read(data) => {
                Vec::<u8>::new() // Throw error (but it shouldn't reach this part)
            },
            MemoryData::ReadWrite(vec) => {
                // Reference to mutable pointer?
                //let mut v = (*vec).as_mut();
                //let mut v = vec.as_mut();

                let mut v = *vec;

                v
            },*/
            MemoryData::ReadWriteOwned(vec) => {
                vec
            }
        };*/

        // TODO: Switch to match expression
        let &mut vec_data;

        if let MemoryData::ReadWriteOwned(vec) = &mut self.data {
            vec_data = vec;
        } else if let MemoryData::ReadWrite(vec) = &mut self.data {
            vec_data = vec;
        } else {
            panic!("Not implmented yet") // Throw error (but it shouldn't reach this part)
        }

        let data = match self.endian {
            IOEndian::Little => value.to_le_bytes(),
            IOEndian::Big => value.to_be_bytes(),
        };
        let data_len = data.len();

        if self.position == vec_data.len() as u64 {
            vec_data.extend_from_slice(&data);
        } else {
            let pos = self.position as usize;
            vec_data[pos..(pos + data_len)].clone_from_slice(&data);
        }

        self.position += data_len as u64;
        Ok(())
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

    fn can_write(&self) -> bool {
        match &self.data {
            MemoryData::Read(_) => false,
            MemoryData::ReadWrite(_) => true,
            MemoryData::ReadWriteOwned(_) => true
        }
    }

    fn len(&mut self) -> Result<usize, Box<dyn Error>> {
        match &self.data {
            MemoryData::Read(data) => {
                Ok((*data).len())
            },
            MemoryData::ReadWrite(vec) => {
                Ok((*vec).len())
            },
            MemoryData::ReadWriteOwned(vec) => {
                Ok(vec.len())
            }
        }
    }

    fn seek(&mut self, pos: SeekFrom) -> Result<(), Box<dyn Error>> {
        self.position = match pos {
            SeekFrom::Start(rel_str) => rel_str,
            SeekFrom::End(rel_end) => ((self.len()? as i64) + rel_end) as u64,
            SeekFrom::Current(rel_cur) => ((self.position as i64) + rel_cur) as u64,
        };
        Ok(())
    }

    fn seek_until(&mut self, needle: &[u8]) -> Result<Option<usize>, Box<dyn Error>> {
        seek_until(self, needle)
    }
}

fn seek_until<T>(stream: &mut T, needle: &[u8]) -> Result<Option<usize>, Box<dyn Error>> where T: Stream {
    let start_pos = stream.position();
    let stream_len = stream.len()?;

    let needle_len = needle.len();
    let search_limit = stream_len - needle_len;

    let mut haystack: Vec<u8>;
    while stream.position() <= search_limit as u64 {
        haystack = stream.read_bytes(needle_len)?;

        if haystack == needle {
            // Data found
            stream.seek(SeekFrom::Current(-(needle_len as i64)))?;

            return Ok(Some((stream.position() - start_pos) as usize));
        } else {
            // Still searching
            stream.seek(SeekFrom::Current(-((needle_len - 1) as i64)))?;
        }
    }

    Ok(None)
}
