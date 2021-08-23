use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::scene::*;
use crate::SystemInfo;
use grim_traits::scene::*;
use std::error::Error;

fn is_version_supported(version: u32) -> bool {
    match version {
        36 => true,
        _ => false
    }
}

impl ObjectReadWrite for MeshObject {
    fn load(&mut self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        let mut reader = Box::new(BinaryStream::from_stream_with_endian(stream, info.endian));

        let version = reader.read_uint32()?;
        if !is_version_supported(version) {
            // TODO: Switch to custom error
            panic!("Mesh version \"{}\" is not supported!", version);
        }

        load_object(self, &mut reader, info)?;
        load_trans(self, &mut reader, info, false)?;
        load_draw(self, &mut reader, info, false)?;

        self.mat = reader.read_prefixed_string()?;
        self.geom_owner = reader.read_prefixed_string()?;

        self.mutable = reader.read_uint32()?;
        self.volume = reader.read_uint32()?.into();

        let bsp = reader.read_uint8()?;
        if bsp != 0 {
            panic!("Expected bsp field to be 0, not \"{}\" in Mesh", bsp);
        }

        let vert_count = reader.read_uint32()?;
        reader.seek(SeekFrom::Current(9))?; // (true, 36, 1)

        self.vertices.clear();
        for _ in 0..vert_count {
            let mut vec = Vertex::default();

            vec.pos.x = reader.read_float32()?;
            vec.pos.y = reader.read_float32()?;
            vec.pos.z = reader.read_float32()?;

            // TODO: Read 16-bit floating-point uvs
            reader.seek(SeekFrom::Current(4))?;

            // TODO: Read 16-bit floating-point normals
            reader.seek(SeekFrom::Current(8))?;

            // Read color?
            vec.color.r = (reader.read_uint8()? as f32) / 255.0;
            vec.color.g = (reader.read_uint8()? as f32) / 255.0;
            vec.color.b = (reader.read_uint8()? as f32) / 255.0;
            vec.color.a = (reader.read_uint8()? as f32) / 255.0;

            // TODO: Figure out what this value is
            reader.seek(SeekFrom::Current(4))?;

            // TODO: Figure out what these values are
            reader.seek(SeekFrom::Current(4))?;

            self.vertices.push(vec);
        }

        let face_count = reader.read_uint32()?;
        self.faces.clear();
        for _ in 0..face_count {
            let mut face = [0u16; 3];

            face[0] = reader.read_uint16()?;
            face[1] = reader.read_uint16()?;
            face[2] = reader.read_uint16()?;

            self.faces.push(face);
        }

        let group_count = reader.read_uint32()?;
        self.face_groups = reader.read_bytes(group_count as usize)?;

        let bone_count = reader.read_uint32()?;
        self.bones.clear();
        for _ in 0..bone_count {
            let mut bone = BoneTrans::default();

            bone.name = reader.read_prefixed_string()?;
            load_matrix(&mut bone.trans, &mut reader)?;

            self.bones.push(bone);
        }

        self.keep_mesh_data = reader.read_boolean()?;

        Ok(())
    }

    fn save(&self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

