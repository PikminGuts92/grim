mod io;

use crate::texture::Bitmap;
use grim_macros::*;
use grim_traits::scene::*;
pub use io::*;

#[milo]
pub struct CubeTexObject {
    pub some_num_1: u32,
    pub some_num_2: u32,

    pub right_ext_path: String,
    pub left_ext_path: String,
    pub top_ext_path: String,
    pub bottom_ext_path: String,
    pub front_ext_path: String,
    pub back_ext_path: String,

    pub some_bool: bool,

    pub right: Option<Bitmap>,
    pub left: Option<Bitmap>,
    pub top: Option<Bitmap>,
    pub bottom: Option<Bitmap>,
    pub front: Option<Bitmap>,
    pub back: Option<Bitmap>,
}

impl Default for CubeTexObject {
    fn default() -> CubeTexObject {
        CubeTexObject {
            // Base object
            name: String::default(),
            type2: String::default(),
            note: String::default(),

            // CubeTex object
            some_num_1: 64,
            some_num_2: 4,

            right_ext_path: String::default(),
            left_ext_path: String::default(),
            top_ext_path: String::default(),
            bottom_ext_path: String::default(),
            front_ext_path: String::default(),
            back_ext_path: String::default(),

            some_bool: true,

            right: None,
            left: None,
            top: None,
            bottom: None,
            front: None,
            back: None,
        }
    }
}