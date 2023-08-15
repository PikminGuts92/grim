mod io;

use super::AnimEvent;
use grim_macros::*;
use grim_traits::scene::*;
pub use io::*;

#[milo(Anim)]
pub struct Morph {
    pub poses: Vec<MorphPose>,
    pub normals: bool,
    pub spline: bool,
    pub intensity: f32,
}

#[derive(Default)]
pub struct MorphPose {
    pub events: Vec<AnimEvent<f32>>
}

impl Default for Morph { // RndMorph
    fn default() -> Morph {
        Morph {
            // Base object
            name: String::default(),
            type2: String::default(),
            note: String::default(),

            // Anim object
            anim_objects: Vec::new(),
            frame: 0.0,
            rate: AnimRate::default(),

            // Morph object
            poses: Vec::new(),
            normals: true,
            spline: true,
            intensity: 1.0
        }
    }
}
