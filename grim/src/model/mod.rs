mod group;
mod mat;
mod mesh;
mod tex;

pub use self::group::*;
pub use self::mat::*;
pub use self::mesh::*;
pub use self::tex::*;

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

#[derive(Debug)]
pub struct Face {
    pub v1: u16,
    pub v2: u16,
    pub v3: u16,
}

#[derive(Debug)]
pub struct AssetManagager {
    groups: Vec<Group>,
    meshes: Vec<MiloMesh>,
    materials: Vec<Mat>,
    textures: Vec<Tex>,
}

impl AssetManagager {
    pub fn new() -> AssetManagager {
        AssetManagager {
            groups: Vec::new(),
            meshes: Vec::new(),
            materials: Vec::new(),
            textures: Vec::new(),
        }
    }

    pub fn get_group(&self, name: &str) -> Option<&Group> {
        self.groups.iter().find(|g| g.name.eq(name))
    }

    pub fn get_mesh(&self, name: &str) -> Option<&MiloMesh> {
        self.meshes.iter().find(|m| m.name.eq(name))
    }

    pub fn get_material(&self, name: &str) -> Option<&Mat> {
        self.materials.iter().find(|m| m.name.eq(name))
    }

    pub fn get_texture(&self, name: &str) -> Option<&Tex> {
        self.textures.iter().find(|t| t.name.eq(name))
    }

    pub fn add_group(&mut self, group: Group) {
        self.groups.push(group);
    }

    pub fn add_mesh(&mut self, mesh: MiloMesh) {
        self.meshes.push(mesh);
    }

    pub fn add_material(&mut self, mat: Mat) {
        self.materials.push(mat);
    }

    pub fn add_tex(&mut self, tex: Tex) {
        self.textures.push(tex);
    }

    pub fn get_groups(&self) -> &Vec<Group> {
        &self.groups
    }
}