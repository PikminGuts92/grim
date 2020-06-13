use crate::io::IOEndian;

#[derive(Copy, Clone, Debug)]
pub enum Platform {
    PS2,
    X360,
}

#[derive(Copy, Clone, Debug)]
pub struct SystemInfo {
    pub version: u32,
    pub platform: Platform,
    pub endian: IOEndian, 
}