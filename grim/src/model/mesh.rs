use crate::io::*;
use crate::model::{Draw, Trans};
use gltf::{Gltf, Mesh, Primitive, Scene};
use gltf::image::Source;
use gltf::mesh::*;
use gltf::mesh::util::*;
use gltf::json::extensions::scene::*;
use gltf::json::extensions::mesh::*;
use itertools::{Itertools, izip};
use nalgebra as na;
use std::{borrow::Borrow, error::Error};
use std::path::Path;

use crate::model::{AssetManagager, Face, GLTFImporter, Group, Mat, Tex, Vertex};

pub fn open_model<T>(model_path: T, mat_path: T) -> Result<AssetManagager, Box<dyn Error>> where T: AsRef<Path> {
    let mut gltf_importer = GLTFImporter::new(&model_path)?;
    gltf_importer.use_mat(&mat_path);
    gltf_importer.process()
}

// TODO: Remove old code
pub fn open_model_old<T>(model_path: T, mat_path: T) -> Result<AssetManagager, Box<dyn Error>> where T: AsRef<Path> {
    let (model, buffers, images) = gltf::import(&model_path)?;

    let mut asset_manager = AssetManagager::new();
    let mut meshes = Vec::new();

    let mut unk_mat_count = 0;

    // Iterate mesh primitives (treat as seperate meshes)
    for (mesh_idx, mesh_name, prim_idx, prim) in model
        .meshes()
        .enumerate()
        .map(move |(m_idx, m)| m
            .primitives()
            .enumerate()
            .map(move |(p_idx, p)| (m_idx, m.name(), p_idx, p)))
                .flat_map(|p| p) {
        let mesh_suffix = match prim_idx {
            0 => String::default(),
            _ => format!("_{}", prim_idx),
        };

        let mesh_name = match mesh_name {
            Some(name) => format!("{}{}.mesh", name, mesh_suffix),
            None => format!("mesh_{}{}.mesh", mesh_idx, mesh_suffix),
        };

        let reader = prim.reader(|buffer| Some(&buffers[buffer.index()]));

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
                    Some(v) => *v,
                    _ => 0.0,
                },
            })
            .collect::<Vec<Vertex>>();

        transform_verts(&mut verts);

        // Print vert info
        for v in &verts {
            println!("{:?}", v);
        }
        println!("{} verts", verts.len());

        let prim_mat = prim.material();
        let mat_name = match prim_mat.name() {
            Some(name) => format!("{}.mat", name),
            None => {
                let mat_name = format!("mat_{}.mat", unk_mat_count);
                unk_mat_count += 1;
                mat_name
            },
        };

        // Existing material not found, create new one
        if asset_manager.get_material(&mat_name).is_none() {
            let mut mat = Mat::from_mat_file(mat_path.as_ref())?;
            mat.name = mat_name.to_owned();

            let base_color_tex = prim_mat.pbr_metallic_roughness().base_color_texture();
            //let tex = images.get(diffuse_tex.texture().index()).unwrap();

            if let Some(diffuse_tex) = base_color_tex {
                // For now copy exising png files
                let tex_source = diffuse_tex.texture().source().source();
                if let Source::Uri { uri, mime_type: _ } = tex_source {
                    let model_path = model_path.as_ref();
                    let png_path = model_path.parent().unwrap().join(uri);

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

            asset_manager.add_material(mat);
        }

        let mesh = MiloMesh {
            name: mesh_name.to_owned(),
            verts,
            faces,
            mat: mat_name.to_owned(),
            parent: Some(String::from("main.grp")),
        };

        if mat_name.is_empty() {
            // Skip meshes without material for now
            continue;
        }

        meshes.push(mesh.name.to_owned());
        asset_manager.add_mesh(mesh);
    }

    let group = Group {
        name: String::from("main.grp"),
        objects: meshes,
    };

    asset_manager.add_group(group);

    Ok(asset_manager)
}

fn transform_verts(verts: &mut Vec<Vertex>) {
    let mat = na::Matrix4::new(
        1.0,  0.0,  0.0, 0.0,
        0.0, -1.0,  0.0, 0.0,
        0.0,  0.0, -1.0, 0.0,
        0.0,  0.0,  0.0, 1.0,
    );

    for vert in verts.iter_mut() {
        // Update position
        let pos = mat.transform_vector(&na::Vector3::new(vert.x, vert.y, vert.z));
        vert.x = *pos.get(0).unwrap();
        vert.y = *pos.get(1).unwrap();
        vert.z = *pos.get(2).unwrap();

        // Update normal
        let norm = mat.transform_vector(&na::Vector3::new(vert.nx, vert.ny, vert.nz));
        vert.nx = *norm.get(0).unwrap();
        vert.ny = *norm.get(1).unwrap();
        vert.nz = *norm.get(2).unwrap();
    }
}

