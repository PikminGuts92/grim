use crate::io::*;
use crate::scene::*;
//use grim_traits::scene::*;
use crate::{Platform, SystemInfo};
use itertools::*;
use gltf_json as json;
use nalgebra as na;
use serde::ser::Serialize;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::rc::Rc;

//type TransObject = dyn Trans + MiloObject;

pub struct BoneNode<'a> {
    pub object: &'a dyn Trans,
    pub children: Vec<BoneNode<'a>>
}

fn get_child_nodes<'a>(parent_name: &str, bone_map: &HashMap<&str, &'a dyn Trans>, child_map: &HashMap<&str, Vec<&dyn Trans>>) -> Vec<BoneNode<'a>> {
    let Some(children) = child_map.get(parent_name) else {
        return Vec::new();
    };

    children
        .iter()
        .sorted_by(|a, b| a.get_name().cmp(b.get_name())) // Sort by name
        .map(|c| BoneNode {
            object: *bone_map.get(c.get_name().as_str()).unwrap(),
            children: get_child_nodes(c.get_name().as_str(), bone_map, child_map)
        })
        .collect()
}

pub fn find_bones<'a>(obj_dir: &'a ObjectDir) -> Vec<BoneNode<'a>> {
    let dir_name = match obj_dir {
        ObjectDir::ObjectDir(base) => base.name.as_str(),
    };

    let bones = obj_dir
        .get_entries()
        .iter()
        .filter_map(|o| match o {
            Object::Mesh(m) if m.faces.is_empty() // GH1 bones
                => Some(m as &dyn Trans),
            Object::Trans(t) => Some(t as &dyn Trans),
            _ => None
        })
        .map(|b| (b.get_name().as_str(), b))
        .collect::<HashMap<_, _>>();

    // Map parent to children
    let child_map = bones
        .iter()
        .fold(HashMap::new(), |mut acc: HashMap<&str, Vec<&'a dyn Trans>>, (_, b)| {
            if b.get_parent().eq(b.get_name()) {
                // If bone references self, ignore
                return acc;
            }

            acc
                .entry(b.get_parent().as_str())
                .and_modify(|e| e.push(*b))
                .or_insert(vec![*b]);

            acc
        });

    let mut root_nodes = Vec::new();

    // Add bones that belong to object dir
    let mut dir_nodes = get_child_nodes(dir_name, &bones, &child_map);
    root_nodes.append(&mut dir_nodes);

    // TODO: Add unparented bones

    root_nodes
}

fn map_bones_to_nodes(dir_name: &str, bones: &Vec<BoneNode>) -> Vec<gltf_json::Node> {
    let mut nodes = Vec::new();

    // Add root obj dir node
    // Ugh... no default derive...
    nodes.push(gltf_json::Node {
        camera: None,
        children: None,
        extensions: None,
        extras: None,
        matrix: Some([
            -1.0,  0.0,  0.0, 0.0,
            0.0,  0.0,  1.0, 0.0,
            0.0,  1.0,  0.0, 0.0,
            0.0,  0.0,  0.0, 1.0,
        ]),
        mesh: None,
        name: Some(dir_name.to_string()),
        rotation: None,
        scale: None,
        translation: None,
        skin: None,
        weights: None,
    });

    let child_indices = populate_child_nodes(&mut nodes, bones);

    if !child_indices.is_empty() {
        nodes[0].children = Some(child_indices);
    }

    //bones
    //    .into_iter()
    //    .enumerate()
    //    .map(|(i, b)|)
    //    .collect()

    nodes
}

fn populate_child_nodes(nodes: &mut Vec<gltf_json::Node>, bones: &Vec<BoneNode>) -> Vec<gltf_json::Index<gltf_json::Node>> {
    let mut indices = Vec::new();

    for bone in bones {
        let child_indices = populate_child_nodes(nodes, &bone.children);

        let m = bone.object.get_local_xfm();
        //let m = Matrix::indentity();

        let mat = na::Matrix4::new(
            // Column-major order...
            m.m11, m.m21, m.m31, m.m41,
            m.m12, m.m22, m.m32, m.m42,

            m.m13, m.m23, m.m33, m.m43,
            m.m14, m.m24, m.m34, m.m44,

            /*m.m11, m.m12, m.m13, m.m14,
            m.m21, m.m22, m.m23, m.m24,
            m.m31, m.m32, m.m33, m.m34,
            m.m41, m.m42, m.m43, m.m44*/
        );

        //let scale_mat = na::Matrix4::new_scaling(1.0);

        /*let trans_mat = na::Matrix4::new(
            -1.0,  0.0,  0.0, 0.0,
            0.0,  0.0,  1.0, 0.0,
            0.0,  1.0,  0.0, 0.0,
            0.0,  0.0,  0.0, 1.0,
        );

        let trans_mat = na::Matrix4::new(
            trans_mat[0], trans_mat[4], trans_mat[8], trans_mat[12],
            trans_mat[1], trans_mat[5], trans_mat[9], trans_mat[13],
            trans_mat[2], trans_mat[6], trans_mat[10], trans_mat[14],
            trans_mat[3], trans_mat[7], trans_mat[11], trans_mat[15],
        );

        // TODO: Apply translation...
        let mat = mat * trans_mat;*/

        //let mat = mat * scale_mat;

        //na::Matrix::from

        //let mut gltf_mat 

        let node = gltf_json::Node {
            camera: None,
            children: if !child_indices.is_empty() {
                Some(child_indices)
            } else {
                None
            },
            extensions: None,
            extras: None,
            matrix: if mat.is_identity(f32::EPSILON) {
                // Don't add identities
                None
            } else {
                mat
                    .as_slice()
                    .try_into()
                    .ok()
            },
            mesh: None,
            name: Some(bone.object.get_name().to_string()),
            rotation: None,
            scale: None,
            translation: None,
            skin: None,
            weights: None,
        };

        nodes.push(node);
        indices.push(gltf_json::Index::new((nodes.len() - 1) as u32));
    }

    indices
}

