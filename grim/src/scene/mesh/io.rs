use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::scene::*;
use crate::SystemInfo;
use grim_traits::scene::*;
use std::error::Error;

fn is_version_supported(version: u32) -> bool {
    match version {
        36..=37 => true, // TBRB/GDRB
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

        self.mutable = reader.read_uint32()?.into();
        self.volume = reader.read_uint32()?.into();

        let bsp = reader.read_uint8()?;
        if bsp != 0 {
            panic!("Expected bsp field to be 0, not \"{}\" in Mesh", bsp);
        }

        let vert_count = reader.read_uint32()?;
        let is_ng = reader.read_boolean()?;

        // If next gen, read stride + 1 constant
        if is_ng {
            reader.seek(SeekFrom::Current(8))?; // (true, 36, 1)
        }

        self.vertices.clear();
        for _ in 0..vert_count {
            let mut vec = Vert::default();

            // Position
            vec.pos.x = reader.read_float32()?;
            vec.pos.y = reader.read_float32()?;
            vec.pos.z = reader.read_float32()?;

            if !is_ng {
                // Normals
                vec.normals.x = reader.read_float32()?;
                vec.normals.y = reader.read_float32()?;
                vec.normals.z = reader.read_float32()?;

                // Weights
                vec.weights[0] = reader.read_float32()?;
                vec.weights[1] = reader.read_float32()?;
                vec.weights[2] = reader.read_float32()?;
                vec.weights[3] = reader.read_float32()?;

                // UVs
                vec.uv.u = reader.read_float32()?;
                vec.uv.v = reader.read_float32()?;

                // Bone indices
                vec.bones[0] = reader.read_uint16()?;
                vec.bones[1] = reader.read_uint16()?;
                vec.bones[2] = reader.read_uint16()?;
                vec.bones[3] = reader.read_uint16()?;

                // Skip unknown data for now
                reader.seek(SeekFrom::Current(16))?;
            } else {
                let uv_check = reader.read_int32()?;

                if uv_check == -1 {
                    // UVs
                    vec.uv.u = reader.read_float16()?.into();
                    vec.uv.v = reader.read_float16()?.into();

                    // Normals
                    vec.normals.x = reader.read_float16()?.into();
                    vec.normals.y = reader.read_float16()?.into();
                    vec.normals.z = reader.read_float16()?.into();

                    // Not sure
                    reader.seek(SeekFrom::Current(6))?;

                    // Bone indices
                    vec.bones[0] = reader.read_uint8()?.into();
                    vec.bones[1] = reader.read_uint8()?.into();
                    vec.bones[2] = reader.read_uint8()?.into();
                    vec.bones[3] = reader.read_uint8()?.into();
                } else {
                    // Read as regular uvs
                    reader.seek(SeekFrom::Current(-4))?;

                    // UVs
                    vec.uv.u = reader.read_float16()?.into();
                    vec.uv.v = reader.read_float16()?.into();

                    // Normals
                    vec.normals.x = reader.read_float16()?.into();
                    vec.normals.y = reader.read_float16()?.into();
                    vec.normals.z = reader.read_float16()?.into();
                    reader.seek(SeekFrom::Current(2))?; // Not sure

                    // Not sure
                    reader.seek(SeekFrom::Current(4))?;

                    // Bone indices
                    vec.bones[0] = reader.read_uint16()?;
                    vec.bones[1] = reader.read_uint16()?;
                    vec.bones[2] = reader.read_uint16()?;
                    vec.bones[3] = reader.read_uint16()?;
                }
            }

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
        if version >= 37 {
            self.set_exclude_from_self_shadow(reader.read_boolean()?);
        }

        Ok(())
    }

    fn save(&self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

