use crate::{SystemInfo};
use crate::io::MemoryStream;
use crate::scene::*;

pub enum Object {
    Anim(AnimObject),
    Cam(CamObject),
    CubeTex(CubeTexObject),
    Draw(DrawObject),
    Group(GroupObject),
    Mat(MatObject),
    Mesh(MeshObject),
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

impl Object {
    pub fn get_name(&self) -> &str {
        match self {
            Object::Anim(anim) => &anim.name,
            Object::Cam(cam) => &cam.name,
            Object::CubeTex(cube) => &cube.name,
            Object::Draw(draw) => &draw.name,
            Object::Group(grp) => &grp.name,
            Object::Mat(mat) => &mat.name,
            Object::Mesh(mesh) => &mesh.name,
            Object::Tex(tex) => &tex.name,
            Object::Trans(trans) => &trans.name,
            Object::Packed(packed) => &packed.name,
        }
    }

    pub fn get_type(&self) -> &str {
        match self {
            Object::Anim(_) => "Anim",
            Object::Cam(_) => "Cam",
            Object::CubeTex(_) => "CubeTex",
            Object::Draw(_) => "Draw",
            Object::Group(_) => "Group",
            Object::Mat(_) => "Mat",
            Object::Mesh(_) => "Mesh",
            Object::Tex(_) => "Tex",
            Object::Trans(_) => "Trans",
            Object::Packed(packed) => &packed.object_type,
        }
    }

    pub fn is_packed(&self) -> bool {
        match self {
            Object::Packed(_) => true,
            _ => false
        }
    }

    pub fn pack(&self, info: &SystemInfo) -> Option<Object> {
        if self.is_packed() {
            todo!("Already packed");
        }

        let obj: &dyn ObjectReadWrite  = match &self {
            Object::Anim(obj) => obj,
            Object::Cam(obj) => obj,
            Object::CubeTex(obj) => obj,
            Object::Draw(obj) => obj,
            Object::Group(obj) => obj,
            Object::Mat(obj) => obj,
            Object::Mesh(obj) => obj,
            Object::Tex(obj) => obj,
            Object::Trans(obj) => obj,
            _ => todo!("Test"),
        };

        let mut data = Vec::new();
        let mut stream = MemoryStream::from_vector_as_read_write(&mut data);

        if obj.save(&mut stream, info).is_err() {
            println!("ERROR: Unable to pack {} with type {}", self.get_name(), self.get_type());
            return None;
        }

        // Return packed object
        Some(Object::Packed(PackedObject {
            name: self.get_name().to_owned(),
            object_type: self.get_type().to_owned(),
            data,
        }))
    }

    pub fn unpack(&self, info: &SystemInfo) -> Option<Object> {
        match self {
            Object::Packed(packed) => {
                match packed.object_type.as_str() {
                    "Anim" => unpack_object(packed, info).map(|o| Object::Anim(o)),
                    "Cam" => unpack_object(packed, info).map(|o| Object::Cam(o)),
                    "CubeTex" => unpack_object(packed, info).map(|o| Object::CubeTex(o)),
                    "Draw" => unpack_object(packed, info).map(|o| Object::Draw(o)),
                    "Group" => unpack_object(packed, info).map(|o| Object::Group(o)),
                    "Mat" => unpack_object(packed, info).map(|o| Object::Mat(o)),
                    "Mesh" => unpack_object(packed, info).map(|o| Object::Mesh(o)),
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
        }
    }
}

fn unpack_object<T: Default + MiloObject + ObjectReadWrite>(packed: &PackedObject, info: &SystemInfo) -> Option<T> {
    let mut stream = MemoryStream::from_slice_as_read(packed.data.as_slice());

    let mut obj = T::default();

    if obj.load(&mut stream, info).is_ok() {
        obj.set_name(packed.name.to_owned());
        return Some(obj);
    }

    None
}