fn get_textures<'a>(obj_dir: &'a ObjectDir) -> Vec<&Tex> {
    obj_dir
        .get_entries()
        .iter()
        .filter_map(|e| match e {
            // TODO: Support external textures
            Object::Tex(tex) if tex.bitmap.is_some() => Some(tex),
            _ => None
        })
        .collect()
}

pub fn export_object_dir_to_gltf(obj_dir: &ObjectDir, output_path: &Path, sys_info: &SystemInfo) {
    super::create_dir_if_not_exists(output_path).unwrap();

    let dir_name = match obj_dir {
        ObjectDir::ObjectDir(base) => base.name.as_str(),
    };

    let textures = get_textures(&obj_dir);

    let images = textures
        .into_iter()
        .map(|t| json::Image {
            buffer_view: None,
            mime_type: Some(json::image::MimeType(String::from("image/png"))),
            name: Some(t.get_name().to_owned()),
            uri: {
                use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

                // Decode image
                let rgba = t.bitmap
                    .as_ref()
                    .unwrap()
                    .unpack_rgba(sys_info)
                    .unwrap();

                let (width, height) = t.bitmap
                    .as_ref()
                    .map(|b| (b.width as u32, b.height as u32))
                    .unwrap();

                // Convert to png
                let png_data = crate::texture::write_rgba_to_vec(width, height, &rgba).unwrap();

                // Encode to base64
                let mut str_data = String::from("data:image/png;base64,");
                general_purpose::STANDARD.encode_string(&png_data, &mut str_data);

                Some(str_data)
            },
            extensions: None,
            extras: None
        })
        .collect();

    let bones = find_bones(&obj_dir);
    let nodes = map_bones_to_nodes(dir_name, &bones);

    let joints = nodes
        .iter()
        .enumerate()
        .map(|(i, _)| json::Index::new(i as u32))
        .collect::<Vec<_>>();

    let root = json::Root {
        asset: json::Asset {
            generator: Some(format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))),
            ..Default::default()
        },
        images,
        nodes: map_bones_to_nodes(dir_name, &bones),
        scene: Some(json::Index::new(0)),
        scenes: vec![
            json::Scene {
                extensions: None,
                extras: None,
                name: None,
                nodes: vec![json::Index::new(0)],
            }
        ],
        skins: vec![
            json::Skin {
                extensions: None,
                extras: None,
                inverse_bind_matrices: None,
                joints: joints,
                name: None,
                skeleton: Some(json::Index::new(0))
            }
        ],
        ..Default::default()
    };

    // Write gltf json
    let writer = std::fs::File::create(output_path.join(format!("{dir_name}.gltf"))).expect("I/O error");
    json::serialize::to_writer_pretty(writer, &root).expect("Serialization error");

    // Write gltf buffer
}

#[derive(Default)]
pub struct GltfExportSettings {
    pub custom_basename: Option<String>,
    pub embed_textures: bool,
    pub write_as_binary: bool
}

pub struct ObjectDirData {
    dir: ObjectDir,
    entries: Vec<Object>,
    path: PathBuf,
    info: SystemInfo
}

struct MappedObject<T: MiloObject> {
    parent: Rc<ObjectDirData>,
    object: T
}

impl<T: MiloObject> MappedObject<T> {
    fn new(object: T, parent: Rc<ObjectDirData>) -> MappedObject<T> {
        MappedObject {
            object,
            parent
        }
    }
}

fn is_mesh_joint(m: &MappedObject<MeshObject>) -> bool {
    m.parent.info.version <= 10 && m.object.faces.is_empty()
}

#[derive(Default)]
pub struct GltfExporter {
    object_dirs: Vec<ObjectDirData>, // TODO: Replace with new milo environment?
    dirs_rc: Vec<Rc<ObjectDirData>>,
    settings: GltfExportSettings,
    groups: HashMap<String, MappedObject<GroupObject>>,
    materials: HashMap<String, MappedObject<MatObject>>,
    meshes: HashMap<String, MappedObject<MeshObject>>,
    transforms: HashMap<String, MappedObject<TransObject>>,
    textures: HashMap<String, MappedObject<Tex>>,

    // TODO: Move to nested struct?
    gltf: json::Root,
    image_indices: HashMap<String, usize>,
}

impl GltfExporter {
    pub fn new() -> GltfExporter {
        GltfExporter::default()
    }

    pub fn with_settings(settings: GltfExportSettings) -> GltfExporter {
        GltfExporter {
            settings,
            ..Default::default()
        }
    }

    pub fn add_milo_from_path<T: Into<PathBuf>>(&mut self, path: T) -> Result<(), Box<dyn Error>> {
        // TODO: Return custom error type
        let milo_path: PathBuf = path.into();

        // Open milo
        let mut stream: Box<dyn Stream> = Box::new(FileStream::from_path_as_read_open(&milo_path)?);
        let milo = MiloArchive::from_stream(&mut stream)?;

        // Guess system info and unpack dir + entries
        let system_info = SystemInfo::guess_system_info(&milo, &milo_path);
        let mut obj_dir = milo.unpack_directory(&system_info)?;
        obj_dir.unpack_entries(&system_info)?;

        let entries = obj_dir.take_entries();

        // Add to list
        self.object_dirs.push(ObjectDirData {
            dir: obj_dir,
            entries: entries,
            path: milo_path,
            info: system_info
        });

        Ok(())
    }

    fn map_objects(&mut self) {
        self.groups.clear();
        self.materials.clear();
        self.meshes.clear();
        self.transforms.clear();
        self.textures.clear();

        self.dirs_rc.clear();

        for mut dir_entry in self.object_dirs.drain(..) {
            let entries = dir_entry.entries.drain(..).collect::<Vec<_>>();
            let parent = Rc::new(dir_entry);

            for entry in entries {
                let name = entry.get_name().to_owned();

                match entry {
                    Object::Group(group) => {
                        self.groups.insert(
                            name,
                            MappedObject::new(group, parent.clone())
                        );
                    },
                    Object::Mat(mat) => {
                        self.materials.insert(
                            name,
                            MappedObject::new(mat, parent.clone())
                        );
                    },
                    Object::Mesh(mesh) => {
                        self.meshes.insert(
                            name,
                            MappedObject::new(mesh, parent.clone())
                        );
                    },
                    Object::Tex(tex) => {
                        self.textures.insert(
                            name,
                            MappedObject::new(tex, parent.clone())
                        );
                    },
                    Object::Trans(trans) => {
                        self.transforms.insert(
                            name,
                            MappedObject::new(trans, parent.clone())
                        );
                    },
                    _ => {}
                }
            }

            self.dirs_rc.push(parent);
        }
    }

