use crate::{SystemInfo};
use crate::io::MemoryStream;
use crate::scene::*;
use grim_macros::*;

#[milo]
pub struct ObjectInstance {}

pub enum MiloObject {
    Anim(AnimObject),
    Cam(CamObject),
    CharClipSamples(CharClipSamples),
    CharLipSync(CharLipSync),
    CubeTex(CubeTexObject),
    Draw(DrawObject),
    Group(GroupObject),
    Mat(MatObject),
    Mesh(MeshObject),
    MeshAnim(MeshAnim),
    Morph(Morph),
    Object(ObjectInstance),
    ObjectDir(ObjectDirInstance),
    P9SongPref(P9SongPref),
    PropAnim(PropAnim),
    SynthSample(SynthSample),
    Tex(Tex),
    Trans(TransObject),
    TransAnim(TransAnim),
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
            MiloObject::CharClipSamples(ccs) => &ccs.name,
            MiloObject::CharLipSync(cls) => &cls.name,
            MiloObject::CubeTex(cube) => &cube.name,
            MiloObject::Draw(draw) => &draw.name,
            MiloObject::Group(grp) => &grp.name,
            MiloObject::Mat(mat) => &mat.name,
            MiloObject::Mesh(mesh) => &mesh.name,
            MiloObject::MeshAnim(mesh_anim) => &mesh_anim.name,
            MiloObject::Morph(morph) => &morph.name,
            MiloObject::Object(obj) => &obj.name,
            MiloObject::ObjectDir(obj_dir) => &obj_dir.name,
            MiloObject::P9SongPref(pref) => &pref.name,
            MiloObject::PropAnim(prop) => &prop.name,
            MiloObject::SynthSample(synth) => &synth.name,
            MiloObject::Tex(tex) => &tex.name,
            MiloObject::Trans(trans) => &trans.name,
            MiloObject::TransAnim(trans_anim) => &trans_anim.name,
            MiloObject::Packed(packed) => &packed.name,
        }
    }

    pub fn get_type(&self) -> &str {
        match self {
            MiloObject::Anim(_) => "Anim",
            MiloObject::Cam(_) => "Cam",
            MiloObject::CharClipSamples(_) => "CharClipSamples",
            MiloObject::CharLipSync(_) => "CharLipSync",
            MiloObject::CubeTex(_) => "CubeTex",
            MiloObject::Draw(_) => "Draw",
            MiloObject::Group(_) => "Group",
            MiloObject::Mat(_) => "Mat",
            MiloObject::Mesh(_) => "Mesh",
            MiloObject::MeshAnim(_) => "MeshAnim",
            MiloObject::Morph(_) => "Morph",
            MiloObject::Object(_) => "Object",
            MiloObject::ObjectDir(_) => "ObjectDir",
            MiloObject::P9SongPref(_) => "P9SongPref",
            MiloObject::PropAnim(_) => "PropAnim",
            MiloObject::SynthSample(_) => "SynthSample",
            MiloObject::Tex(_) => "Tex",
            MiloObject::Trans(_) => "Trans",
            MiloObject::TransAnim(_) => "TransAnim",
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
            Object::Anim(obj) => obj,
            Object::Cam(obj) => obj,
            Object::CubeTex(obj) => obj,
            Object::Draw(obj) => obj,
            Object::Group(obj) => obj,
            Object::Mat(obj) => obj,
            Object::Mesh(obj) => obj,
            Object::Morph(obj) => obj,
            Object::P9SongPref(obj) => obj,
            Object::PropAnim(obj) => obj,
            Object::Tex(obj) => obj,
            Object::Trans(obj) => obj,
            Object::TransAnim(obj) => obj,
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
                    "Anim" => unpack_object(packed, info).map(|o| Object::Anim(o)),
                    "Cam" => unpack_object(packed, info).map(|o| Object::Cam(o)),
                    "CharClipSamples" => unpack_object(packed, info).map(|o| Object::CharClipSamples(o)),
                    "CharLipSync" => unpack_object(packed, info).map(|o| Object::CharLipSync(o)),
                    "CubeTex" => unpack_object(packed, info).map(|o| Object::CubeTex(o)),
                    "Draw" => unpack_object(packed, info).map(|o| Object::Draw(o)),
                    "Group" => unpack_object(packed, info).map(|o| Object::Group(o)),
                    "Mat" => unpack_object(packed, info).map(|o| Object::Mat(o)),
                    "Mesh" => unpack_object(packed, info).map(|o| Object::Mesh(o)),
                    "MeshAnim" => unpack_object(packed, info).map(|o| Object::MeshAnim(o)),
                    "Morph" => unpack_object(packed, info).map(|o| Object::Morph(o)),
                    "P9SongPref" => unpack_object(packed, info).map(|o| Object::P9SongPref(o)),
                    "PropAnim" => unpack_object(packed, info).map(|o| Object::PropAnim(o)),
                    "SynthSample" => unpack_object(packed, info).map(|o| Object::SynthSample(o)),
                    "Tex" => {
                        let mut stream = MemoryStream::from_slice_as_read(packed.data.as_slice());

                        // TODO: Update tex to use same io traits
                        match Tex::from_stream(&mut stream, info) {
                            Ok(mut tex) => {
                                tex.name = packed.name.to_owned();
                                Some(MiloObject::Tex(tex))
                            },
                            Err(_) => None,
                        }
                    },
                    "Trans" => unpack_object(packed, info).map(|o| MiloObject::Trans(o)),
                    "TransAnim" => unpack_object(packed, info).map(|o| MiloObject::TransAnim(o)),
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