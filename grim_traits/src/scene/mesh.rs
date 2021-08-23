use super::{Color4, Draw, Matrix, MiloObject, Trans, Vector4};

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum Volume {
    kVolumeEmpty,
    kVolumeTriangles,
    kVolumeBSP,
    kVolumeBox,
}

impl Default for Volume {
    fn default() -> Volume {
        Volume::kVolumeEmpty
    }
}

impl From<u32> for Volume {
    fn from(num: u32) -> Volume {
        match num {
            0 => Volume::kVolumeEmpty,
            1 => Volume::kVolumeTriangles,
            2 => Volume::kVolumeBSP,
            3 => Volume::kVolumeBox,
            // Default
            _ => Volume::kVolumeEmpty,
        }
    }
}

#[derive(Default)]
pub struct BoneTrans {
    pub name: String,
    pub trans: Matrix,
}

#[derive(Default)]
pub struct UV {
    pub u: f32,
    pub v: f32,
}

#[derive(Default)]
pub struct Vertex {
    pub pos: Vector4,
    pub normals: Vector4,
    pub color: Color4,
    pub uv: UV,
}

pub trait Mesh : Draw + MiloObject + Trans {
    fn get_mat(&self) -> &String;
    fn get_mat_mut(&mut self) -> &mut String;
    fn set_mat(&mut self, mat: String);

    fn get_geom_owner(&self) -> &String;
    fn get_geom_owner_mut(&mut self) -> &mut String;
    fn set_geom_owner(&mut self, geom_owner: String);

    fn get_mutable(&self) -> u32;
    fn set_mutable(&mut self, mutable: u32);

    fn get_volume(&self) -> &Volume;
    fn get_volume_mut(&mut self) -> &mut Volume;
    fn set_volume(&mut self, volume: Volume);

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

    fn get_keep_mesh_data(&self) -> bool;
    fn set_keep_mesh_data(&mut self, keep_mesh_data: bool);
}