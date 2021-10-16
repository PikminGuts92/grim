use crate::{SystemInfo, io::*};
use crate::model::{Draw, GroupObject, MatObject, MeshObject, TexPath, Trans, Vert};
use gltf::buffer::Data as BufferData;
use gltf::{Document, Gltf, Mesh, Primitive, Scene};
use gltf::image::{Data as ImageData, Source};
use gltf::mesh::*;
use gltf::mesh::util::*;
use gltf::json::extensions::scene::*;
use gltf::json::extensions::mesh::*;
use gltf::scene::Node;
use grim_traits::scene::{Color3, UV, Vector4};
use itertools::{Itertools, izip};
use nalgebra as na;
use std::{borrow::Borrow, error::Error};
use std::path::{Path, PathBuf};

use crate::model::AssetManagager;

pub struct GLTFImporter {
    model_path: PathBuf,
    document: Option<Document>,
    buffers: Vec<BufferData>,
    images: Vec<ImageData>,
    mats: Vec<MatObject>,
}

impl GLTFImporter {
    pub fn new<T>(model_path: T) -> Result<GLTFImporter, Box<dyn Error>> where T: AsRef<Path> {
        let (document, buffers, images) = gltf::import(&model_path)?;

        Ok(GLTFImporter {
            model_path: model_path.as_ref().to_owned(),
            document: Some(document),
            buffers,
            images,
            mats: Vec::new(),
        })
    }

    pub fn process(&mut self, info: SystemInfo) -> Result<AssetManagager, Box<dyn Error>> {
        // Hacky way to get around ownership when iterating over scenes
        let mut document = self.document.take().unwrap();

        let mut asset_manager = AssetManagager::new(info);
        self.process_materials(&mut document, &mut asset_manager);

        // TODO: How to handle same mesh used in different scenes?
        for scene in document.scenes() {
            // Create group name
            let group_name = match scene.name() {
                Some(name) => format!("{}.grp", name),
                None => format!("group_{}.grp", scene.index()),
            };

            let mut group = GroupObject {
                name: group_name,
                ..GroupObject::default()
            };

            for node in scene.nodes() {
                let mut meshes = self.process_node(&node, &mut asset_manager)?;

                for mesh in meshes.iter_mut() {
                    mesh.parent = group.name.to_owned();
                    group.objects.push(mesh.name.to_owned());
                }

                // Add meshes to asset manager
                while !meshes.is_empty() {
                    let mut mesh = meshes.remove(0);
                    transform_verts(&mut mesh.vertices); // Update to DX coordinates

                    asset_manager.add_mesh(mesh);
                }
            }

            asset_manager.add_group(group);
        }

        // Add materials to asset manager
        while !self.mats.is_empty() {
            asset_manager.add_material(self.mats.remove(0));
        }

        self.document = Some(document); // Give back
        Ok(asset_manager)
    }

