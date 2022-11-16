mod loader;
mod milo_entry;

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

pub use loader::*;
pub use milo_entry::*;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

use grim::{Platform, SystemInfo};
use grim::io::*;
use grim::scene::{RndMesh, Matrix, MeshObject, MiloObject, Object, ObjectDir, PackedObject, Tex, Trans, TransConstraint};

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
    bevy_textures: &mut ResMut<Assets<Image>>,
    milo: &ObjectDir,
    system_info: &SystemInfo,
) {
    let entries = milo.get_entries();

    let groups = entries
        .iter()
        .map(|o| match o {
            Object::Group(grp) => Some(grp),
            _ => None,
        })
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect::<Vec<_>>();

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

    let transforms = entries
        .iter()
        .map(|o| match o {
            Object::Group(grp) => Some(get_transform(grp)),
            Object::Mesh(mesh) => Some(get_transform(mesh)),
            Object::Trans(trans) => Some(get_transform(trans)),
            _ => None,
        })
        .filter(|t| t.is_some())
        .map(|o| o.unwrap())
        .collect::<Vec<_>>();

    let mut tex_map = HashMap::new();

    for tex in textures.iter() {
        if let Some(bitmap) = &tex.bitmap {
            // TODO: Use bevy supported texture formats instead of converting to rgba
            //  DXT1 = Bc1RgbaUnorm
            //  DXT5 = Bc3RgbaUnorm
            //  ATI2 = Bc5RgUnorm
            match bitmap.unpack_rgba(system_info) {
                Ok(rgba) => {
                    println!("Processing {}", tex.get_name());

                    // TODO: Figure out how bevy can support mip maps
                    let tex_size = (bitmap.width as usize) * (bitmap.height as usize) * 4;

                    let bevy_tex = Image::new_fill(
                        Extent3d {
                            width: bitmap.width.into(),
                            height: bitmap.height.into(),
                            depth_or_array_layers: 1
                        },
                        TextureDimension::D2,
                        &rgba[..tex_size],
                        TextureFormat::Rgba8UnormSrgb,
                    );

                    tex_map.insert(tex.get_name().as_str(), bevy_tex);
                },
                Err(_err) => {
                    println!("Failed to convert {}", tex.get_name());
                }
            }
        }
    }

    println!("Found {} groups, {} meshes, {} textures, and {} materials", groups.len(), meshes.len(), textures.len(), mats.len());

    for mesh in meshes {
        // Ignore meshes without geometry (used mostly in GH1)
        if mesh.vertices.is_empty() {
            continue;
        }

        let mut bevy_mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut tangents = Vec::new();
        let mut uvs = Vec::new();

        for vert in mesh.get_vertices() {
            positions.push([vert.pos.x, vert.pos.y, vert.pos.z]);

            // TODO: Figure out normals/tangents
            //normals.push([vert.normals.x, vert.normals.y, vert.normals.z]);
            normals.push([1.0, 1.0, 1.0]);
            tangents.push([0.0, 0.0, 0.0, 1.0]);

            uvs.push([vert.uv.u, vert.uv.v]);
        }

        let indices = bevy::render::mesh::Indices::U16(
            mesh.faces.iter().flat_map(|f| *f).collect()
        );

        bevy_mesh.set_indices(Some(indices));
        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_TANGENT, tangents);
        bevy_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

        // Get base matrix
        let base_matrix = transforms
            .iter()
            .find(|t| t.get_name().eq(mesh.get_parent()))
            .and_then(|p| Some(map_matrix(p.get_world_xfm())))
            .unwrap_or(map_matrix(mesh.get_world_xfm()));

        // Translate to bevy coordinate system
        let matrix = Mat4::from_cols_array(&[
            -1.0,  0.0,  0.0, 0.0,
            0.0,  0.0,  1.0, 0.0,
            0.0,  1.0,  0.0, 0.0,
            0.0,  0.0,  0.0, 1.0,
        ]);

        let mat = mats
            .iter()
            .find(|m| m.get_name().eq(&mesh.mat));

        if mat.is_none() {
            println!("Mat not found for \"{}\"", &mesh.mat);
        } else {
            let mat = mat.unwrap();
            if !mat.diffuse_tex.is_empty() && tex_map.get(mat.diffuse_tex.as_str()).is_none() {
                println!("Diffuse tex not found for \"{}\"", &mat.diffuse_tex);
            }
        }

        let bevy_mat = match mat {
            Some(mat) => StandardMaterial {
                base_color: Color::rgba(
                    mat.color.r,
                    mat.color.g,
                    mat.color.b,
                    mat.alpha,
                ),
                double_sided: true,
                unlit: true,
                base_color_texture: match tex_map.get(mat.diffuse_tex.as_str()) {
                    Some(texture)
                        => Some(bevy_textures.add(texture.to_owned())),
                    None => None,
                },
                // TODO: Add extra texture maps
                normal_map_texture: match tex_map.get(mat.normal_map.as_str()) {
                    Some(texture)
                        => Some(bevy_textures.add(texture.to_owned())),
                    None => None,
                },
                emissive_texture: match tex_map.get(mat.emissive_map.as_str()) {
                    Some(texture)
                        => Some(bevy_textures.add(texture.to_owned())),
                    None => None,
                },
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
        commands.spawn(PbrBundle {
            mesh: bevy_meshes.add(bevy_mesh),
            material: materials.add(bevy_mat),
            transform: Transform::from_matrix(base_matrix)
                * Transform::from_matrix(matrix)
                * Transform::from_scale(Vec3::new(0.1, 0.1, 0.1)),
            ..Default::default()
        });

        println!("Added {}", &mesh.name);
    }
}

fn get_transform<T: Trans>(trans: &T) -> &dyn Trans {
    trans
}

pub fn map_matrix(m: &Matrix) -> Mat4 {
    Mat4::from_cols_array(&[
        m.m11,
        m.m12,
        m.m13,
        m.m14,
        m.m21,
        m.m22,
        m.m23,
        m.m24,
        m.m31,
        m.m32,
        m.m33,
        m.m34,
        m.m41,
        m.m42,
        m.m43,
        m.m44,
    ])
}