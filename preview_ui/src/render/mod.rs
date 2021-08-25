use bevy::prelude::*;
use bevy::render::texture::{Extent3d, TextureDimension, TextureFormat};

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

use grim::{Platform, SystemInfo};
use grim::io::*;
use grim::scene::{RndMesh, MeshObject, MiloObject, Object, ObjectDir, PackedObject, Tex};

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
    bevy_textures: &mut ResMut<Assets<Texture>>,
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

    let textures = entries
        .iter()
        .map(|o| match o {
            Object::Tex(tex) => Some(tex),
            _ => None,
        })
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect::<Vec<_>>();

    let mut tex_map = HashMap::new();

    for tex in textures.iter() {
        if let Some(bitmap) = &tex.bitmap {
            match bitmap.unpack_rgba(system_info) {
                Ok(rgba) => {
                    println!("Processing {}", tex.get_name());

                    // TODO: Figure out how bevy can support mip maps
                    let tex_size = (bitmap.width as usize) * (bitmap.height as usize) * 4;

                    let bevy_tex = Texture::new_fill(
                        Extent3d {
                            width: bitmap.width.into(),
                            height: bitmap.height.into(),
                            depth: 1,
                        },
                        TextureDimension::D2,
                        &rgba[..tex_size],
                        TextureFormat::Rgba8UnormSrgb,
                    );

                    tex_map.insert(tex.get_name().as_str(), bevy_tex);
                },
                Err(err) => {
                    println!("Failed to convert {}", tex.get_name());
                }
            }
        }
    }

    println!("Found {} meshes, {} textures, and {} materials", meshes.len(), textures.len(), mats.len());

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

        let matrix = Mat4::from_cols_array(&[
            -1.0,  0.0,  0.0, 0.0,
            0.0,  0.0,  1.0, 0.0,
            0.0,  1.0,  0.0, 0.0,
            0.0,  0.0,  0.0, 1.0,
        ]);

        let mat = mats
            .iter()
            .find(|m| m.get_name().eq(&mesh.mat));

        let bevy_mat = match mat {
            Some(mat) => StandardMaterial {
                base_color: Color::rgba(
                    mat.color.r,
                    mat.color.g,
                    mat.color.b,
                    mat.alpha,
                ),
                double_sided: true,
                unlit: false,
                base_color_texture: match tex_map.get(mat.diffuse_tex.as_str()) {
                    Some(texture)
                        => Some(bevy_textures.add(texture.to_owned())),
                    None => None,
                },
                // TODO: Add extra texture maps
                /*normal_map: match tex_map.get(mat.normal_map.as_str()) {
                    Some(texture)
                        => Some(bevy_textures.add(texture.to_owned())),
                    None => None,
                },
                emissive_texture: match tex_map.get(mat.emissive_map.as_str()) {
                    Some(texture)
                        => Some(bevy_textures.add(texture.to_owned())),
                    None => None,
                },*/
                ..Default::default()
            },
            None => StandardMaterial {
                base_color: Color::rgb(0.3, 0.5, 0.3),
                double_sided: true,
                unlit: false,
                ..Default::default()
            },
        };

        // Add mesh
        commands.spawn_bundle(PbrBundle {
            mesh: bevy_meshes.add(bevy_mesh),
            material: materials.add(bevy_mat),
            transform: Transform::from_matrix(matrix)
                * Transform::from_scale(Vec3::new(0.1, 0.1, 0.1)),
            ..Default::default()
        });

        println!("Added {}", &mesh.name);
    }
}