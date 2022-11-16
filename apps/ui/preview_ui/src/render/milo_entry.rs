use bevy::prelude::*;
use bevy::render::render_resource::{AddressMode, Extent3d, SamplerDescriptor, TextureDimension, TextureFormat};
use bevy::render::texture::ImageSampler;

use itertools::*;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::num::NonZeroU8;
use std::path::{Path, PathBuf};
use thiserror::Error;

use grim::{Platform, SystemInfo};
use grim::io::*;
use grim::scene::{GroupObject, Matrix, MeshObject, Milo, MiloObject, Object, ObjectDir, PackedObject, RndMesh, Tex, Trans, TransConstraint};
use grim::texture::Bitmap;

use crate::WorldMesh;
use super::{map_matrix, MiloLoader, TextureEncoding};

pub fn render_milo_entry(
    commands: &mut Commands,
    bevy_meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    bevy_textures: &mut ResMut<Assets<Image>>,
    milo: &ObjectDir,
    milo_entry: Option<String>,
    system_info: &SystemInfo,
) {
    let mut loader = MiloLoader::new(milo);

    // Get meshes for single object or return all meshes
    // TODO: Make less hacky
    let meshes = match milo_entry {
        Some(entry) => {
            let milo_object = loader.get_object(&entry).unwrap();
            get_object_meshes(
                milo_object,
                &mut loader,
            )
        },
        None => milo.get_entries()
            .iter()
            .map(|e| match e {
                Object::Mesh(m) => Some(m),
                _ => None,
            })
            .filter(|e| e.is_some())
            .map(|e| e.unwrap())
            .collect::<Vec<_>>()
    };

    // Translate to bevy coordinate system
    let trans_mat = Mat4::from_cols_array(&[
        -1.0,  0.0,  0.0, 0.0,
        0.0,  0.0,  1.0, 0.0,
        0.0,  1.0,  0.0, 0.0,
        0.0,  0.0,  0.0, 1.0,
    ]);

    // Scale down
    let scale_mat = Mat4::from_scale(Vec3::new(0.1, 0.1, 0.1));

    let trans = Transform::from_matrix(trans_mat * scale_mat);
    let global_trans = GlobalTransform::from(trans);

    // Root transform
    let root_entity = commands.spawn_empty()
        .insert(trans)
        .insert(global_trans)
        .insert(VisibilityBundle::default())
        .id();

    for mesh in meshes {
        // Ignore meshes without geometry (used mostly in GH1)
        if mesh.vertices.is_empty() || mesh.name.starts_with("shadow") {
            continue;
        }

        // Get transform
        let mat = get_computed_mat(mesh as &dyn Trans, &mut loader);

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

        // Load textures
        let tex_names = loader.get_mat(&mesh.mat)
            .map(|mat| (
                mat.diffuse_tex.to_owned(),
                mat.normal_map.to_owned(),
                mat.emissive_map.to_owned(),
            ));

        let (diffuse, normal, emissive) = tex_names
            .map(|(diffuse, normal, emissive)| (
                get_texture(&mut loader, &diffuse, system_info)
                    .map(map_texture)
                    .map(|t| bevy_textures.add(t)),
                get_texture(&mut loader, &normal, system_info)
                    .map(map_texture)
                    .map(|t| bevy_textures.add(t)),
                get_texture(&mut loader, &emissive, system_info)
                    .map(map_texture)
                    .map(|t| bevy_textures.add(t)),
            ))
            .unwrap_or_default();

        let bevy_mat = match loader.get_mat(&mesh.mat) {
            Some(mat) => StandardMaterial {
                base_color: Color::rgba(
                    mat.color.r,
                    mat.color.g,
                    mat.color.b,
                    mat.alpha,
                ),
                double_sided: true,
                unlit: true,
                base_color_texture: diffuse,
                normal_map_texture: normal,
                emissive_texture: emissive,
                //roughness: 0.8, // TODO: Bevy 0.6 migration
                /*base_color_texture: get_texture(&mut loader, &mat.diffuse_tex, system_info)
                    .and_then(map_texture)
                    .and_then(|t| Some(bevy_textures.add(t))),
                normal_map: get_texture(&mut loader, &mat.norm_detail_map, system_info)
                    .and_then(map_texture)
                    .and_then(|t| Some(bevy_textures.add(t))),
                emissive_texture: get_texture(&mut loader, &mat.emissive_map, system_info)
                    .and_then(map_texture)
                    .and_then(|t| Some(bevy_textures.add(t))),*/
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
        commands.entity(root_entity)
            .with_children(|parent| {
                parent.spawn(PbrBundle {
                    mesh: bevy_meshes.add(bevy_mesh),
                    material: materials.add(bevy_mat),
                    transform: Transform::from_matrix(mat),
                    ..Default::default()
                }).insert(WorldMesh {
                    name: mesh.name.to_owned(),
                    vert_count: mesh.vertices.len(),
                    face_count: mesh.faces.len()
                });
            });
    }
}

fn get_object_meshes<'a>(
    milo_object: &'a Object,
    loader: &mut MiloLoader<'a>,
) -> Vec<&'a MeshObject> {
    let mut meshes = Vec::new();

    match milo_object {
        Object::Group(grp) => {
            // Iterate sub objects
            for obj_name in &grp.objects {
                let child_object = loader.get_object(&obj_name);
                if child_object.is_none() {
                    continue;
                }

                let mut child_meshes = get_object_meshes(
                    child_object.unwrap(),
                    loader,
                );

                meshes.append(&mut child_meshes);
            }
        },
        Object::Mesh(mesh) => {
            meshes.push(mesh);

            // Iterate sub meshes
            for sub_draw_name in &mesh.draw_objects {
                let child_draw = loader.get_object(&sub_draw_name);
                if child_draw.is_none() {
                    continue;
                }

                let mut child_meshes = get_object_meshes(
                    child_draw.unwrap(),
                    loader,
                );

                meshes.append(&mut child_meshes);
            }
        },
        _ => {

        }
    };

    // Return meshes
    meshes
}

