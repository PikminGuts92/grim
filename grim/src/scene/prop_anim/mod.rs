mod io;
pub use io::*;
use crate::dta::DataArray;
use grim_macros::*;
use grim_traits::scene::*;

// TODO: Move events to shared file
#[derive(Debug, Default)]
pub struct AnimEventFloat {
    pub value: f32,
    pub pos: f32,
}

#[derive(Debug, Default)]
pub struct AnimEventColor {
    pub value: Color4,
    pub pos: f32,
}

#[derive(Debug, Default)]
pub struct AnimEventObject {
    pub text1: String,
    pub text2: String,
    pub pos: f32,
}

#[derive(Debug, Default)]
pub struct AnimEventBool {
    pub value: bool,
    pub pos: f32,
}

#[derive(Debug, Default)]
pub struct AnimEventQuat {
    pub value: Quat,
    pub pos: f32,
}

#[derive(Debug, Default)]
pub struct AnimEventVector3 {
    pub value: Vector3,
    pub pos: f32,
}

#[derive(Debug, Default)]
pub struct AnimEventSymbol {
    pub text: String,
    pub pos: f32,
}

#[derive(Debug)]
pub enum PropKeysEvents {
    Float(Vec<AnimEventFloat>),
    Color(Vec<AnimEventColor>),
    Object(Vec<AnimEventObject>),
    Bool(Vec<AnimEventBool>),
    Quat(Vec<AnimEventQuat>),
    Vector3(Vec<AnimEventVector3>),
    Symbol(Vec<AnimEventSymbol>),
}

impl Default for PropKeysEvents {
    fn default() -> PropKeysEvents {
        PropKeysEvents::Symbol(Default::default())
    }
}

#[derive(Debug, Default)]
pub struct PropKeys {
    pub target: String,
    pub property: DataArray,

    pub interpolation: u32, // Some flags?
    pub interp_handler: String,

    pub unknown_enum: u32,
    pub events: PropKeysEvents,
}

#[milo]
#[milo_super(Anim)]
#[derive(Debug)]
pub struct PropAnim {
    pub unknown_toggle: bool, // Used in GDRB
    pub keys: Vec<PropKeys>,
}

impl Default for PropAnim {
    fn default() -> PropAnim {
        PropAnim {
            // Base object
            name: String::default(),
            type2: String::default(),
            note: String::default(),

            // Anim object
            anim_objects: Vec::new(),
            frame: 0.0,
            rate: AnimRate::default(),

            unknown_toggle: false,
            keys: Vec::new(),
        }
    }
}