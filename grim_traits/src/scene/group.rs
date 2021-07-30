use crate::scene::{Anim, Draw, Trans};

pub trait Group : Anim + Draw + Trans {
    fn get_environ(&self) -> &String;
    fn get_environ_mut(&mut self) -> &mut String;
    fn set_environ(&mut self, environ: String);

    fn get_objects(&self) -> &Vec<String>;
    fn get_objects_mut(&mut self) -> &mut Vec<String>;
    fn set_objects(&mut self, objects: Vec<String>);

    fn get_lod_width(&self) -> f32;
    fn set_lod_width(&mut self, lod_width: f32);

    fn get_lod_height(&self) -> f32;
    fn set_lod_height(&mut self, lod_height: f32);
}