fn get_object_meshes_with_transform<'a>(
    milo_object: &'a Object,
    loader: &mut MiloLoader<'a>,
) -> Vec<(&'a MeshObject, Mat4)> {
    let mut meshes = Vec::new();

    match milo_object {
        Object::Group(grp) => {
            // Iterate sub objects
            for obj_name in &grp.objects {
                let child_object = loader.get_object(&obj_name);
                if child_object.is_none() {
                    continue;
                }

                let child_meshes = get_object_meshes_with_transform(
                    child_object.unwrap(),
                    loader,
                );

                for (mesh, mat) in child_meshes {
                    meshes.push((mesh, mat));
                }
            }
        },
        Object::Mesh(mesh) => {
            let mat = get_computed_mat(mesh as &dyn Trans, loader);
            meshes.push((mesh, mat));

            // Iterate sub meshes
            for sub_draw_name in &mesh.draw_objects {
                let child_draw = loader.get_object(&sub_draw_name);
                if child_draw.is_none() {
                    continue;
                }

                let child_meshes = get_object_meshes_with_transform(
                    child_draw.unwrap(),
                    loader,
                );

                for (mesh, mat) in child_meshes {
                    meshes.push((mesh, mat));
                }
            }
        },
        _ => {

        }
    };

    // Return meshes
    meshes
}

fn get_computed_mat<'a>(
    milo_object: &'a dyn Trans,
    loader: &mut MiloLoader<'a>,
) -> Mat4 {
    let parent_name = milo_object.get_parent();
    if parent_name.eq(milo_object.get_name()) {
        // References self, use own world transform
        return map_matrix(milo_object.get_world_xfm());
    }

    // Use relative transform
    if let Some(parent) = loader.get_transform(parent_name) {
        let parent_mat = get_computed_mat(parent, loader);
        let local_mat = map_matrix(milo_object.get_local_xfm());

        return parent_mat * local_mat;
    }

    if !parent_name.is_empty() {
        println!("Can't find trans for {}", parent_name);
    }

    map_matrix(milo_object.get_world_xfm())
}

fn get_product_local_mat<'a>(
    milo_object: &'a dyn Trans,
    loader: &mut MiloLoader<'a>,
) -> Mat4 {
    let parent_name = milo_object.get_parent();
    if parent_name.eq(milo_object.get_name()) {
        // References self, use own local transform
        return map_matrix(milo_object.get_local_xfm());
    }

    // Use relative transform
    if let Some(parent) = loader.get_transform(parent_name) {
        let parent_mat = get_product_local_mat(parent, loader);
        let local_mat = map_matrix(milo_object.get_local_xfm());

        return parent_mat * local_mat;
    }

    if parent_name.is_empty() {
        println!("Can't find trans for {}", parent_name);
    }

    map_matrix(milo_object.get_local_xfm())
}