    fn get_transform<'a>(&'a self, name: &str) -> Option<&'a dyn Trans> {
        self.transforms
            .get(name)
            .map(|t| &t.object as &dyn Trans)
            .or(self.groups.get(name).map(|g| &g.object as &dyn Trans))
            .or(self.meshes.get(name).map(|m| &m.object as &dyn Trans))
    }

    fn get_mesh<'a>(&'a self, name: &str) -> Option<&MeshObject> {
        self.meshes
            .get(name)
            .map(|m| &m.object)
    }

    fn process_node<'a>(&'a self, gltf: &mut json::Root, name: &'a str, child_map: &HashMap<&'a str, Vec<&'a str>>, depth: usize) -> usize {
        let node_index = gltf.nodes.len();

        // Get + compute transform matrix
        let mat = match (self.get_transform(name), depth) {
            (Some(trans), 0) => {
                let m = trans.get_world_xfm();

                let mat = na::Matrix4::new(
                    // Column-major order...
                    m.m11, m.m21, m.m31, m.m41,
                    m.m12, m.m22, m.m32, m.m42,
                    m.m13, m.m23, m.m33, m.m43,
                    m.m14, m.m24, m.m34, m.m44
                );

                // Apply translation
                let trans_mat = na::Matrix4::new(
                    -1.0,  0.0,  0.0, 0.0,
                     0.0,  0.0,  1.0, 0.0,
                     0.0,  1.0,  0.0, 0.0,
                     0.0,  0.0,  0.0, 1.0,
                );

                mat * trans_mat
            },
            (Some(trans), _) => {
                let m = trans.get_local_xfm();

                na::Matrix4::new(
                    // Column-major order...
                    m.m11, m.m21, m.m31, m.m41,
                    m.m12, m.m22, m.m32, m.m42,
                    m.m13, m.m23, m.m33, m.m43,
                    m.m14, m.m24, m.m34, m.m44
                )
            },
            (None, 0) => {
                na::Matrix4::new(
                    -1.0,  0.0,  0.0, 0.0,
                     0.0,  0.0,  1.0, 0.0,
                     0.0,  1.0,  0.0, 0.0,
                     0.0,  0.0,  0.0, 1.0,
                )
            },
            _ => na::Matrix4::identity()
        };

        gltf.nodes.push(gltf_json::Node {
            camera: None,
            children: None,
            extensions: None,
            extras: None,
            matrix: if mat.is_identity(f32::EPSILON) {
                // Don't add identities
                None
            } else {
                mat
                    .as_slice()
                    .try_into()
                    .ok()
            },
            mesh: None,
            name: Some(name.to_owned()),
            rotation: None,
            scale: None,
            translation: None,
            skin: None,
            weights: None,
        });

        if let Some(children) = child_map.get(name) {
            let mut child_indices = Vec::new();

            for child_name in children {
                /*if !self.transforms.contains_key(*child_name) {
                    continue;
                }*/

                let idx = self.process_node(gltf, child_name, child_map, depth + 1);
                child_indices.push(gltf_json::Index::new(idx as u32));
            }

            if !child_indices.is_empty() {
                gltf.nodes[node_index].children = Some(child_indices);
            }
        }

        node_index
    }

    fn process_textures(&self, gltf: &mut json::Root) -> HashMap<String, usize> {
        let mut image_indices = HashMap::new();

        gltf.samplers = vec![
            json::texture::Sampler {
                mag_filter: Some(json::validation::Checked::Valid(json::texture::MagFilter::Linear)),
                min_filter: Some(json::validation::Checked::Valid(json::texture::MinFilter::Nearest)),
                wrap_s: json::validation::Checked::Valid(json::texture::WrappingMode::Repeat),
                wrap_t: json::validation::Checked::Valid(json::texture::WrappingMode::Repeat),
                ..Default::default()
            }
        ];

        (gltf.images, gltf.textures) = self.textures
            .values()
            .sorted_by(|a, b| a.object.get_name().cmp(b.object.get_name()))
            .filter(|t| t.object.bpp != 24) // TODO: Support 24bpp textures...
            .enumerate()
            .map(|(i, mt)| {
                let t = &mt.object;
                let sys_info = &mt.parent.info;

                // Remove .tex extension
                // TODO: Use more robust method
                let image_name = t.get_name().replace(".tex", ".png");

                image_indices.insert(t.get_name().to_owned(), i);

                let image = json::Image {
                    buffer_view: None,
                    mime_type: Some(json::image::MimeType(String::from("image/png"))),
                    name: Some(image_name),
                    uri: {
                        use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

                        // Decode image
                        let rgba = t.bitmap
                            .as_ref()
                            .unwrap()
                            .unpack_rgba(sys_info)
                            .unwrap();

                        let (width, height) = t.bitmap
                            .as_ref()
                            .map(|b| (b.width as u32, b.height as u32))
                            .unwrap();

                        // Convert to png
                        let png_data = crate::texture::write_rgba_to_vec(width, height, &rgba).unwrap();

                        // Encode to base64
                        // TODO: Support writing to external files
                        let mut str_data = String::from("data:image/png;base64,");
                        general_purpose::STANDARD.encode_string(&png_data, &mut str_data);

                        Some(str_data)
                    },
                    extensions: None,
                    extras: None
                };

                let texture = json::Texture {
                    name: Some(t.get_name().to_owned()),
                    sampler: Some(json::Index::new(0u32)),
                    source: json::Index::new(i as u32), // Image index
                    extensions: None,
                    extras: None
                };

                (image, texture)
            })
            .fold((Vec::new(), Vec::new()), |(mut imgs, mut texs), (img, tex)| {
                imgs.push(img);
                texs.push(tex);
                (imgs, texs)
            });

        image_indices
    }

    fn process_materials(&self, gltf: &mut json::Root, tex_map: &HashMap<String, usize>) -> HashMap<String, usize> {
        let mut mat_indices = HashMap::new();

        gltf.materials = self.materials
            .values()
            .sorted_by(|a, b| a.object.get_name().cmp(b.object.get_name()))
            .enumerate()
            .map(|(i, mm)| {
                let mat = &mm.object;
                let diff_tex = tex_map.get(&mat.diffuse_tex);
                let _norm_tex = tex_map.get(&mat.normal_map);
                let _spec_tex = tex_map.get(&mat.specular_map);

                mat_indices.insert(mat.get_name().to_owned(), i);

                json::Material {
                    name: Some(mat.get_name().to_owned()),
                    pbr_metallic_roughness: json::material::PbrMetallicRoughness {
                        base_color_texture: diff_tex
                            .map(|d| json::texture::Info {
                                index: json::Index::new(*d as u32),
                                tex_coord: 0,
                                extensions: None,
                                extras: None
                            }),
                        //base_color_factor:
                        ..Default::default()
                    },
                    emissive_factor: json::material::EmissiveFactor([0.0f32; 3]),
                    alpha_mode: json::validation::Checked::Valid(json::material::AlphaMode::Mask),
                    double_sided: true,
                    ..Default::default()
                }
            })
            .collect();

        mat_indices
    }

    fn find_skins(&self, gltf: &mut json::Root, acc_builder: &mut AccessorBuilder) -> HashMap<String, (usize, usize)> {
        let root_indices = gltf
            .scenes[0]
            .nodes
            .iter()
            .map(|n| n.value())
            .collect::<Vec<_>>();

        let mut skins = Vec::new();
        let mut bone_indices = HashMap::new();

        for (i, idx) in root_indices.into_iter().enumerate() {
            let mut joints = Vec::new();
            self.find_joints(gltf, idx, &mut joints, na::Matrix4::identity());

            if !joints.is_empty() {
                // TODO: Figure out how to handle when nested
                let root_joint = idx;

                // Sort by index
                joints.sort_by(|(a, _), (b, _)| a.cmp(b));

                for (j, _) in joints.iter() {
                    let node_name = gltf.nodes[*j].name.as_ref().unwrap();
                    bone_indices.insert(node_name.to_owned(), (skins.len(), *j)); // (Skin idx, node idx)
                }

                // Add ibm list to accessors
                let ibm_idx = acc_builder.add_array(
                    format!("skin_{i}"),
                    joints
                        .iter()
                        .map(|(_, m)| m.as_slice().try_into().unwrap_or_default())
                        .collect::<Vec<[f32; 16]>>()
                );

                skins.push(json::Skin {
                    extensions: None,
                    extras: None,
                    inverse_bind_matrices: ibm_idx
                        .map(|i| json::Index::new(i as u32)),
                    joints: joints
                        .into_iter()
                        .map(|(j, _)| json::Index::new(j as u32))
                        .collect(),
                    name: None,
                    skeleton: Some(json::Index::new(root_joint as u32))
                });
            }
        }

        gltf.skins = skins;
        bone_indices
    }

    fn find_joints(&self, gltf: &json::Root, idx: usize, joints: &mut Vec<(usize, na::Matrix4<f32>)>, parent_mat: na::Matrix4<f32>) {
        let (node_name, children, mat) = gltf
            .nodes
            .get(idx)
            .map(|n| (
                n.name.as_ref().unwrap(),
                &n.children,
                n.matrix
                    .map(|m| na::Matrix4::from_column_slice(&m))
                    .unwrap_or_default()
            ))
            .unwrap();

        // Is a joint if Trans or Mesh w/ no faces
        let is_joint = self.transforms.contains_key(node_name.as_str())
            || self.meshes.get(node_name.as_str()).map(is_mesh_joint).unwrap_or_default();

        if is_joint {
            // Calculate inverse bind matrix (shouldn't fail but idk)
            let mut ibm = (parent_mat * mat).try_inverse().unwrap_or_default();
            ibm[15] = 1.0; // Force for precision

            // Add index to joint list
            joints.push((idx, ibm));
        }

        if let Some(children) = children {
            // Traverse children
            for child in children {
                self.find_joints(gltf, child.value(), joints, mat);
            }
        }
    }

    fn process_accessor_data(&self, gltf: &mut json::Root) {
        //let mut acc_indices = HashMap::new();

        /*gltf.accessors = self.meshes
            .values()
            .map(|m| &m.object)
            .filter(|m| !m.faces.is_empty())
            .fold((0, 0, 0, 0), |(vn, uv, wt, fc), m| (
                vn + (m.vertices.len() * 12 * 2), // Verts + norms
                uv + (m.vertices.len() * 8),      // UVs
                wt + (m.vertices.len() * 16 * 2), // Weights + tangents
                fc + (m.faces.len() * 6)          // Faces
            ));*/

        let (bv_verts_norms, bv_uvs, bv_weights_tans, mut bv_faces) = self.meshes
            .values()
            .map(|m| &m.object)
            .filter(|m| !m.faces.is_empty())
            .fold((0, 0, 0, 0), |(vn, uv, wt, fc), m| (
                vn + (m.vertices.len() * 12 * 2), // Verts + norms
                uv + (m.vertices.len() * 8),      // UVs
                wt + (m.vertices.len() * 16 * 2), // Weights + tangents
                fc + (m.faces.len() * 6)          // Faces
            ));

        // Make multiple of 4
        bv_faces = align_to_multiple_of_four(bv_faces);
        let total_size = bv_verts_norms + bv_uvs + bv_weights_tans + bv_faces;

        gltf.buffers = vec![{
            use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

            // TODO: Encode actual data...
            let bin_data = vec![0u8; total_size];

            let mut str_data = String::from("data:application/octet-stream;base64,");
            general_purpose::STANDARD.encode_string(&bin_data, &mut str_data);
            
            json::Buffer {
                name: None,
                byte_length: total_size as u32,
                uri: Some(str_data),
                extensions: None,
                extras: None
            }
        }];

        gltf.buffer_views = vec![
            json::buffer::View {
                name:  Some(String::from("verts_norms")),
                byte_length: bv_verts_norms as u32,
                byte_offset: Some(0),
                byte_stride: Some(12),
                buffer: json::Index::new(0),
                target: None,
                extensions: None,
                extras: None
            },
            json::buffer::View {
                name:  Some(String::from("uvs")),
                byte_length: bv_uvs as u32,
                byte_offset: Some(bv_verts_norms as u32),
                byte_stride: Some(8),
                buffer: json::Index::new(0),
                target: None,
                extensions: None,
                extras: None
            },
            json::buffer::View {
                name:  Some(String::from("weights_tans")),
                byte_length: bv_weights_tans as u32,
                byte_offset: Some((bv_verts_norms + bv_uvs) as u32),
                byte_stride: Some(16),
                buffer: json::Index::new(0),
                target: None,
                extensions: None,
                extras: None
            },
            json::buffer::View {
                name:  Some(String::from("faces")),
                byte_length: bv_faces as u32,
                byte_offset: Some((bv_verts_norms + bv_uvs + bv_weights_tans) as u32),
                byte_stride: None,
                buffer: json::Index::new(0),
                target: None,
                extensions: None,
                extras: None
            }
        ];
    }

    fn process_meshes(&self, gltf: &mut json::Root, acc_builder: &mut AccessorBuilder, mat_map: &HashMap<String, usize>, joint_map: &HashMap<String, (usize, usize)>) -> HashMap<String, usize> {
        let milo_meshes = self
            .meshes
            .values()
            .filter(|m| !is_mesh_joint(m))
            .map(|m| &m.object)
            .sorted_by(|a, b| a.get_name().cmp(b.get_name()))
            .collect::<Vec<_>>();

        // Get skins
        // Compute relative skin indices
        let local_joint_map = gltf
            .skins
            .iter()
            .map(|s| s.joints
                .iter()
                .enumerate()
                .map(|(ji, jnode)| (
                    // Get local skin index of joint
                    gltf.nodes[jnode.value()].name.as_ref().unwrap(),
                    ji
                ))
                .collect::<Vec<_>>())
            .enumerate()
            .fold(HashMap::new(), |mut acc, (si, mut joints)| {
                joints
                    .drain(..)
                    .for_each(|(name, ji)| {
                        acc.insert(name, (si, ji));
                    });

                acc
            });

        // Map mesh name to node index
        let mesh_node_map = gltf
            .nodes
            .iter()
            .enumerate()
            .filter_map(|(i, node)| node.name.as_ref().map(|n| (n.to_owned(), i)))
            .collect::<HashMap<_, _>>();

        // Track skinned meshes
        let mut meshes_to_update = Vec::new();

        let mut meshes = Vec::new();
        let mut mesh_map = HashMap::new();

        for mesh in milo_meshes {
            let pos_idx = acc_builder.add_array(
                format!("{}_pos", mesh.get_name()),
                mesh.get_vertices().iter().map(|v| [v.pos.x, v.pos.y, v.pos.z])
            );

            let norm_idx = acc_builder.add_array(
                format!("{}_norm", mesh.get_name()),
                mesh.get_vertices().iter().map(|v| [v.normals.x, v.normals.y, v.normals.z])
            );

            let uv_idx = acc_builder.add_array(
                format!("{}_uv", mesh.get_name()),
                mesh.get_vertices().iter().map(|v| [v.uv.u, v.uv.v])
            );

            let mut weight_idx = None;
            let mut bone_idx = None;

            // Get joint info
            // Convert local bone offset to skin joint offset
            let joint_translate_map = mesh
                .bones
                .iter()
                .enumerate()
                .flat_map(|(i, b)| local_joint_map
                    .get(&b.name)
                    .map(|j| (i, *j)))
                .collect::<HashMap<_, _>>();

            // Only add if bones found
            if !joint_translate_map.is_empty() {
                // Convert mesh bones to vert bones
                let bones = [
                    joint_translate_map.get(&0).map(|(_, b)| *b as u16).unwrap_or_default(),
                    joint_translate_map.get(&1).map(|(_, b)| *b as u16).unwrap_or_default(),
                    joint_translate_map.get(&2).map(|(_, b)| *b as u16).unwrap_or_default(),
                    joint_translate_map.get(&3).map(|(_, b)| *b as u16).unwrap_or_default(),
                ];

                // Create combined bones + weights
                let (conv_weights, conv_bones) = mesh.get_vertices()
                    .iter()
                    .map(|v| {
                        let w = v.weights;
                        let mut b = bones.to_owned();

                        // If weight is 0.0, set bone index to 0
                        for (b, w) in b.iter_mut().zip_eq(w) {
                            if w.eq(&0.0) {
                                *b = 0;
                            }
                        }

                        (w, b)
                    })
                    .fold((Vec::new(), Vec::new()), |(mut ws, mut bs), (w, b)| {
                        ws.push(w);
                        bs.push(b);

                        (ws, bs)
                    });

                // Add bone weights
                weight_idx = acc_builder.add_array(
                    format!("{}_weight", mesh.get_name()),
                    conv_weights
                );

                // Add bone indices
                bone_idx = acc_builder.add_array(
                    format!("{}_bone", mesh.get_name()),
                    conv_bones
                );

                // Get first skin (all bones should use the same skin...)
                // Still need to check in case bone isn't found
                let skin_idx = (0..4).find_map(|i| joint_translate_map.get(&i).map(|(s, _)| *s));

                if let Some(skin_idx) = skin_idx {
                    let node_idx = mesh_node_map.get(mesh.get_name());

                    if let Some(node_idx) = node_idx {
                        meshes_to_update.push((*node_idx, skin_idx));
                    }
                }
            }

            // Ignore tangents for now
            let tan_idx: Option<usize> = None;
            /*let tan_idx = acc_builder.add_array(
                format!("{}_tan", mesh.get_name()),
                mesh.get_vertices().iter().map(|v| [v.tangent.x, v.tangent.y, v.tangent.z, v.tangent.w])
            );*/

            // Need to be scalar for some reason
            let face_idx = acc_builder.add_scalar(
                format!("{}_face", mesh.get_name()),
                mesh.get_faces().iter().map(|f| f.to_owned()).flatten()
            );

            let mesh_idx = meshes.len();

            meshes.push(json::Mesh {
                name: Some(mesh.get_name().to_owned()),
                primitives: vec![
                    json::mesh::Primitive {
                        attributes: {
                            let mut map = HashMap::new();

                            // Add positions
                            if let Some(acc_idx) = pos_idx {
                                map.insert(
                                    json::validation::Checked::Valid(json::mesh::Semantic::Positions),
                                    json::Index::new(acc_idx as u32)
                                );
                            }

                            // Add normals
                            if let Some(acc_idx) = norm_idx {
                                map.insert(
                                    json::validation::Checked::Valid(json::mesh::Semantic::Normals),
                                    json::Index::new(acc_idx as u32)
                                );
                            }

                            // Add uvs
                            if let Some(acc_idx) = uv_idx {
                                map.insert(
                                    json::validation::Checked::Valid(json::mesh::Semantic::TexCoords(0)),
                                    json::Index::new(acc_idx as u32)
                                );
                            }

                            // Add weights
                            if let Some(acc_idx) = weight_idx {
                                map.insert(
                                    json::validation::Checked::Valid(json::mesh::Semantic::Weights(0)),
                                    json::Index::new(acc_idx as u32)
                                );
                            }

                            // Add bones
                            if let Some(acc_idx) = bone_idx {
                                map.insert(
                                    json::validation::Checked::Valid(json::mesh::Semantic::Joints(0)),
                                    json::Index::new(acc_idx as u32)
                                );
                            }

                            // Add tangents
                            if let Some(acc_idx) = tan_idx {
                                map.insert(
                                    json::validation::Checked::Valid(json::mesh::Semantic::Tangents),
                                    json::Index::new(acc_idx as u32)
                                );
                            }

                            map
                        },
                        indices: face_idx
                            .map(|idx| json::Index::new(idx as u32)),
                        material: mat_map
                            .get(&mesh.mat)
                            .map(|idx| json::Index::new(*idx as u32)),
                        mode: json::validation::Checked::Valid(gltf::mesh::Mode::Triangles),
                        targets: None,
                        extras: None,
                        extensions: None
                    },
                ],
                weights: None,
                extras: None,
                extensions: None
            });

            // Update map
            mesh_map.insert(mesh.get_name().to_owned(), mesh_idx);
        }

        // Update skins for each mesh node updated
        for (node_idx, skin_idx) in meshes_to_update {
            //gltf.nodes[node_idx].skin = Some(json::Index::new(skin_idx as u32));
        }

        // Assign meshes and return mesh indices
        gltf.meshes = meshes;
        mesh_map
    }

    fn final_process_nodes(&self, gltf: &mut json::Root, mesh_map: &HashMap<String, usize>, joint_map: &HashMap<String, (usize, usize)>) {
        // Useless code... does nothing
        for i in 0..gltf.nodes.len() {
            // Get node name
            let Some(node_name) = gltf.nodes[i].name.as_ref().map(|n| n.to_owned()) else {
                continue;
            };

            if let Some(mesh_idx) = mesh_map.get(&node_name) {
                // Update mesh index for node
                gltf.nodes[i].mesh = Some(json::Index::new(*mesh_idx as u32));
            } else {
                // Can't add skin without mesh
                continue;
            }

            if let Some((skin_idx, _)) = joint_map.get(&node_name) {
                // Update skin index for node
                gltf.nodes[i].skin = Some(json::Index::new(*skin_idx as u32));
            }
        }

        /*let milo_meshes = self
            .meshes
            .values()
            .filter(|m| !is_mesh_joint(m))
            .map(|m| m.object.get_name())
            .collect::<HashSet<_>>();

        let skin_nodes = gltf
            .skins
            .iter()
            .map(|s| s.skeleton.unwrap().value());*/
    }

    /*fn update_meshes_with_skins(&self, gltf: &mut json::Root, node: usize, skin: usize, meshes: &) {

    }*/

    fn map_mesh_nodes(&self, gltf: &mut json::Root) -> HashMap<String, usize> {
        todo!()
    }

    fn build_binary(&self, gltf: &mut json::Root, acc_builder: AccessorBuilder) {
        let (accessors, views, mut buffer, data) = acc_builder.generate("test.bin");

        // TODO: Write to external file
        buffer.uri = {
            use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

            let mut str_data = String::from("data:application/octet-stream;base64,");
            general_purpose::STANDARD.encode_string(&data, &mut str_data);

            Some(str_data)
        };

        gltf.accessors = accessors;
        gltf.buffers = vec![buffer];
        gltf.buffer_views = views;
    }

    pub fn process(&mut self) -> Result<(), Box<dyn Error>> {
        let mut gltf = json::Root {
            asset: json::Asset {
                generator: Some(format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))),
                ..Default::default()
            },
            ..Default::default()
        };

        self.map_objects();

        let children = self.find_node_children();
        let root_nodes = self.get_root_nodes(&children);

        let image_indices = self.process_textures(&mut gltf);
        let mat_indices = self.process_materials(&mut gltf, &image_indices);

        let scene_nodes = root_nodes
            .into_iter()
            .map(|n| self.process_node(&mut gltf, n, &children, 0))
            .collect::<Vec<_>>();

        gltf.scene = Some(json::Index::new(0));
        gltf.scenes = vec![
            json::Scene {
                extensions: None,
                extras: None,
                name: None,
                nodes: scene_nodes
                    .into_iter()
                    .map(|i| json::Index::new(i as u32))
                    .collect(),
            }
        ];

        let mut acc_builder = AccessorBuilder::new();
        let joint_indices = self.find_skins(&mut gltf, &mut acc_builder);

        let mesh_indices = self.process_meshes(&mut gltf, &mut acc_builder, &mat_indices, &joint_indices);

        self.final_process_nodes(&mut gltf, &mesh_indices, &joint_indices);

        // Write binary data
        self.build_binary(&mut gltf, acc_builder);
        //self.process_accessor_data(&mut gltf);

        self.gltf = gltf;

        /*self.gltf = json::Root {
            images,
            nodes: map_bones_to_nodes(dir_name, &bones),
            scene: Some(json::Index::new(0)),
            scenes: vec![
                json::Scene {
                    extensions: None,
                    extras: None,
                    name: None,
                    nodes: vec![json::Index::new(0)],
                }
            ],
            skins: vec![
                json::Skin {
                    extensions: None,
                    extras: None,
                    inverse_bind_matrices: None,
                    joints: joints,
                    name: None,
                    skeleton: Some(json::Index::new(0))
                }
            ],
            ..Default::default()
        };*/

        Ok(())
    }

    pub fn save_to_fs<T: AsRef<Path>>(&self, output_dir: T) -> Result<(), Box<dyn Error>> {
        let output_dir = output_dir.as_ref();

        super::create_dir_if_not_exists(output_dir)?;

        // TODO: Replace
        /*let (obj_dir, sys_info) = self
            .object_dirs
            .iter()
            .map(|(o, _, info)| (o.as_ref(), info))
            .next()
            .unwrap();

        export_object_dir_to_gltf(obj_dir, output_dir, sys_info);*/

        // Write gltf json
        let writer = std::fs::File::create(output_dir.join(format!("test.gltf"))).expect("I/O error");
        json::serialize::to_writer_pretty(writer, &self.gltf).expect("Serialization error");

        Ok(())
    }

    fn find_node_children<'a>(&'a self) -> HashMap<&'a str, Vec<&'a str>> {
        // Use gh1-style child hierarchy first
        /*let (legacy_node_map, legacy_children) = self.transforms
            .values()
            .map(|t| &t.object as &dyn Trans)
            .chain(self.groups.values().map(|g| &g.object as &dyn Trans))
            .chain(self.meshes.values().map(|m| &m.object as &dyn Trans))
            .filter(|t| !t.get_trans_objects().is_empty())
            .fold((HashMap::new(), HashSet::new()), |(mut map, mut ch_set), t| {
                let parent = t.get_name().as_str();
                let children = t.get_trans_objects()
                    .iter()
                    .map(|c| c.as_str())
                    .collect::<Vec<_>>();

                for child in children.iter() {
                    ch_set.insert(*child);
                }

                if t.get_name() != t.get_parent() {
                    //println!("WARN: Object \"{}\", doesn't match object \"{}\"", t.get_name(), t.get_parent());
                    println!("{} : {}", t.get_name(), t.get_parent());
                }

                map.insert(parent, children);
                (map, ch_set)
            });*/

        let mut node_map = self.transforms
            .values()
            .map(|t| &t.object as &dyn Trans)
            .chain(self.groups.values().map(|g| &g.object as &dyn Trans))
            .chain(self.meshes.values().map(|m| &m.object as &dyn Trans))
            .fold(HashMap::new(), |mut acc, b| {
                if b.get_parent().eq(b.get_name()) || b.get_parent().is_empty() {
                    // If bone references self, ignore
                    return acc;
                }

                let parent = b.get_parent().as_str();
                let name = b.get_name().as_str();

                acc
                    .entry(parent)
                    .and_modify(|e: &mut Vec<&'a str>| e.push(name))
                    .or_insert_with(|| vec![name]);

                acc
            });

        // Sort children
        node_map.values_mut().for_each(|ch| ch.sort());
        node_map
    }

    fn get_root_nodes<'a>(&'a self, node_map: &HashMap<&'a str, Vec<&'a str>>) -> Vec<&'a str> {
        let children = node_map
            .values()
            .flatten()
            .map(|s| *s)
            .collect::<HashSet<_>>();

        // Anything not in child map is considered root
        self.dirs_rc
            .iter()
            .map(|d| match &d.as_ref().dir {
                ObjectDir::ObjectDir(dir) => dir.name.as_str()
            })
            .chain(self.transforms.values().map(|t| t.object.get_name().as_str()))
            .chain(self.groups.values().map(|g| g.object.get_name().as_str()))
            .chain(self.meshes.values().map(|m| m.object.get_name().as_str()))
            .filter(|s| !s.is_empty() && !children.contains(s))
            .sorted()
            .collect()
    }
}

