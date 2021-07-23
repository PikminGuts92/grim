use crate::io::*;
use crate::model::{Draw, Trans};
use gltf::buffer::Data as BufferData;
use gltf::{Document, Gltf, Mesh, Primitive, Scene};
use gltf::image::{Data as ImageData, Source};
use gltf::mesh::*;
use gltf::mesh::util::*;
use gltf::json::extensions::scene::*;
use gltf::json::extensions::mesh::*;
use gltf::scene::Node;
use itertools::{Itertools, izip};
use nalgebra as na;
use std::{borrow::Borrow, error::Error};
use std::path::{Path, PathBuf};

use crate::model::{AssetManagager, Face, Group, Mat, MiloMesh, Tex, Vertex};

pub struct GLTFImporter {
    model_path: PathBuf,
    document: Option<Document>,
    buffers: Vec<BufferData>,
    images: Vec<ImageData>,
    mat_path: Option<PathBuf>,
    mats: Vec<Mat>,
}

impl GLTFImporter {
    pub fn new<T>(model_path: T) -> Result<GLTFImporter, Box<dyn Error>> where T: AsRef<Path> {
        let (document, buffers, images) = gltf::import(&model_path)?;

        Ok(GLTFImporter {
            model_path: model_path.as_ref().to_owned(),
            document: Some(document),
            buffers,
            images,
            mat_path: None,
            mats: Vec::new(),
        })
    }

    pub fn use_mat<T>(&mut self, mat_path: T) where T: AsRef<Path> {
        self.mat_path = Some(mat_path.as_ref().to_path_buf());
    }

    pub fn process(&mut self) -> Result<AssetManagager, Box<dyn Error>> {
        // Hacky way to get around ownership when iterating over scenes
        let mut document = self.document.take().unwrap();

        let mut asset_manager = AssetManagager::new();
        self.process_materials(&mut document, &mut asset_manager);

        // TODO: How to handle same mesh used in different scenes?
        for scene in document.scenes() {
            // Create group name
            let group_name = match scene.name() {
                Some(name) => format!("{}.grp", name),
                None => format!("group_{}.grp", scene.index()),
            };

            let mut group = Group {
                name: group_name,
                objects: Vec::new(),
            };

            for node in scene.nodes() {
                let mut meshes = self.process_node(&node, &mut asset_manager)?;

                for mesh in meshes.iter_mut() {
                    mesh.parent = Some(group.name.to_owned());
                    group.objects.push(mesh.name.to_owned());
                }

                // Add meshes to asset manager
                while meshes.len() > 0 {
                    asset_manager.add_mesh(meshes.remove(0));
                }
            }

            asset_manager.add_group(group);
        }

        // Add materials to asset manager
        while self.mats.len() > 0 {
            asset_manager.add_material(self.mats.remove(0));
        }

        self.document = Some(document); // Give back
        Ok(asset_manager)
    }

    pub fn process_materials(&mut self, document: &mut Document, asset_manager: &mut AssetManagager) {
        let base_mat = match &self.mat_path {
            Some(mat_path) => Mat::from_mat_file(mat_path).unwrap(), // TODO: Safely handle
            _ => panic!("External material required!"), // TODO: Use system default
        };

        for doc_mat in document.materials() {
            // Create mat name
            let mat_name = match doc_mat.name() {
                Some(name) => format!("{}.mat", name),
                None => format!("mat_{}.mat", doc_mat.index().unwrap()),
            };

            let mut mat = base_mat.clone();
            mat.name = mat_name;

            if let Some(diffuse_tex) = doc_mat.pbr_metallic_roughness().base_color_texture() {
                // For now copy exising png files
                let tex_source = diffuse_tex.texture().source().source();
                if let Source::Uri { uri, mime_type: _ } = tex_source {
                    let png_path = self.model_path.parent().unwrap().join(uri);

                    let tex_name = format!(
                        "{}.tex",
                        png_path.file_stem().unwrap().to_str().unwrap().to_ascii_lowercase()
                    );
                    mat.diffuse_tex = tex_name.to_owned();

                    // Existing texture not found, create new one
                    if asset_manager.get_texture(&tex_name).is_none() {
                        let tex = Tex {
                            name: tex_name,
                            rgba: Vec::new(),
                            png_path,
                        };

                        asset_manager.add_tex(tex);
                    }
                }
            }

            // TODO: How to handle materials without texture (i.e. base color)?

            self.mats.push(mat);
        }
    }