    pub fn process_materials(&mut self, document: &mut Document, asset_manager: &mut AssetManagager) {
        for doc_mat in document.materials() {
            // Create mat name
            let mat_name = match doc_mat.name() {
                Some(name) => format!("{}.mat", name),
                None => format!("mat_{}.mat", doc_mat.index().unwrap()),
            };

            let mut mat = MatObject::default();
            mat.name = mat_name;

            // Get base color
            let [r, g, b, a] = doc_mat.pbr_metallic_roughness().base_color_factor();
            mat.color = Color3 { r, g, b };
            mat.alpha = a;

            // Get diffuse texture
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
                        let tex = TexPath {
                            name: tex_name,
                            rgba: Vec::new(),
                            png_path,
                        };

                        asset_manager.add_tex(tex);
                    }
                }
            }

            self.mats.push(mat);
        }
    }

    fn process_node(&mut self, node: &Node, asset_manager: &mut AssetManagager) -> Result<Vec<MeshObject>, Box<dyn Error>> {
        let mut meshes = Vec::new();

        // Process mesh
        if let Some(mesh) = node.mesh() {
            let mut milo_meshes = self.read_mesh(&mesh);
            meshes.append(&mut milo_meshes);
        }

        // Process children
        for child_node in node.children() {
            let mut sub_meshes = self.process_node(&child_node, asset_manager)?;
            meshes.append(&mut sub_meshes);
        }

        // Apply transforms
        let trans = node.transform();
        let matrix = trans.matrix();

        for mesh in meshes.iter_mut() {
            transform_verts_with_mat(&mut mesh.vertices, &matrix);
        };

        Ok(meshes)
    }

    fn read_mesh(&mut self, mesh: &Mesh) -> Vec<MeshObject> {
        let mesh_name_prefix = match mesh.name() {
            Some(name) => name.to_string(),
            None => format!("mesh_{}", mesh.index()),
        };

        let mut meshes = Vec::new();

        for prim in mesh.primitives() {
            let mut mesh = self.read_primitive(&prim, &mesh_name_prefix);
            mesh.recompute_face_groups();
            meshes.push(mesh);
        }

        meshes
    }

    fn read_primitive(&mut self, prim: &Primitive, mesh_name_prefix: &str) -> MeshObject {
        let reader = prim.reader(|buffer| Some(&self.buffers[buffer.index()]));

        let faces: Vec<u16> = match reader.read_indices().unwrap() {
            ReadIndices::U8(itr) => itr.map(|i| i as u16).collect(),
            ReadIndices::U16(itr) => itr.collect(),
            ReadIndices::U32(itr) => itr.map(|i| i as u16).collect(),
        };

        let faces_chunked = faces.chunks_exact(3);

        let faces: Vec<[u16; 3]> = faces_chunked
            .map(|f| [
                *f.get(2).unwrap(), // Clockwise -> Anti
                *f.get(1).unwrap(),
                *f.get(0).unwrap(),
            ])
            .collect();

        let verts_interleaved = izip!(
            reader.read_positions().unwrap(),
            reader.read_normals().unwrap(),
            //reader.read_colors(0).unwrap().into_rgb_f32().into_iter(),
            reader.read_tex_coords(0).unwrap().into_f32(),
        );

        let verts = verts_interleaved
            .map(|(pos, norm, uv)| Vert {
                pos: Vector4 {
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
                    ..Vector4::default()
                },
                normals: Vector4 {
                    x: match norm.get(0) {
                        Some(n) => *n,
                        _ => 0.0,
                    },
                    y: match norm.get(1) {
                        Some(n) => *n,
                        _ => 0.0,
                    },
                    z: match norm.get(2) {
                        Some(n) => *n,
                        _ => 0.0,
                    },
                    ..Vector4::default()
                },
                uv: UV {
                    u: match uv.get(0) {
                        Some(u) => match u {
                            //u if *u > 1.0 => u.fract(),
                            //u if *u < 0.0 => u.fract() + 1.0,
                            _ => *u,
                        },
                        _ => 0.0,
                    },
                    v: match uv.get(1) {
                        Some(v) => match v {
                            //v if *v > 1.0 => v.fract(),
                            //v if *v < 0.0 => v.fract() + 1.0,
                            _ => *v,
                        },
                        _ => 0.0,
                    },
                },
                ..Vert::default()
            })
            .collect::<Vec<Vert>>();

        let mat_name = match prim.material().index() {
            Some(idx) => self.mats[idx].name.to_owned(),
            None => String::from(""),
        };

        let mesh_name = match prim.index() {
            0 => format!("{}.mesh", mesh_name_prefix),
            _ => format!("{}_{}.mesh", mesh_name_prefix, prim.index()),
        };

        MeshObject {
            name: mesh_name.to_owned(),
            vertices: verts,
            faces,
            mat: mat_name,
            geom_owner: mesh_name,
            parent: String::default(),
            ..MeshObject::default()
        }
    }
}

fn transform_verts(verts: &mut Vec<Vert>) {
    let mat = na::Matrix4::new(
        -1.0,  0.0,  0.0, 0.0,
        0.0,  0.0,  1.0, 0.0,
        0.0,  1.0,  0.0, 0.0,
        0.0,  0.0,  0.0, 1.0,
    );

    for vert in verts.iter_mut() {
        let Vector4 { x, y, z, .. } = &mut vert.pos;

        // Update position
        let pos = mat.transform_vector(&na::Vector3::new(*x, *y, *z));
        *x = *pos.get(0).unwrap();
        *y = *pos.get(1).unwrap();
        *z = *pos.get(2).unwrap();
    }
}

fn transform_verts_with_mat(verts: &mut Vec<Vert>, matrix: &[[f32; 4]; 4]) {
    let mat = na::Matrix4::from(matrix.to_owned());

    for vert in verts.iter_mut() {
        let Vector4 { x, y, z, .. } = &mut vert.pos;

        // Update position
        let pos = mat.transform_vector(&na::Vector3::new(*x, *y, *z));
        *x = *pos.get(0).unwrap();
        *y = *pos.get(1).unwrap();
        *z = *pos.get(2).unwrap();
    }
}