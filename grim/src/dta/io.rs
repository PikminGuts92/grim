use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::dta::*;
use itertools::Itertools;
use thiserror::Error as ThisError;
use std::error::Error;

#[derive(Debug, ThisError)]
pub enum DtaLoadError {
    #[error("Unknown node type: {node_type:#02X}")]
    UnknownNodeType {
        node_type: u32
    },
}

impl DataArray {
    pub fn load(&mut self, stream: &mut Box<BinaryStream>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl RootData {
    pub fn save(&self, stream: &mut Box<BinaryStream>) -> Result<(), Box<dyn Error>> {
        let has_data = !self.data.is_empty();

        // Save data
        stream.write_boolean(has_data)?;
        if has_data {
            save_array(&self.data, stream, &mut 0)?;
        }

        Ok(())
    }

    pub fn load(&mut self, stream: &mut Box<BinaryStream>) -> Result<(), Box<dyn Error>> {
        // Clear data
        self.data.clear();

        // Read data
        let has_data = stream.read_boolean()?;
        if has_data {
            self.data = load_array(stream)?;
        }

        Ok(())
    }
}

fn save_array(data: &Vec<DataArray>, stream: &mut Box<BinaryStream>, id: &mut u32) -> Result<(), Box<dyn Error>> {
    stream.write_uint32(data.len() as u32)?;
    stream.write_uint32(*id)?;

    for node in data {
        save_node(node, stream, id)?;
    }

    Ok(())
}

fn load_array(stream: &mut Box<BinaryStream>) -> Result<Vec<DataArray>, Box<dyn Error>> {
    let count = stream.read_uint16()? as usize;
    let _id = stream.read_uint32()?;

    let mut nodes = Vec::new();

    for _ in 0..count {
        nodes.push(load_node(stream)?);
    }

    Ok(nodes)
}

fn save_node(data: &DataArray, stream: &mut Box<BinaryStream>, id: &mut u32) -> Result<(), Box<dyn Error>> {
    match data {
        DataArray::Integer(int) => {
            stream.write_uint32(0x00)?;
            stream.write_int32(*int)?;
        },
        DataArray::Float(f) => {
            stream.write_uint32(0x01)?;
            stream.write_float32(*f)?;
        },
        DataArray::Variable(str) => {
            stream.write_uint32(0x02)?;
            save_string(str, stream)?;
        }
        _ => {}
    };
    Ok(())
}

fn load_node(stream: &mut Box<BinaryStream>) -> Result<DataArray, Box<dyn Error>> {
    let node_type = stream.read_uint32()?;

    let node = match node_type {
        0x00 => DataArray::Integer(stream.read_int32()?),
        0x01 => DataArray::Float(stream.read_float32()?),
        0x02 => DataArray::Variable(load_string(stream)?),
        0x04 => DataArray::Object(load_string(stream)?),
        0x05 => DataArray::Symbol(load_string(stream)?),
        0x06 => {
            stream.seek(SeekFrom::Current(4))?;
            DataArray::KDataUnhandled
        },
        0x07 => DataArray::IfDef(load_string(stream)?),
        0x08 => {
            stream.seek(SeekFrom::Current(4))?;
            DataArray::Else
        },
        0x09 => {
            stream.seek(SeekFrom::Current(4))?;
            DataArray::EndIf
        },
        0x10 => DataArray::Array(load_array(stream)?),
        0x11 => DataArray::Command(load_array(stream)?),
        0x12 => DataArray::String(load_string(stream)?),
        0x13 => DataArray::Property(load_array(stream)?),
        0x20 => DataArray::Define(load_string(stream)?),
        0x21 => DataArray::Include(load_string(stream)?),
        0x22 => DataArray::Merge(load_string(stream)?),
        0x23 => DataArray::IfNDef(load_string(stream)?),
        0x24 => {
            stream.seek(SeekFrom::Current(4))?;
            DataArray::Autorun
        },
        0x25 => DataArray::Undef(load_string(stream)?),
        _ => return Err(Box::new(DtaLoadError::UnknownNodeType {
            node_type
        }))
    };

    Ok(node)
}

fn save_string(str: &DataString, stream: &mut Box<BinaryStream>) -> Result<(), Box<dyn Error>> {
    let raw_data = str.get_raw();

    stream.write_uint32(raw_data.len() as u32)?;
    stream.write_bytes(raw_data.as_slice())?;

    Ok(())
}

fn load_string(stream: &mut Box<BinaryStream>) -> Result<DataString, Box<dyn Error>> {
    let length = stream.read_uint32()? as usize;
    Ok(stream.read_bytes(length)?.into())
}