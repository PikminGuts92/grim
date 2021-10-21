mod io;

pub use io::*;

pub struct DataString {
    data: Vec<u8>,
}

impl DataString {
    pub fn from_vec(data: Vec<u8>) -> DataString {
        DataString {
            data,
        }
    }

    pub fn from_string(str: String) -> DataString {
        DataString {
            data: str.as_bytes().to_owned(),
        }
    }

    pub fn as_utf8(&self) -> String {
        String::from_utf8_lossy(&self.data).into_owned()
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

pub enum DataArray {
    Integer(i32),
    Float(f32),
    Variable(DataString),
    Object(DataString),
    Symbol(DataString),
    KDataUnhandled,
    IfDef(DataString),
    Else,
    EndIf,
    String(DataString),
    Define(DataString),
    Include(DataString),
    Merge(DataString),
    IfNDef(DataString),
    Autorun,
    Undef(DataString),
    Array(Vec<DataArray>),
    Command(Vec<DataArray>),
    Property(Vec<DataArray>),
}

#[derive(Default)]
pub struct RootData {
    pub data: Vec<DataArray>,
}

impl RootData {
    pub fn new() -> RootData {
        RootData::default()
    }
}