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
    document: Document,
    buffers: Vec<BufferData>,
    images: Vec<ImageData>,
    mat_path: Option<PathBuf>
}

impl GLTFImporter {
    pub fn new<T>(model_path: T) -> Result<GLTFImporter, Box<dyn Error>> where T: AsRef<Path> {
        let (document, buffers, images) = gltf::import(&model_path)?;

        Ok(GLTFImporter {
            document,
            buffers,
            images,
            mat_path: None,
        })
    }

    pub fn use_mat<T>(&mut self, mat_path: T) where T: AsRef<Path> {
        self.mat_path = Some(mat_path.as_ref().to_path_buf());
    }

    pub fn process(&mut self) -> Result<AssetManagager, Box<dyn Error>> {
        let mut asset_manager = AssetManagager::new();

        for scene in self.document.scenes() {
            for node in scene.nodes() {
                let meshes = self.process_node(&node, &mut asset_manager)?;
            }
        }

        Ok(asset_manager)
    }

    fn process_node(&self, node: &Node, asset_manager: &mut AssetManagager) -> Result<Vec<MiloMesh>, Box<dyn Error>> {
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

    fn read_mesh(&self, mesh: &Mesh) -> MiloMesh {
        for prim in mesh.primitives() {
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

            let mut verts = verts_interleaved
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
        }

        MiloMesh {
            name: mesh.name().unwrap().to_string(),
            verts: Vec::new(),
            faces: Vec::new(),
            mat: String::from(""),
            parent: None,
        }
    }
}