fn transform_verts_with_quat(verts: &mut Vec<Vertex>, i: f32, j: f32, k: f32, w: f32) {
    // TODO: Figure out less ugly way for type conversion
    let quat = na::Quaternion::new(w, i, j, k);
    let unit_quat = na::UnitQuaternion::from_quaternion(quat);
    let mat: na::Matrix4<f32> = unit_quat.into();

    for vert in verts.iter_mut() {
        // Update position
        let pos = mat.transform_vector(&na::Vector3::new(vert.x, vert.y, vert.z));
        vert.x = *pos.get(0).unwrap();
        vert.y = *pos.get(1).unwrap();
        vert.z = *pos.get(2).unwrap();

        // Update normal
        let norm = mat.transform_vector(&na::Vector3::new(vert.nx, vert.ny, vert.nz));
        vert.nx = *norm.get(0).unwrap();
        vert.ny = *norm.get(1).unwrap();
        vert.nz = *norm.get(2).unwrap();
    }
}

#[derive(Debug)]
pub struct MiloMesh {
    pub name: String,
    pub verts: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub mat: String,
    pub parent: Option<String>,
}

impl MiloMesh {
    pub fn write_to_file<T>(&self, out_path: T, version: u32) -> Result<(), Box<dyn Error>> where T: AsRef<Path> {
        // Write to file
        let mut stream = FileStream::from_path_as_read_write_create(out_path.as_ref())?;
        let mut writer = BinaryStream::from_stream_with_endian(&mut stream, IOEndian::Big);

        // Write version
        //writer.write_int32(36)?;
        writer.write_uint32(version)?;

        // Write meta
        // TODO: Use struct
        writer.write_uint32(2)?; // Revision - VERY important
        writer.write_bytes(&[0u8; 9])?;

        // Write trans
        let mut trans = Trans::default();
        trans.transform = match &self.parent {
            Some(parent) => parent.to_owned(),
            _ => self.name.to_owned(),
        };
        trans.write_to_stream(&mut writer)?;

        // Write draw
        let draw = Draw::default();
        draw.write_to_stream(&mut writer)?;

        writer.write_prefixed_string(&self.mat)?;
        writer.write_prefixed_string(&self.name)?;

        writer.write_int32(0)?;
        writer.write_int32(1)?;
        writer.write_int8(0)?;

        // Write vertices
        writer.write_uint32(self.verts.len() as u32)?;

        if version >= 36 {
            writer.write_int8(1)?;
            writer.write_uint32(36)?; // Size of vertex entry
            writer.write_uint32(1)?;
        }

        for v in &self.verts {
            // Write position
            writer.write_float32(v.x)?;
            writer.write_float32(v.y)?;
            writer.write_float32(v.z)?;

            if version == 34 {
                writer.write_float32(0.0)?; // w?
            }

            if version <= 34 {
                // Write normals
                writer.write_float32(v.nx)?;
                writer.write_float32(v.ny)?;
                writer.write_float32(v.nz)?;
                if version == 34 {
                    writer.write_float32(0.0)?; // w?
                }

                // Write color
                writer.write_float32(v.r)?;
                writer.write_float32(v.g)?;
                writer.write_float32(v.b)?;
                writer.write_float32(v.a)?;

                // Write UV
                writer.write_float32(v.u)?;
                writer.write_float32(v.v)?;

                if version == 34 {
                    // Write unknown data
                    writer.write_int16(0)?;
                    writer.write_int16(1)?;
                    writer.write_int16(2)?;
                    writer.write_int16(3)?;

                    writer.write_float32(1.0)?;
                    writer.write_float32(0.0)?;
                    writer.write_float32(0.0)?;
                    writer.write_float32(-1.0)?;
                }

                continue;
            }

            // Write UV
            //writer.write_int32(-1)?;
            writer.write_float16(f16::from_f32(v.u))?;
            writer.write_float16(f16::from_f32(v.v))?;

            // Write normals
            // TODO: Support f16
            writer.write_uint16(0)?;
            writer.write_uint16(0)?;
            writer.write_uint16(0)?;
            writer.write_uint16(0)?;

            // Write color
            writer.write_uint8(0xFF)?;
            writer.write_uint8(0xFF)?;
            writer.write_uint8(0xFF)?;
            writer.write_uint8(0xFF)?;

            // Write unknown indicies
            writer.write_uint16(0)?;
            writer.write_uint16(1)?;
            writer.write_uint16(2)?;
            writer.write_uint16(3)?;
        }

        // Write faces
        writer.write_uint32(self.faces.len() as u32)?;

        for f in &self.faces {
            writer.write_uint16(f.v1)?;
            writer.write_uint16(f.v2)?;
            writer.write_uint16(f.v3)?;
        }

        // Write groups
        let mut face_count = self.faces.len() as u32;
        let group_count = (face_count as f32 / 255.0).ceil() as u32;

        writer.write_uint32(group_count)?;

        while face_count > 0 {
            if face_count < 255 {
                writer.write_uint8(face_count as u8)?;
                break;
            }

            writer.write_uint8(255)?;
            face_count -= 255;
        }

        // Write bones
        writer.write_uint32(0)?;

        if version >= 36 {
            writer.write_uint8(0)?;
        }

        Ok(())
    }
}