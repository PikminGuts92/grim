use super::{MiloObject, Sphere};

pub trait Draw : MiloObject {
    fn get_showing(&self) -> bool;
    fn set_showing(&mut self, showing: bool);

    fn get_bounding(&self) -> &Sphere;
    fn get_bounding_mut(&mut self) -> &mut Sphere;
    fn set_bounding(&mut self, sphere: Sphere);

    fn get_draw_order(&self) -> f32;
    fn set_draw_order(&mut self, draw_order: f32);
}