fn get_texture<'a, 'b>(loader: &'b mut MiloLoader<'a>, tex_name: &str, system_info: &SystemInfo) -> Option<&'b (&'a Tex, Vec<u8>, TextureEncoding)> {
    // Check for cached texture
    if let Some(_cached) = loader.get_cached_texture(tex_name).take() {
        // TODO: Figure out why commented out line doesn't work (stupid lifetimes)
        //return Some(cached);
        return loader.get_cached_texture(tex_name);
    }

    // Get bitmap and decode texture
    // TODO: Check for external textures
    loader.get_texture(tex_name)
        .and_then(|t| t.bitmap.as_ref())
        .and_then(|b| match (system_info.platform, b.encoding) {
            (Platform::X360 | Platform::PS3, 8 | 24 | 32) => {
                let enc = match b.encoding {
                     8 => TextureEncoding::DXT1,
                    24 => TextureEncoding::DXT5,
                    32 | _ => TextureEncoding::ATI2,
                };

                let mut data = b.raw_data.to_owned();

                if system_info.platform.eq(&Platform::X360) {
                    // Swap bytes
                    for ab in data.chunks_mut(2) {
                        let tmp = ab[0];

                        ab[0] = ab[1];
                        ab[1] = tmp;
                    }
                }

                Some((data, enc))
            },
            _ => b.unpack_rgba(system_info).ok()
                .and_then(|rgba| Some((rgba, TextureEncoding::RGBA)))
        })
        .and_then(move |(rgba, enc)| {
            // Cache decoded texture
            loader.set_cached_texture(tex_name, rgba, enc);
            loader.get_cached_texture(tex_name)
        })
}

fn map_texture<'a>(tex: &'a (&'a Tex, Vec<u8>, TextureEncoding)) -> Image {
    let (tex, rgba, enc) = tex;

    let bpp: usize = match enc {
        TextureEncoding::DXT1 => 4,
        TextureEncoding::DXT5 | TextureEncoding::ATI2 => 8,
        TextureEncoding::RGBA => 32,
    };

    let tex_size = ((tex.width as usize) * (tex.height as usize) * bpp) / 8;
    let use_mips = rgba.len() > tex_size; // TODO: Always support mips?

    let img_slice = if use_mips {
        &rgba
    } else {
        &rgba[..tex_size]
    };

    let image_new_fn = match enc {
        TextureEncoding::RGBA => image_new_fill, // Use fill method for older textures
        _ => image_new,
    };

    let mut texture = /*Image::new_fill*/ image_new_fn(
        Extent3d {
            width: tex.width.into(),
            height: tex.height.into(),
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        img_slice,
        match enc {
            TextureEncoding::DXT1 => TextureFormat::Bc1RgbaUnormSrgb,
            TextureEncoding::DXT5 => TextureFormat::Bc3RgbaUnormSrgb,
            TextureEncoding::ATI2 => TextureFormat::Bc5RgUnorm,
            _ => TextureFormat::Rgba8UnormSrgb,
        }
    );

    // Update texture wrap mode
    texture.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
        address_mode_u: AddressMode::Repeat,
        address_mode_v: AddressMode::Repeat,
        anisotropy_clamp: NonZeroU8::new(16),
        ..SamplerDescriptor::default()
    });

    // Set mipmap level
    if use_mips {
        texture.texture_descriptor.mip_level_count = tex
            .bitmap
            .as_ref()
            .map_or(1, |b| (b.mip_maps as u32) + 1);
    }

    texture
}

fn image_new(
    size: Extent3d,
    dimension: TextureDimension,
    pixel: &[u8],
    format: TextureFormat,
) -> Image {
    // Problematic!!!
    /*debug_assert_eq!(
        size.volume() * format.pixel_size(),
        data.len(),
        "Pixel data, size and format have to match",
    );*/
    let mut image = Image {
        data: pixel.to_owned(),
        ..Default::default()
    };
    image.texture_descriptor.dimension = dimension;
    image.texture_descriptor.size = size;
    image.texture_descriptor.format = format;
    image
}

fn image_new_fill(
    size: Extent3d,
    dimension: TextureDimension,
    pixel: &[u8],
    format: TextureFormat,
) -> Image {
    let mut value = Image::default();
    value.texture_descriptor.format = format;
    value.texture_descriptor.dimension = dimension;
    value.resize(size);

    // Problematic!!!
    /*debug_assert_eq!(
        pixel.len() % format.pixel_size(),
        0,
        "Must not have incomplete pixel data."
    );
    debug_assert!(
        pixel.len() <= value.data.len(),
        "Fill data must fit within pixel buffer."
    );*/

    for current_pixel in value.data.chunks_exact_mut(pixel.len()) {
        current_pixel.copy_from_slice(pixel);
    }
    value
}