fn align_to_multiple_of_four(n: usize) -> usize {
    (n + 3) & !3
}

struct AccessorBuilder {
    // Key = stride, Value = (idx, data)
    working_data: HashMap<usize, (usize, Vec<u8>)>,
    accessors: Vec<json::Accessor>,
}

impl AccessorBuilder {
    fn new() -> AccessorBuilder {
        AccessorBuilder {
            working_data: Default::default(),
            accessors: Vec::new()
        }
    }

    fn calc_stride<const N: usize, T: ComponentValue>(&self) -> usize {
        N * T::size()
    }

    fn update_buffer_view<const N: usize, T: ComponentValue>(&mut self, mut data: Vec<u8>) -> (usize, usize) {
        let stride = self.calc_stride::<N, T>();
        let data_size = data.len();
        let next_idx = self.working_data.len();

        // Upsert buffer data
        let (idx, buff) = self.working_data
            .entry(stride)
            .and_modify(|(_, b)| b.append(&mut data))
            .or_insert_with(|| (next_idx, data));

        // Return index of updated buffer view + insert offset
        (*idx, buff.len() - data_size)
    }

    pub fn add_scalar<S: Into<String>, T: ComponentValue, U: IntoIterator<Item = T>>(&mut self, name: S, data: U) -> Option<usize> {
        // Map to iter of single-item arrays (definitely hacky)
        self.add_array(name, data.into_iter().map(|d| [d]))
    }

