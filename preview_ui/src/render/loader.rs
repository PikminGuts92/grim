use grim::{Platform, SystemInfo};
use grim::io::*;
use grim::scene::{GroupObject, MatObject, Matrix, MeshObject, Milo, MiloObject, Object, ObjectDir, PackedObject, RndMesh, Tex, Trans, TransConstraint};

use itertools::*;

use std::collections::HashMap;

pub struct MiloLoader<'a> {
    milo: &'a ObjectDir,
    objects: HashMap<&'a str, &'a Object>,
    groups: HashMap<&'a str, &'a GroupObject>,
    mats: HashMap<&'a str, &'a MatObject>,
    meshes: HashMap<&'a str, &'a MeshObject>,
    textures: HashMap<&'a str, &'a Tex>,
    cached_textures: HashMap<&'a str, (&'a Tex, Vec<u8>, TextureEncoding)>,
    transforms: HashMap<&'a str, &'a dyn Trans>,
}

pub enum TextureEncoding {
    RGBA,
    DXT1,
    DXT5,
    ATI2
}

impl<'a> MiloLoader<'a> {
    pub fn new(milo: &ObjectDir) -> MiloLoader {
        let entries = milo.get_entries();

        let objects = entries
            .iter()
            .fold(HashMap::new(), |mut acc, o| {
                acc.insert(o.get_name(), o);
                acc
            });

        let groups = get_objects_mapped(
            entries,
            |o| match o {
                Object::Group(g) => Some(g),
                _ => None,
            });

        let mats = get_objects_mapped(
            entries,
            |o| match o {
                Object::Mat(m) => Some(m),
                _ => None,
            });

        let meshes = get_objects_mapped(
            entries,
            |o| match o {
                Object::Mesh(m) => Some(m),
                _ => None,
            });

        let textures = get_objects_mapped(
            entries,
            |o| match o {
                Object::Tex(t) => Some(t),
                _ => None,
            });

        let transforms = get_objects_mapped_dyn(
            entries,
            |o| match o {
                Object::Group(grp) => Some(grp as &dyn Trans),
                Object::Mesh(mesh) => Some(mesh as &dyn Trans),
                Object::Trans(trans) => Some(trans as &dyn Trans),
                _ => None,
            });

        MiloLoader {
            milo,
            objects,
            groups,
            mats,
            meshes,
            textures,
            cached_textures: HashMap::new(),
            transforms,
        }
    }

    pub fn get_object(&self, name: &str) -> Option<&'a Object> {
        self.objects
            .get(name)
            .and_then(|o| Some(*o))
    }

    pub fn get_group(&self, name: &str) -> Option<&'a GroupObject> {
        self.groups
            .get(name)
            .and_then(|o| Some(*o))
    }

    pub fn get_mat(&self, name: &str) -> Option<&'a MatObject> {
        self.mats
            .get(name)
            .and_then(|o| Some(*o))
    }

    pub fn get_mesh(&self, name: &str) -> Option<&'a MeshObject> {
        self.meshes
            .get(name)
            .and_then(|o| Some(*o))
    }

    pub fn get_texture(&self, name: &str) -> Option<&'a Tex> {
        self.textures
            .get(name)
            .and_then(|o| Some(*o))
    }

    pub fn get_cached_texture(&self, name: &str) -> Option<&(&'a Tex, Vec<u8>, TextureEncoding)> {
        self.cached_textures.get(name)
    }

    pub fn set_cached_texture(&mut self, name: &str, rgba: Vec<u8>, encoding: TextureEncoding) {
        let tex = self.get_texture(name).unwrap();

        self.cached_textures.insert(tex.get_name().as_str(), (tex, rgba, encoding));
    }

    pub fn get_transform(&self, name: &str) -> Option<&'a dyn Trans> {
        self.transforms
            .get(name)
            .and_then(|o| Some(*o))
    }
}

fn get_objects_mapped<T: MiloObject>(objects: &Vec<Object>, filter: impl Fn(&Object) -> Option<&T>) -> HashMap<&str, &T> {
    objects
        .iter()
        .map(filter)
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .fold(HashMap::new(), |mut acc, o| {
            acc.insert(o.get_name().as_str(), o);
            acc
        })
}

fn get_objects_mapped_dyn<T: MiloObject + ?Sized>(objects: &Vec<Object>, filter: impl Fn(&Object) -> Option<&T>) -> HashMap<&str, &T> {
    objects
        .iter()
        .map(filter)
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .fold(HashMap::new(), |mut acc, o| {
            acc.insert(o.get_name().as_str(), o);
            acc
        })
}