use crate::ark::*;
use crate::io::*;
use std::io::BufWriter;
use std::path::Path;

const MAX_HDR_SIZE: u64 = 20 * 0x100000; // 20MB

impl Ark {
    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Ark, ArkReadError> {
        let path = path.as_ref();

        let hdr_size = get_file_size(path);
        if hdr_size > MAX_HDR_SIZE {
            return Err(ArkReadError::HdrTooBig);
        }

        let mut hdr_data = read_to_bytes(path);
        let version = get_version(&hdr_data[0..4]);

        if !version_is_supported(version) {
            // Decrypt hdr (use version as key)
            crypt_dtb_style(&mut hdr_data[4..], version, None);
        }

        let stream = FileStream::from_path_as_read_open(path)
            .map_err(|e| ArkReadError::CantOpenArk)?;

        Ok(Ark::default())
    }
}

fn get_version(data: &[u8]) -> i32 {
    let mut buffer = [0u8; 4];
    buffer.copy_from_slice(&data[0..4]);

    i32::from_le_bytes(buffer)
}

fn version_is_supported(version: i32) -> bool {
    match version {
        5 => true,
        _ => false
    }
}