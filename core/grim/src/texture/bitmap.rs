#[derive(Debug)]
pub struct Bitmap {
    pub bpp: u8,
    pub encoding: u32,
    pub mip_maps: u8,

    pub width: u16,
    pub height: u16,
    pub bpl: u16,

    pub raw_data: Vec<u8>,
}

impl Bitmap {
    pub fn new() -> Bitmap {
        Bitmap {
            bpp: 8,
            encoding: 3,
            mip_maps: 0,

            width: 0,
            height: 0,
            bpl: 0,

            raw_data: Vec::new()
        }
    }
}