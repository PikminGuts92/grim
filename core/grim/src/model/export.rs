use crate::scene::*;
//use grim_traits::scene::*;
use std::collections::{HashMap, HashSet};

//type TransObject = dyn Trans + MiloObject;

pub struct BoneNode<'a> {
    object: &'a dyn Trans,
    children: Vec<BoneNode<'a>>
}

/*fn map_to_bone_nodes<'a>(bones: ) -> Vec<BoneNode<'a>> {

}*/

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
            acc
                .entry(b.get_parent().as_str())
                .and_modify(|e| e.push(*b))
                .or_insert(vec![*b]);

            acc
        });

    let mut root_nodes = Vec::new();

    // Add bones that belong to object dir
    if let Some(children) = child_map.get(dir_name) {

        for child in children {
            let mut node = BoneNode {
                object: *bones.get(child.get_name().as_str()).unwrap(),
                children: Vec::new()
            };

            if let Some(child_objs) = child_map.get(child.get_name().as_str()) {
                
            }
        }

        let mut nodes = Vec::new();

        nodes.push((dir_name, children));

        //let mut node = BoneNode {
        //    object
        //}


    }

    // Add unparented bones


    root_nodes

    // Find bones not referenced by other bones
    // Assume they're root bones
    /*let referenced_bones = bones
        .iter()
        .flat_map(|(_, bones)| bones.iter().map(|(b, _)| *b))
        .collect::<HashSet<_>>();

    let root_indices = bones
        .iter()
        .enumerate()
        .filter(|(_, (b, _))| !referenced_bones.contains(b.get_name().as_str()))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    // Bones, root indicies
    // TODO: Maybe return full mapped obj?
    (bones.into_iter().map(|(b, _)| b).collect(), root_indices)*/
}