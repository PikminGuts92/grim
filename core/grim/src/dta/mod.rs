mod errors;
mod io;
mod parser;

pub use errors::*;
pub use io::*;
use parser::*;

#[derive(Debug, Default, PartialEq)]
pub struct DataString {
    data: Vec<u8>,
}

impl DataString {
    pub fn from_vec(data: Vec<u8>) -> DataString {
        DataString {
            data,
        }
    }

    pub fn from_string<S: Into<String>>(str: S) -> DataString {
        DataString {
            data: str.into().into_bytes(),
        }
    }

    pub fn as_utf8(&self) -> Option<&str> {
        std::str::from_utf8(&self.data).ok()
    }

    pub fn get_raw(&self) -> &Vec<u8> {
        &self.data
    }
}

impl From<Vec<u8>> for DataString {
    fn from(data: Vec<u8>) -> DataString {
        DataString::from_vec(data)
    }
}

#[derive(Debug, PartialEq)]
pub enum DataArray {
    Integer(i32),
    Float(f32),
    Variable(DataString),
    //Func() ???
    Object(DataString),
    Symbol(DataString),
    KDataUnhandled,
    IfDef(DataString),
    Else,
    EndIf,
    Array(Vec<DataArray>),
    Command(Vec<DataArray>),
    String(DataString),
    Property(Vec<DataArray>),
    Define(DataString),
    Include(DataString),
    Merge(DataString),
    IfNDef(DataString),
    Autorun,
    Undef(DataString),
}

impl Default for DataArray {
    fn default() -> DataArray {
        DataArray::Integer(0)
    }
}

impl DataArray {
    pub(crate) fn get_enum_value(&self) -> u32 {
        match self {
            DataArray::Integer(_)     => 0x00,
            DataArray::Float(_)       => 0x01,
            DataArray::Variable(_)    => 0x02,
            //DataArray::Func(_)        => 0x03,
            DataArray::Object(_)      => 0x04,
            DataArray::Symbol(_)      => 0x05,
            DataArray::KDataUnhandled => 0x06,
            DataArray::IfDef(_)       => 0x07,
            DataArray::Else           => 0x08,
            DataArray::EndIf          => 0x09,
            DataArray::Array(_)       => 0x10,
            DataArray::Command(_)     => 0x11,
            DataArray::String(_)      => 0x12,
            DataArray::Property(_)    => 0x13,
            DataArray::Define(_)      => 0x20,
            DataArray::Include(_)     => 0x21,
            DataArray::Merge(_)       => 0x22,
            DataArray::IfNDef(_)      => 0x23,
            DataArray::Autorun        => 0x24,
            DataArray::Undef(_)       => 0x25,
        }
    }

    pub fn find_value_for_symbol<'a, 'b, T: Into<&'b DataString>>(&'a self, symbol: T) -> Option<&[DataArray]> {
        let DataArray::Array(array) = self else {
            return None;
        };

        let symbol = symbol.into();

        match array.split_first() {
            // Found element
            Some((DataArray::Symbol(s) | DataArray::String(s), elements)) if s.eq(symbol) => {
                // Return remaining elements
                return Some(elements);
            },
            // Keep searching
            _ => {}
        }

        // Recursively search
        for val in array {
            let result = val.find_value_for_symbol(symbol);

            if result.is_some() {
                return result;
            }
        }

        None
    }

    pub fn as_float(&self) -> Option<f32> {
        match self {
            DataArray::Float(f) => Some(*f),
            DataArray::Integer(i) => Some(*i as f32),
            _ => None
        }
    }

    pub fn as_integer(&self) -> Option<i32> {
        match self {
            DataArray::Integer(i) => Some(*i),
            DataArray::Float(f) => Some(*f as i32),
            _ => None
        }
    }
}

#[derive(Debug, Default)]
pub struct RootData {
    pub data: Vec<DataArray>,
}

impl RootData {
    pub fn new() -> RootData {
        RootData::default()
    }
}