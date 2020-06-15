use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::texture::Bitmap;
use crate::system::{Platform, SystemInfo};
use std::error::Error;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum BitmapError {
    #[error("Unsupported texture encoding")]
    UnsupportedEncoding,
    #[error("Unsupported bitmap bpp of {bpp}")]
    UnsupportedBitmapBpp {
        bpp: u8
    },
}

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

    pub fn unpack_rgba(&self, info: &SystemInfo) -> Result<Vec<u8>, Box<dyn Error>> {
        if self.encoding != 3 || info.platform != Platform::PS2 {
            return Err(Box::new(BitmapError::UnsupportedEncoding));
        }
    
        let mut rgba = vec![0u8; calc_rgba_size(self)];
        decode_from_bitmap(self, info, &mut rgba[..])?;
        Ok(rgba)
    }
}

fn calc_rgba_size(bitmap: &Bitmap) -> usize {
    let Bitmap { width: mut w, height: mut h, mip_maps: mut mips, ..} = bitmap;
    let mut size = 0;

    loop {
        size += (w as usize) * (h as usize) * 4;

        if mips == 0 {
            break;
        }

        w >>= 1;
        h >>= 1;
        mips -= 1;
    }

    size
}

fn decode_from_bitmap(bitmap: &Bitmap, info: &SystemInfo, rgba: &mut [u8]) -> Result<(), Box<dyn Error>> {
    let Bitmap { bpp, raw_data: data, .. } = bitmap;

    if *bpp == 4 || *bpp == 8 {
        let mut palette = data[..(1 << (*bpp + 2))].to_owned(); // Takes 1024 bytes for 8bpp and 64 bytes for 4bpp
        let encoded = &data[palette.len()..];
        update_alpha_channels(&mut palette, false);

        let mut i = 0; // Image index
        let mut e = 0; // Encoded index

        if *bpp == 4 {
            // Each byte encodes two colors as palette indices
            let mut p1;
            let mut p2;

            while i < rgba.len() {
                // Palette indices
                p1 = ((encoded[e] & 0x0F) << 2) as usize;
                p2 = ((encoded[e] & 0xF0) >> 2) as usize;

                // Copy colors from palette into rgba array
                rgba[i..(i + 4)].clone_from_slice(&palette[p1..(p1 + 4)]);
                rgba[(i + 4)..(i + 8)].clone_from_slice(&palette[p2..(p2 + 4)]);

                // Increment index
                e += 1;
                i += 8; // 2 pixels
            }
        } else { // 8 bpp
            // Each byte encodes single color as palette index
            let mut p1;
            let mut enc;

            while i < rgba.len() {
                enc = encoded[e];

                // Palette index
                // Swaps bits 3 and 4 with eachother
                // Ex: 0110 1011 -> 0111 0011
                p1 = (((enc & 0b1110_0111)
                    | ((enc & 0b0000_1000) << 1)
                    | ((enc & 0b0001_0000) >> 1)) << 2) as usize;

                // Copy color from palette into rgba array
                rgba[i..(i + 4)].clone_from_slice(&palette[p1..(p1 + 4)]);

                // Increment index
                e += 1;
                i += 4; // 1 pixel
            }
        }

    } else {
        return Err(Box::new(BitmapError::UnsupportedBitmapBpp { bpp: bitmap.bpp}));
    }

    Ok(())
}

fn update_alpha_channels(data: &mut [u8], reduce: bool) {
    if reduce {
        // 8-bit -> 7-bit alpha
        for alpha in data.iter_mut().step_by(4) {
            *alpha = match *alpha {
                0xFF => 0x80,
                _ => *alpha >> 1
            }
        }
    } else {
        // 7-bit -> 8-bit alpha
        for alpha in data.iter_mut().step_by(4) {
            *alpha = match *alpha {
                0x80 ..= 0xFF => 0xFF, // It should max out at 0x80 but just in case
                _ => (*alpha & 0x7F) << 1
            }
        }
    }
}