use bevy::prelude::*;

use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

use grim::{Platform, SystemInfo};
use grim::io::*;
use grim::scene::{RndMesh, MeshObject, Object, ObjectDir, PackedObject, Tex};

pub fn open_and_unpack_milo<T: AsRef<Path>>(milo_path: T) -> Result<(ObjectDir, SystemInfo), Box<dyn Error>> {
    let milo_path = milo_path.as_ref();

    let mut stream: Box<dyn Stream> = Box::new(FileStream::from_path_as_read_open(milo_path)?);
    let milo = MiloArchive::from_stream(&mut stream)?;

    let system_info = SystemInfo::guess_system_info(&milo, &milo_path);
    let mut obj_dir = milo.unpack_directory(&system_info)?;
    obj_dir.unpack_entries(&system_info)?;

    Ok((obj_dir, system_info))
}

pub fn render_milo(
    commands: &mut Commands,
    bevy_meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    milo: &ObjectDir,
    system_info: &SystemInfo,
) {
    let entries = milo.get_entries();

    let mats = entries
        .iter()
        .map(|o| match o {
            Object::Mat(mat) => Some(mat),
            _ => None,
        })
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect::<Vec<_>>();

    let meshes = entries
        .iter()
        .map(|o| match o {
            Object::Mesh(mesh) => Some(mesh),
            _ => None,
        })
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect::<Vec<_>>();

    println!("Found {} meshes", meshes.len());

    for mesh in meshes {
        let mut bevy_mesh = Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();

        for vert in mesh.get_vertices() {
            positions.push([vert.pos.x, vert.pos.y, vert.pos.z]);

            // TODO: Figure out normals
            //normals.push([vert.normals.x, vert.normals.y, vert.normals.z]);
            normals.push([1.0, 1.0, 1.0]);

            uvs.push([vert.uv.u, vert.uv.v]);
        }

        let indices = bevy::render::mesh::Indices::U16(
            mesh.faces.iter().flat_map(|f| *f).collect()
        );

        bevy_mesh.set_indices(Some(indices));
        bevy_mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        bevy_mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        bevy_mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        let mat = Mat4::from_cols_array(&[
            -1.0,  0.0,  0.0, 0.0,
            0.0,  0.0,  1.0, 0.0,
            0.0,  1.0,  0.0, 0.0,
            0.0,  0.0,  0.0, 1.0,
        ]);

        // Add mesh
        commands.spawn_bundle(PbrBundle {
            mesh: bevy_meshes.add(bevy_mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 0.5, 0.3),
                double_sided: true,
                unlit: false,
                ..Default::default()
            }),
            transform: Transform::from_matrix(mat),
            ..Default::default()
        });

        println!("Added {}", &mesh.name);
    }
}