    pub fn add_array<const N: usize, S: Into<String>, T: ComponentValue, U: IntoIterator<Item = V>, V: Into<[T; N]>>(&mut self, name: S, data: U) -> Option<usize> {
        let comp_type = T::get_component_type();

        let acc_type = match N {
            1 => json::accessor::Type::Scalar,
            2 => json::accessor::Type::Vec2,
            3 => json::accessor::Type::Vec3,
            4 => json::accessor::Type::Vec4,
            9 => json::accessor::Type::Mat3,
            16 => json::accessor::Type::Mat4,
            _ => unimplemented!()
        };

        // Write to stream and find min/max values
        let mut data_stream = Vec::new();
        let (count, min, max) = data
            .into_iter()
            .fold((0usize, [T::max(); N], [T::min(); N]), |(count, mut min, mut max), item| {
                let mut i = 0;
                for v in item.into() {
                    // Encode + append each value to master buffer
                    data_stream.append(&mut v.encode());

                    // Calc min + max values
                    min[i] = min[i].get_min(v);
                    max[i] = max[i].get_max(v);

                    i += 1;
                }

                (count + 1, min, max)
            });

        if count == 0 {
            // If count is 0, don't bother adding
            return None;
        }

        // Update buffer views
        let (buff_idx, buff_off) = self.update_buffer_view::<N, T>(data_stream);

        let acc_index = self.accessors.len();

        let (min_value, max_value) = Self::get_min_max_values(
            &acc_type,
            min,
            max
        ).unwrap();

        // Create accessor
        let accessor = json::Accessor {
            buffer_view: Some(json::Index::new(buff_idx as u32)),
            byte_offset: buff_off as u32,
            count: count as u32,
            component_type: json::validation::Checked::Valid(json::accessor::GenericComponentType(comp_type)),
            extensions: None,
            extras: None,
            type_: json::validation::Checked::Valid(acc_type),
            min: Some(min_value),
            max: Some(max_value),
            name: match name.into() {
                s if !s.is_empty() => Some(s),
                _ => None
            },
            normalized: false,
            sparse: None
        };

        self.accessors.push(accessor);
        Some(acc_index)
    }

