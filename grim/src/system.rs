use crate::io::{IOEndian, MiloArchive};
use std::path::{Path, PathBuf};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Platform {
    PS2,
    PS3,
    Wii,
    X360,
}

#[derive(Copy, Clone, Debug)]
pub struct SystemInfo {
    pub version: u32,
    pub platform: Platform,
    pub endian: IOEndian, 
}

impl SystemInfo {
    pub fn guess_system_info(milo: &MiloArchive, milo_path: &Path) -> SystemInfo {
        let platform = match milo_path.extension() {
            Some(ext) => match ext.to_str() {
                Some("milo_ps2") => Platform::PS2,
                Some("milo_ps3") => Platform::PS3,
                Some("milo_wii") => Platform::Wii,
                Some("milo_xbox") => Platform::X360,
                Some("rnd_ps2") => Platform::PS2,
                Some("gh") => Platform::PS2,
                _ => Platform::X360,
            },
            None => Platform::X360,
        };

        // Default: Big endian - RB1
        let mut endian = IOEndian::Big;
        let mut version = 25;

        if platform == Platform::X360 {
            if let Some((end, ver)) = milo.guess_endian_version() {
                endian = end;
                version = ver;
            }
        } else if platform == Platform::PS2 {
            endian = IOEndian::Little;
            version = milo.get_version(endian).unwrap_or(24); // GH2
        }

        SystemInfo {
            version,
            platform,
            endian,
        }
    }

    pub fn is_next_gen(&self) -> bool {
        match self.platform {
            Platform::PS3 | Platform::X360 => true,
            _ => false,
        }
    }
}
