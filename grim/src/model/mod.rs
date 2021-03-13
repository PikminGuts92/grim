mod mesh;

pub use self::mesh::*;

#[derive(Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,

    pub nx: f32,
    pub ny: f32,
    pub nz: f32,

    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,

    pub u: f32,
    pub v: f32,
}