use crate::{Color4, Matrix, Vector4};
use crate::scene::{Draw, Trans};

pub struct BoneTrans {
    pub name: String,
    pub trans: Matrix,
}

pub struct UV {
    pub u: f32,
    pub v: f32,
}

pub struct Vertex {
    pub pos: Vector4,
    pub normals: Vector4,
    pub color: Color4,
    pub uv: UV,
}

pub trait Mesh : Draw + Trans {
    fn get_material(&self) -> &String;
    fn get_material_mut(&mut self) -> &mut String;
    fn set_material(&mut self, mat: String);

    fn get_mesh_name(&self) -> &String;
    fn get_mesh_name_mut(&mut self) -> &mut String;
    fn set_mesh_name(&mut self, mesh: String);

    fn get_vertices(&self) -> &Vec<Vertex>;
    fn get_vertices_mut(&mut self) -> &mut Vec<Vertex>;
    fn set_vertices(&mut self, vertices: Vec<Vertex>);

    fn get_faces(&self) -> &Vec<[u16; 3]>;
    fn get_faces_mut(&mut self) -> &mut Vec<[u16; 3]>;
    fn set_faces(&mut self, faces: Vec<[u16; 3]>);

    fn get_face_groups(&self) -> &Vec<u8>;
    fn get_face_groups_mut(&mut self) -> &mut Vec<u8>;
    fn set_face_groups(&mut self, face_groups: Vec<u8>);

    fn get_bones(&self) -> &Vec<BoneTrans>;
    fn get_bones_mut(&mut self) -> &mut Vec<BoneTrans>;
    fn set_bones(&mut self, bones: Vec<BoneTrans>);
}