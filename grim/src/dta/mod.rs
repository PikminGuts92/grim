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
}

pub enum DataArray {
    Integer(u32),
    Float(f32),
    Variable(DataString),
    Object(DataString),
    Symbol(DataString),
    KDataUnhandled,
    IfDef(DataString),
    Else(DataString),
    EndIf(DataString),
    String(DataString),
    Define(DataString),
    Include(DataString),
    Merge(DataString),
    IfNDef(DataString),
    Autorun(DataString),
    Undef(DataString),
    Array(Vec<DataArray>),
    Command(Vec<DataArray>),
    Property(Vec<DataArray>),
}