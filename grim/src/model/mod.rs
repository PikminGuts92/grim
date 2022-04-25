//mod anim;
//mod draw;
mod gltf;
//mod group;
//mod mat;
//mod mesh;
mod tex_path;
//mod trans;

use std::{error::Error, fs::copy, path::Path};

use crate::SystemInfo;
use crate::scene::*;
pub(crate) use self::gltf::*;
pub use self::tex_path::*;

pub struct AssetManagager {
    info: SystemInfo,
    groups: Vec<GroupObject>,
    meshes: Vec<MeshObject>,
    materials: Vec<MatObject>,
    textures: Vec<TexPath>,
}

impl AssetManagager {
    pub fn new(info: SystemInfo) -> AssetManagager {
        AssetManagager {
            info,
            groups: Vec::new(),
            meshes: Vec::new(),
            materials: Vec::new(),
            textures: Vec::new(),
        }
    }

    pub fn get_group(&self, name: &str) -> Option<&GroupObject> {
        self.groups.iter().find(|g| g.name.eq(name))
    }

    pub fn get_mesh(&self, name: &str) -> Option<&MeshObject> {
        self.meshes.iter().find(|m| m.name.eq(name))
    }

    pub fn get_material(&self, name: &str) -> Option<&MatObject> {
        self.materials.iter().find(|m| m.name.eq(name))
    }

    pub fn get_texture(&self, name: &str) -> Option<&TexPath> {
        self.textures.iter().find(|t| t.name.eq(name))
    }

    pub fn add_group(&mut self, group: GroupObject) {
        self.groups.push(group);
    }

    pub fn add_mesh(&mut self, mesh: MeshObject) {
        self.meshes.push(mesh);
    }

    pub fn add_material(&mut self, mat: MatObject) {
        self.materials.push(mat);
    }

    pub fn add_tex(&mut self, tex: TexPath) {
        self.textures.push(tex);
    }

    pub fn get_groups(&self) -> &Vec<GroupObject> {
        &self.groups
    }

    pub fn dump_to_directory<T>(&self, out_dir: T) -> Result<(), Box<dyn Error>> where T: AsRef<Path> {
        // Create output dir
        create_dir_if_not_exists(&out_dir)?;

        let groups = self.get_groups();

        for grp in groups {
            let meshes: Vec<&MeshObject> = (&grp.objects).iter().map(|m| self.get_mesh(m).unwrap()).collect();

            for mesh in meshes {
                // Write mat
                let mat = self.get_material(&mesh.mat).unwrap();
                let mat_path = out_dir.as_ref().join(&mat.name);
                save_to_file(mat, &mat_path, &self.info)?;
                println!("Wrote {}", &mat.name);

                // Write diffuse tex
                if let Some(tex) = self.get_texture(&mat.diffuse_tex) {
                    let png_path = &tex.png_path;
                    let png_name = png_path.file_name().unwrap().to_str().unwrap().to_ascii_lowercase();
                    let dest_png_path = out_dir.as_ref().join(&png_name);

                    copy(png_path, &dest_png_path)?;
                    println!("Wrote {}", &png_name);
                }

                // Write mesh
                let mesh_path = out_dir.as_ref().join(&mesh.name);
                save_to_file(mesh, &mesh_path, &self.info)?;
                println!("Wrote {}", &mesh.name);
            }

            // Write group
            let group_path = out_dir.as_ref().join(&grp.name);
            save_to_file(grp, &group_path, &self.info)?;
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

pub fn open_model<T>(model_path: T, info: SystemInfo) -> Result<AssetManagager, Box<dyn Error>> where T: AsRef<Path> {
    let mut gltf_importer = GLTFImporter::new(&model_path)?;
    gltf_importer.process(info)
}