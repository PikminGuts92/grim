mod io;
pub use io::*;
use crate::texture::Bitmap;

#[derive(Debug)]
pub struct Tex {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub bpp: u32,

    pub index_f: f32,
    pub index: i32,

    pub ext_path: String,
    pub use_ext_path: bool,

    pub bitmap: Option<Bitmap>
}

impl Tex {
    pub fn new() -> Tex {
        Tex {
            name: String::from(""),
            width: 0,
            height: 0,
            bpp: 0,

            index_f: -13.0,
            index: 1,

            ext_path: String::from(""),
            use_ext_path: false,

            bitmap: None
        }
    }
}