    fn process_node(&mut self, node: &Node, asset_manager: &mut AssetManagager) -> Result<Vec<MiloMesh>, Box<dyn Error>> {
        let mut meshes = Vec::new();

        for child_node in node.children() {
            let mut sub_meshes = self.process_node(&child_node, asset_manager)?;
            meshes.append(&mut sub_meshes);
        }

        // Process mesh
        if let Some(mesh) = node.mesh() {
            let milo_mesh = self.read_mesh(&mesh);
        }

        // Apply transforms
        let trans = node.transform();
        let matrix = trans.matrix();

        Ok(meshes)
    }

    fn read_mesh(&mut self, mesh: &Mesh) -> Vec<MiloMesh> {
        let mesh_name_prefix = match mesh.name() {
            Some(name) => format!("{}", name),
            None => format!("mesh_{}", mesh.index()),
        };

        let mut meshes = Vec::new();

        for prim in mesh.primitives() {
            let mesh = self.read_primitive(&prim, &mesh_name_prefix);
            meshes.push(mesh);
        }

        meshes
    }

    fn read_primitive(&mut self, prim: &Primitive, mesh_name_prefix: &str) -> MiloMesh {
        let reader = prim.reader(|buffer| Some(&self.buffers[buffer.index()]));

        let faces: Vec<u16> = match reader.read_indices().unwrap() {
            ReadIndices::U8(itr) => itr.map(|i| i as u16).collect(),
            ReadIndices::U16(itr) => itr.collect(),
            ReadIndices::U32(itr) => itr.map(|i| i as u16).collect(),
        };

        let faces_chunked = faces.chunks_exact(3);

        let faces: Vec<Face> = faces_chunked
            .map(|f| Face {
                v1: *f.get(0).unwrap(), // Clockwise -> Anti
                v2: *f.get(1).unwrap(),
                v3: *f.get(2).unwrap(),
            })
            .collect();

        let verts_interleaved = izip!(
            reader.read_positions().unwrap(),
            reader.read_normals().unwrap(),
            //reader.read_colors(0).unwrap().into_rgb_f32().into_iter(),
            reader.read_tex_coords(0).unwrap().into_f32(),
        );

        let verts = verts_interleaved
            .map(|(pos, norm, uv)| Vertex {
                x: match pos.get(0) {
                    Some(p) => *p,
                    _ => 0.0,
                },
                y: match pos.get(1) {
                    Some(p) => *p,
                    _ => 0.0,
                },
                z: match pos.get(2) {
                    Some(p) => *p,
                    _ => 0.0,
                },
                nx: match norm.get(0) {
                    Some(n) => *n,
                    _ => 0.0,
                },
                ny: match norm.get(1) {
                    Some(n) => *n,
                    _ => 0.0,
                },
                nz: match norm.get(2) {
                    Some(n) => *n,
                    _ => 0.0,
                },
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
                u: match uv.get(0) {
                    Some(u) => match u {
                        u if *u > 1.0 => u.fract(),
                        u if *u < 0.0 => u.fract() + 1.0,
                        _ => *u,
                    },
                    _ => 0.0,
                },
                v: match uv.get(1) {
                    Some(v) => match v {
                        v if *v > 1.0 => v.fract(),
                        v if *v < 0.0 => v.fract() + 1.0,
                        _ => *v,
                    },
                    _ => 0.0,
                },
            })
            .collect::<Vec<Vertex>>();

        let mat_name = match prim.material().index() {
            Some(idx) => self.mats[idx].name.to_owned(),
            None => String::from(""),
        };

        let mesh_name = match prim.index() {
            0 => format!("{}.mesh", mesh_name_prefix),
            _ => format!("{}_{}.mesh", mesh_name_prefix, prim.index()),
        };

        MiloMesh {
            name: mesh_name,
            verts,
            faces,
            mat: mat_name,
            parent: None,
        }
    }
}