    fn generate_buffer_views(&mut self) -> (Vec<json::buffer::View>, Vec<u8>) {
        // Get view info and sort by assigned index
        let view_data = self.working_data
            .drain()
            .map(|(k, (idx, data))| (idx, k, data)) // (idx, stride, data)
            .sorted_by(|(a, ..), (b, ..)| a.cmp(b));

        let mut views = Vec::new();
        let mut all_data = Vec::new();

        for (_idx, stride, mut data) in view_data {
            // Pad buffer view if required
            let padded_size = align_to_multiple_of_four(data.len());
            if padded_size > data.len() {
                let diff_size = padded_size - data.len();
                data.append(&mut vec![0u8; diff_size]);
            }

            let data_size = data.len();
            let data_offset = all_data.len();

            // Move data from view to full buffer
            all_data.append(&mut data);

            views.push(json::buffer::View {
                name: None,
                byte_length: data_size as u32,
                byte_offset: Some(data_offset as u32),
                byte_stride: match stride % 4 {
                    0 => Some(stride as u32),
                    _ => None // Don't encode if not multiple
                },
                buffer: json::Index::new(0),
                target: None,
                extensions: None,
                extras: None
            });
        }

        (views, all_data)
    }

    fn generate<T: Into<String>>(mut self, name: T) -> (Vec<json::Accessor>, Vec<json::buffer::View>, json::Buffer, Vec<u8>) {
        // Generate buffer views + final buffer blob
        let (views, buffer_data) = self.generate_buffer_views();

        // Create buffer json
        let buffer = json::Buffer {
            name: None,
            byte_length: buffer_data.len() as u32,
            uri: match name.into() {
                s if !s.is_empty() => Some(s),
                _ => None
            },
            extensions: None,
            extras: None
        };

        // Return everything
        (self.accessors,
            views,
            buffer,
            buffer_data)
    }

