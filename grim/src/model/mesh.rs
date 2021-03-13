use gltf::{Gltf, Mesh, Scene};
use gltf::mesh::*;
use gltf::mesh::util::*;
use gltf::json::extensions::scene::*;
use gltf::json::extensions::mesh::*;
use itertools::{Itertools, izip};
use std::error::Error;
use std::path::Path;

use crate::model::{AssetManagager, Face, Group, Mat, Tex, Vertex};

pub fn open_model<T>(model_path: T, mat_path: T) -> Result<AssetManagager, Box<dyn Error>> where T: AsRef<Path>  {
    let (model, buffers, images) = gltf::import(model_path)?;

    // Use first mesh for now
    let mesh = model.meshes().nth(0).unwrap();
    let prim = mesh.primitives().next().unwrap();

    let reader = prim.reader(|buffer| Some(&buffers[buffer.index()]));

    let faces: Vec<u16> = match reader.read_indices().unwrap() {
        ReadIndices::U8(itr) => itr.map(|i| i as u16).collect(),
        ReadIndices::U16(itr) => itr.collect(),
        ReadIndices::U32(itr) => itr.map(|i| i as u16).collect(),
    };

    let faces_chunked = faces.chunks_exact(3);

    let faces: Vec<Face> = faces_chunked
        .map(|f| Face {
            v1: *f.get(0).unwrap(),
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
                Some(u) => *u,
                _ => 0.0,
            },
            v: match uv.get(1) {
                Some(v) => *v,
                _ => 0.0,
            },
        })
        .collect::<Vec<Vertex>>();

    // Print vert info
    for v in &verts {
        println!("{:?}", v);
    }
    print!("{} verts", verts.len());

    let mat = prim.material();
    let diffuse_tex = mat.pbr_metallic_roughness().base_color_texture().unwrap();
    let tex = images.get(diffuse_tex.texture().index()).unwrap();

    let mut asset_manager = AssetManagager::new();

    let mut mat = Mat::from_mat_file(mat_path)?;
    mat.name = String::from("main.mat");
    mat.diffuse_tex = String::from("main.tex");

    let mesh = MiloMesh {
        name: String::from("main.mesh"),
        verts,
        faces,
        mat: mat.name.to_owned(),
    };

    let group = Group {
        name: String::from("main.grp"),
        objects: vec![
            mesh.name.to_owned()
        ],
    };

    asset_manager.add_material(mat);
    asset_manager.add_mesh(mesh);
    asset_manager.add_group(group);

    Ok(asset_manager)
}

#[derive(Debug)]
pub struct MiloMesh {
    pub name: String,
    pub verts: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub mat: String,
}