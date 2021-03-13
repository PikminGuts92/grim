use gltf::{Gltf, Mesh, Scene};
use gltf::mesh::*;
use gltf::mesh::util::*;
use gltf::json::extensions::scene::*;
use gltf::json::extensions::mesh::*;
use std::error::Error;
use std::path::Path;

use crate::model::Vertex;

pub fn open_model<T>(model_path: T) -> Result<(), Box<dyn Error>> where T: AsRef<Path>  {
    let (model, buffers, images) = gltf::import(model_path)?;

    // Use first mesh for now
    let mesh = model.meshes().nth(0).unwrap();
    let prim = mesh.primitives().next().unwrap();

    let reader = prim.reader(|buffer| Some(&buffers[buffer.index()]));

    let faces: Vec<u8> = match reader.read_indices().unwrap() {
        ReadIndices::U8(itr) => itr.collect(),
        ReadIndices::U16(itr) => itr.map(|i| i as u8).collect(),
        ReadIndices::U32(itr) => itr.map(|i| i as u8).collect(),
    };

    print!("{:?}", faces);
    print!("{} faces", faces.len());

    if let Some(positions) = reader.read_positions() {
        let verts = positions.map(|p| Vertex {
            x: match p.get(0) {
                Some(p) => *p,
                _ => 0.0,
            },
            y: match p.get(1) {
                Some(p) => *p,
                _ => 0.0,
            },
            z: match p.get(2) {
                Some(p) => *p,
                _ => 0.0,
            },
            nx: 0.0,
            ny: 0.0,
            nz: 0.0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
            u: 0.0,
            v: 0.0,
        })
        .collect::<Vec<Vertex>>();

        print!("{:?}", verts);
        print!("{} verts", verts.len());
    }


    /*for prim in mesh.primitives() {

        let attr = prim.attributes();
    }*/

    Ok(())
}