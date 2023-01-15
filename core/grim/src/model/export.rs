use crate::io::*;
use crate::scene::*;
//use grim_traits::scene::*;
use crate::{Platform, SystemInfo};
use itertools::*;
use gltf_json as json;
use nalgebra as na;
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
    let mut indicies = Vec::new();

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
        indicies.push(gltf_json::Index::new((nodes.len() - 1) as u32));
    }

    indicies
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

#[derive(Default)]
pub struct GltfExporter {
    object_dirs: Vec<ObjectDirData>, // TODO: Replace with new milo environment?
    dirs_rc: Vec<Rc<ObjectDirData>>,
    settings: GltfExportSettings,
    groups: HashMap<String, MappedObject<GroupObject>>,
    meshes: HashMap<String, MappedObject<MeshObject>>,
    transforms: HashMap<String, MappedObject<TransObject>>,
    textures: HashMap<String, MappedObject<Tex>>,

    // TODO: Move to nested struct?
    gltf: json::Root
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
                    .or_insert(vec![name]);

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