    fn get_min_max_values<const N: usize, T: ComponentValue>(acc_type: &json::accessor::Type, min: [T; N], max: [T; N]) -> Option<(json::Value, json::Value)> {
        let result = match acc_type {
            json::accessor::Type::Scalar => (
                json::serialize::to_value([min.iter().fold(T::max(), |acc, m| acc.get_min(*m))]),
                json::serialize::to_value([max.iter().fold(T::min(), |acc, m| acc.get_max(*m))]),
            ),
            _ => (
                json::serialize::to_value(min.to_vec()),
                json::serialize::to_value(max.to_vec()),
            ),
        };

        match result {
            (Ok(min), Ok(max)) => Some((min, max)),
            _ => None
        }
    }
}

trait ComponentValue : Copy + Serialize {
    fn min() -> Self;
    fn max() -> Self;

    fn get_min(self, other: Self) -> Self;
    fn get_max(self, other: Self) -> Self;

    fn encode(self) -> Vec<u8>;
    fn get_component_type() -> json::accessor::ComponentType;

    fn size() -> usize {
        std::mem::size_of::<Self>()
    }
}

impl ComponentValue for u16 {
    fn min() -> Self {
        u16::MIN
    }

    fn max() -> Self {
        u16::MAX
    }

    fn get_min(self, other: Self) -> Self {
        std::cmp::min(self, other)
    }

    fn get_max(self, other: Self) -> Self {
        std::cmp::max(self, other)
    }

    fn encode(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn get_component_type() -> json::accessor::ComponentType {
        json::accessor::ComponentType::U16
    }
}

impl ComponentValue for f32 {
    fn min() -> Self {
        f32::MIN
    }

    fn max() -> Self {
        f32::MAX
    }

    fn get_min(self, other: Self) -> Self {
        f32::min(self, other)
    }

    fn get_max(self, other: Self) -> Self {
        f32::max(self, other)
    }

    fn encode(self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn get_component_type() -> json::accessor::ComponentType {
        json::accessor::ComponentType::F32
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::*;

    #[rstest]
    fn accessor_builder_test() {
        let mut acc_builder = AccessorBuilder::new();

        //acc_builder.add_array_f32([[0.0f32, 0.1f32, 0.2f32]]);

        acc_builder.add_array("", [[0.0f32, 0.1f32, 0.2f32]]);
        //acc_builder.add("", [0.0f32, 0.1f32, 0.2f32]);

        //assert!(false);
    }
}