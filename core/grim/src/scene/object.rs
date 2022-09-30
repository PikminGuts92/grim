use crate::{SystemInfo};
use crate::io::MemoryStream;
use crate::scene::*;
use grim_macros::*;

#[milo]
pub struct ObjectInstance {}

pub enum MiloObject {
    Anim(AnimObject),
    Cam(CamObject),
    CubeTex(CubeTexObject),
    Draw(DrawObject),
    Group(GroupObject),
    Mat(MatObject),
    Mesh(MeshObject),
    Object(ObjectInstance),
    ObjectDir(ObjectDirInstance),
    P9SongPref(P9SongPref),
    PropAnim(PropAnim),
    Tex(Tex),
    Trans(TransObject),
    Packed(PackedObject),
}

#[derive(Debug)]
pub struct PackedObject {
    pub name: String,
    pub object_type: String,
    pub data: Vec<u8>
}

impl MiloObject {
    pub fn get_name(&self) -> &str {
        match self {
            MiloObject::Anim(anim) => &anim.name,
            MiloObject::Cam(cam) => &cam.name,
            MiloObject::CubeTex(cube) => &cube.name,
            MiloObject::Draw(draw) => &draw.name,
            MiloObject::Group(grp) => &grp.name,
            MiloObject::Mat(mat) => &mat.name,
            MiloObject::Mesh(mesh) => &mesh.name,
            MiloObject::Object(obj) => &obj.name,
            MiloObject::ObjectDir(obj_dir) => &obj_dir.name,
            MiloObject::P9SongPref(pref) => &pref.name,
            MiloObject::PropAnim(prop) => &prop.name,
            MiloObject::Tex(tex) => &tex.name,
            MiloObject::Trans(trans) => &trans.name,
            MiloObject::Packed(packed) => &packed.name,
        }
    }

    pub fn get_type(&self) -> &str {
        match self {
            MiloObject::Anim(_) => "Anim",
            MiloObject::Cam(_) => "Cam",
            MiloObject::CubeTex(_) => "CubeTex",
            MiloObject::Draw(_) => "Draw",
            MiloObject::Group(_) => "Group",
            MiloObject::Mat(_) => "Mat",
            MiloObject::Mesh(_) => "Mesh",
            MiloObject::Object(_) => "Object",
            MiloObject::ObjectDir(_) => "ObjectDir",
            MiloObject::P9SongPref(_) => "P9SongPref",
            MiloObject::PropAnim(_) => "PropAnim",
            MiloObject::Tex(_) => "Tex",
            MiloObject::Trans(_) => "Trans",
            MiloObject::Packed(packed) => &packed.object_type,
        }
    }

    pub fn is_packed(&self) -> bool {
        match self {
            MiloObject::Packed(_) => true,
            _ => false
        }
    }

    pub fn pack(&self, info: &SystemInfo) -> Option<MiloObject> {
        if self.is_packed() {
            todo!("Already packed");
        }

        let obj: &dyn ObjectReadWrite  = match &self {
            MiloObject::Anim(obj) => obj,
            MiloObject::Cam(obj) => obj,
            MiloObject::CubeTex(obj) => obj,
            MiloObject::Draw(obj) => obj,
            MiloObject::Group(obj) => obj,
            MiloObject::Mat(obj) => obj,
            MiloObject::Mesh(obj) => obj,
            MiloObject::P9SongPref(obj) => obj,
            MiloObject::PropAnim(obj) => obj,
            MiloObject::Tex(obj) => obj,
            MiloObject::Trans(obj) => obj,
            _ => todo!("Test"),
        };

        let mut data = Vec::new();
        let mut stream = MemoryStream::from_vector_as_read_write(&mut data);

        if obj.save(&mut stream, info).is_err() {
            println!("ERROR: Unable to pack {} with type {}", self.get_name(), self.get_type());
            return None;
        }

        // Return packed object
        Some(MiloObject::Packed(PackedObject {
            name: self.get_name().to_owned(),
            object_type: self.get_type().to_owned(),
            data,
        }))
    }

    pub fn unpack(&self, info: &SystemInfo) -> Option<MiloObject> {
        todo!();

        // TODO: (Refactor)
        /*match self {
            Object::Packed(packed) => {
                match packed.object_type.as_str() {
                    "Anim" => unpack_object(packed, info).map(|o| MiloObject::Anim(o)),
                    "Cam" => unpack_object(packed, info).map(|o| MiloObject::Cam(o)),
                    "CubeTex" => unpack_object(packed, info).map(|o| MiloObject::CubeTex(o)),
                    "Draw" => unpack_object(packed, info).map(|o| MiloObject::Draw(o)),
                    "Group" => unpack_object(packed, info).map(|o| MiloObject::Group(o)),
                    "Mat" => unpack_object(packed, info).map(|o| MiloObject::Mat(o)),
                    "Mesh" => unpack_object(packed, info).map(|o| MiloObject::Mesh(o)),
                    "P9SongPref" => unpack_object(packed, info).map(|o| MiloObject::P9SongPref(o)),
                    "PropAnim" => unpack_object(packed, info).map(|o| MiloObject::PropAnim(o)),
                    "Tex" => {
                        let mut stream = MemoryStream::from_slice_as_read(packed.data.as_slice());

                        // TODO: Update tex to use same io traits
                        match Tex::from_stream(&mut stream, info) {
                            Ok(mut tex) => {
                                tex.name = packed.name.to_owned();
                                Some(Object::Tex(tex))
                            },
                            Err(_) => None,
                        }
                    },
                    "Trans" => unpack_object(packed, info).map(|o| Object::Trans(o)),
                    _ => None
                }
            },
            _ => None
        }*/
    }

    pub fn as_dir(&self) -> Option<&dyn ObjectDir> {
        match self {
            MiloObject::ObjectDir(dir) => Some(dir as &dyn ObjectDir),
            _ => None
        }
    }

    pub fn as_dir_mut(&mut self) -> Option<&mut dyn ObjectDir> {
        match self {
            MiloObject::ObjectDir(dir) => Some(dir as &mut dyn ObjectDir),
            _ => None
        }
    }
}

fn unpack_object<T: Default + Object + ObjectReadWrite>(packed: &PackedObject, info: &SystemInfo) -> Option<T> {
    let mut stream = MemoryStream::from_slice_as_read(packed.data.as_slice());

    let mut obj = T::default();

    if obj.load(&mut stream, info).is_ok() {
        obj.set_name(packed.name.to_owned());
        return Some(obj);
    }

    None
}