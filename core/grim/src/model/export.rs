use crate::scene::*;
//use grim_traits::scene::*;
use nalgebra as na;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

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
            // TODO: Support GH1 bones
            /*Object::Mesh(m) if m.faces.is_empty()
                => Some((
                    m as &dyn Trans,
                    m.bones
                        .iter()
                        .map(|b| (b.name.as_str(), &b.trans))
                        .collect::<Vec<_>>()
                )),*/
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
        matrix: None,
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
            m.m11, m.m12, m.m13, m.m14,
            m.m21, m.m22, m.m23, m.m24,
            m.m31, m.m32, m.m33, m.m34,
            m.m41, m.m42, m.m43, m.m44
        );

        /*let conv_mat = na::Matrix4::new(
            -1.0,  0.0,  0.0, 0.0,
            0.0,  0.0,  1.0, 0.0,
            0.0,  1.0,  0.0, 0.0,
            0.0,  0.0,  0.0, 1.0,
        );

        // TODO: Apply translation...
        let mat = mat * conv_mat;*/

        //na::Matrix::from

        let node = gltf_json::Node {
            camera: None,
            children: if !child_indices.is_empty() {
                Some(child_indices)
            } else {
                None
            },
            extensions: None,
            extras: None,
            matrix: Some([
                // TODO: Figure out better way to re-map
                // Column-major order...
                mat[0],
                mat[1],
                mat[2],
                mat[3],

                mat[4],
                mat[5],
                mat[6],
                mat[7],

                mat[8],
                mat[9],
                mat[10],
                mat[11],

                mat[12],
                mat[13],
                mat[14],
                mat[15],
            ]),
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

pub fn export_object_dir_to_gltf(obj_dir: &ObjectDir, output_path: &Path) {
    use gltf_json as json;

    super::create_dir_if_not_exists(output_path).unwrap();

    let dir_name = match obj_dir {
        ObjectDir::ObjectDir(base) => base.name.as_str(),
    };

    let bones = find_bones(&obj_dir);
    let nodes = map_bones_to_nodes(dir_name, &bones);

    let joints = nodes
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, _)| json::Index::new(i as u32))
        .collect::<Vec<_>>();

    let root = json::Root {
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