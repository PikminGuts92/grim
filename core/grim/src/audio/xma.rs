use bitstream_io::{BigEndian, BitRead, BitReader, Endianness};
use std::io::{Cursor, Read, Result, SeekFrom};

const XMA_BYTES_PER_PACKET: usize = 2048;
const XMA_SAMPLES_PER_FRAME: usize = 512;
const XMA_SAMPLES_PER_SUBFRAME: usize = 128;

#[derive(Debug)]
struct PacketInfo {
    pub frame_count: u8,
    pub frame_offset_in_bits: u16,
    pub metadata: u8,
    pub skip_count: u8
}

impl PacketInfo {
    /*pub fn from_u32(n: u32) -> Self {
        Self {
            frame_count: (n >> 26) as u8,
            frame_offset_in_bits
        }
    }*/

    pub fn from_reader<R: Read, E: Endianness>(reader: &mut BitReader<R, E>) -> Result<Self> {
        // 32 bytes
        let frame_count = reader.read::<u8>(6)?;
        let frame_offset_in_bits = reader.read::<u16>(15)?;
        let metadata = reader.read::<u8>(3)?;
        let skip_count = reader.read::<u8>(8)?;

        Ok(Self {
            frame_count,
            frame_offset_in_bits,
            metadata,
            skip_count
        })
    }
}

// TODO: Probably put inside struct as method
pub fn read_frame_length<R: Read, E: Endianness>(reader: &mut BitReader<R, E>) -> Result<u16> {
    reader.read::<u16>(15)
}

pub fn decode_xma_packets(packets: &[u8], sample_count: i32) -> Result<Vec<i16>> {
    let mut reader = BitReader::endian(Cursor::new(packets), BigEndian);

    let packet = PacketInfo::from_reader(&mut reader)?;
    println!("{packet:#?}");

    let frame_length = read_frame_length(&mut reader)? - 15;
    println!("Frame length: {frame_length}");

    reader.seek_bits(SeekFrom::Current(frame_length as i64))?;

    let frame_length = read_frame_length(&mut reader)? - 15;
    println!("Frame length: {frame_length}");

    todo!()
}