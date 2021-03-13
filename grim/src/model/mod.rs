mod group;
mod mat;
mod mesh;
mod tex;

use std::{error::Error, path::Path};

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

    pub fn dump_to_directory<T>(&self, out_dir: T) -> Result<(), Box<dyn Error>> where T: AsRef<Path> {
        // Create output dir
        create_dir_if_not_exists(&out_dir)?;

        let groups = self.get_groups();

        for grp in groups {
            let meshes: Vec<&MiloMesh> = (&grp.objects).iter().map(|m| self.get_mesh(&m).unwrap()).collect();

            for mesh in meshes {
                // Write mat
                let mat = self.get_material(&mesh.mat).unwrap();
                let mat_path = out_dir.as_ref().join(&mat.name);
                mat.write_to_file(&mat_path)?;
                println!("Wrote {}", &mat.name);

                // TODO: Write tex
                //let diffuse_tex = self.get_texture(&mat.diffuse_tex).unwrap();
                //println!("Wrote {}", &diffuse_tex.name);

                // Write mesh
                let mesh_path = out_dir.as_ref().join(&mesh.name);
                println!("Wrote {}", &mesh.name);
            }

            println!("Wrote {}", &grp.name);
        }

        Ok(())
    }
}

pub(crate) fn create_dir_if_not_exists<T>(dir_path: T) -> Result<(), Box<dyn Error>> where T: AsRef<Path> {
    let dir_path = dir_path.as_ref();

    if !dir_path.exists() {
        // Not found, create directory
        std::fs::create_dir_all(&dir_path)?;
    }

    Ok(())
}