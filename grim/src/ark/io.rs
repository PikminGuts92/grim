use crate::ark::*;
use crate::io::*;
use std::collections::HashMap;
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
        let mut version = get_version(&hdr_data[0..4]);
        let mut crypt_key = None;

        if !version_is_supported(version) {
            // Decrypt hdr (use version as key)
            crypt_dtb_style(&mut hdr_data[4..], version, None);
            crypt_key = Some(version);

            version = get_version(&hdr_data[4..8]);

            // Check version compatibility again
            if !version_is_supported(version) {
                return Err(ArkReadError::ArkVersionNotSupported {
                    version
                });
            }
        }

        let mut ark = Ark {
            version,
            encryption: match crypt_key {
                Some(key) => ArkEncryption::ClassicEncryption(key),
                None => ArkEncryption::None,
            },
            path: path.to_owned(),
            ..Default::default()
        };

        let read_hdr = match ark.encryption {
            ArkEncryption::None => &hdr_data[..],
            _ => &hdr_data[4..],
        };

        ark.parse_header(read_hdr)?;
        Ok(ark)
    }

    fn parse_header(&mut self, hdr: &[u8]) -> Result<(), ArkReadError> {
        let mut stream = MemoryStream::from_slice_as_read(hdr);
        let mut reader = BinaryStream::from_stream(&mut stream);

        // Skip read version
        reader.seek(SeekFrom::Current(0))
            .map_err(|_| ArkReadError::ArkNotSupported)?;

        let part_count = reader.read_uint32()
            .map_err(|_| ArkReadError::ArkNotSupported)?;

        // TODO: Verify both counts match
        let part_size_count = reader.read_uint32()
            .map_err(|_| ArkReadError::ArkNotSupported)?;

        let mut part_sizes = vec![0u32; part_size_count as usize];

        // Read part sizes
        for p in part_sizes.iter_mut() {
            *p = reader.read_uint32()
                .map_err(|_| ArkReadError::ArkNotSupported)?;
        }

        // TODO: Verify count matches here too
        let part_name_count = reader.read_uint32()
            .map_err(|_| ArkReadError::ArkNotSupported)?;

        // Skip part file names
        for _ in 0..part_name_count {
            reader.read_prefixed_string()
                .map_err(|_| ArkReadError::ArkNotSupported)?;
        }

        // Read string blob
        let strings = parse_string_blob(&mut reader)?;

        // Read string indicies
        let string_indicies = parse_string_indices(&mut reader)?;

        Ok(())
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

fn parse_string_blob(reader: &mut BinaryStream) -> Result<HashMap<u32, String>, ArkReadError> {
    let mut strings = HashMap::new();
    let blob_size = reader.read_uint32()
        .map_err(|_| ArkReadError::ArkNotSupported)?;

    let mut offset = 0;
    let start_pos = reader.pos();

    // Read string from table
    while offset < blob_size {
        let s = reader.read_null_terminated_string()
            .map_err(|_| ArkReadError::ArkNotSupported)?;

        strings.insert(offset, s);
        offset = (reader.pos() - start_pos) as u32;
    }

    Ok(strings)
}

fn parse_string_indices(reader: &mut BinaryStream) -> Result<Vec<u32>, ArkReadError> {
    let indices_count = reader.read_uint32()
        .map_err(|_| ArkReadError::ArkNotSupported)?;

    let mut indices = vec![0; indices_count as usize];

    for ind in indices.iter_mut() {
        *ind = reader.read_uint32()
            .map_err(|_| ArkReadError::ArkNotSupported)?;
    }

    Ok(indices)
}