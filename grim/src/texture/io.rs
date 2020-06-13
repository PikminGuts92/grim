use std::error::Error;
use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::texture::Bitmap;
use crate::system::SystemInfo;

impl Bitmap {
    pub fn from_stream(stream: &mut dyn Stream, info: &SystemInfo) -> Result<Bitmap, Box<dyn Error>> {
        let mut bitmap = Bitmap::new();
        let mut reader = BinaryStream::from_stream_with_endian(stream, info.endian);

        let byte_1 = reader.read_uint8()?; // TODO: Verify always 1

        bitmap.bpp = reader.read_uint8()?;
        bitmap.encoding = reader.read_uint32()?;
        bitmap.mip_maps = reader.read_uint8()?;

        bitmap.width = reader.read_uint16()?;
        bitmap.height = reader.read_uint16()?;
        bitmap.bpl = reader.read_uint16()?;

        reader.seek(SeekFrom::Current(19))?; // Skip empty bytes

        // TODO: Calculate expected data size and verify against actual
        let current_pos = reader.pos();
        let stream_len = reader.len()?;
        let rem_bytes = stream_len - current_pos as usize;

        bitmap.raw_data = reader.read_bytes(rem_bytes)?;
        Ok(bitmap)
    }
}
