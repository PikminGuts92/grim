use crate::Platform;
use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::scene::*;
use crate::SystemInfo;
use grim_traits::scene::*;
use std::error::Error;

fn is_version_supported(version: u32) -> bool {
    match version {
        25 => true, // GH1
        28 => true, // GH2/GH2 360
        34 => true, // RB1/RB2
        36 | 37 => true, // TBRB/GDRB
        38 => true, // RB3
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
        let mut is_ng = false;

        if version >= 36 {
            is_ng = reader.read_boolean()?;

            // If next gen, read stride + 1 constant
            if is_ng {
                reader.seek(SeekFrom::Current(8))?; // (36, 1) | (40, 2)
            }
        }

        self.vertices.clear();
        self.raw_vertices.clear();
        for _ in 0..vert_count {
            // TODO: Remove once next gen vertex format is figured out
            if version >= 36 && is_ng {
                // Read raw vert data
                let mut raw_vert = [0u8; 36];
                let data = reader.read_bytes(36)?;
                raw_vert.copy_from_slice(data.as_slice());

                reader.seek(SeekFrom::Current(-36))?;
                self.raw_vertices.push(raw_vert);
            }

            let mut vec = Vert::default();

            // Position
            vec.pos.x = reader.read_float32()?;
            vec.pos.y = reader.read_float32()?;
            vec.pos.z = reader.read_float32()?;
            if version == 34 {
                vec.pos.w = reader.read_float32()?;
            }

            if version < 35 || !is_ng {
                if version >= 38 {
                    // Skip extra bytes
                    // TODO: Figure out what this data is...
                    reader.seek(SeekFrom::Current(16))?;
                }

                // Normals
                vec.normals.x = reader.read_float32()?;
                vec.normals.y = reader.read_float32()?;
                vec.normals.z = reader.read_float32()?;
                if version == 34 {
                    vec.normals.w = reader.read_float32()?;
                }

                if version >= 38 {
                    // Packed in different order?
                    // UVs
                    vec.uv.u = reader.read_float32()?;
                    vec.uv.v = reader.read_float32()?;

                    // Weights
                    vec.weights[0] = reader.read_float32()?;
                    vec.weights[1] = reader.read_float32()?;
                    vec.weights[2] = reader.read_float32()?;
                    vec.weights[3] = reader.read_float32()?;
                } else {
                    // Weights
                    vec.weights[0] = reader.read_float32()?;
                    vec.weights[1] = reader.read_float32()?;
                    vec.weights[2] = reader.read_float32()?;
                    vec.weights[3] = reader.read_float32()?;

                    // UVs
                    vec.uv.u = reader.read_float32()?;
                    vec.uv.v = reader.read_float32()?;
                }

                if version >= 34 {
                    // Bone indices
                    vec.bones[0] = reader.read_uint16()?;
                    vec.bones[1] = reader.read_uint16()?;
                    vec.bones[2] = reader.read_uint16()?;
                    vec.bones[3] = reader.read_uint16()?;

                    if version >= 38 {
                        // Skip unknown bytes
                        // TODO: Figure out what this data is...
                        reader.seek(SeekFrom::Current(16))?;
                    } else {
                        // Tangent?
                        vec.tangent.x = reader.read_float32()?;
                        vec.tangent.y = reader.read_float32()?;
                        vec.tangent.z = reader.read_float32()?;
                        vec.tangent.w = reader.read_float32()?;
                    }
                }
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

                if version >= 38 {
                    // Skip extra bytes
                    // TODO: Figure out what this data is...
                    reader.seek(SeekFrom::Current(4))?;
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

        // Read bones
        self.bones.clear();
        if version >= 34 {
            let bone_count = reader.read_uint32()?;

            for _ in 0..bone_count {
                let mut bone = BoneTrans::default();

                bone.name = reader.read_prefixed_string()?;
                load_matrix(&mut bone.trans, &mut reader)?;

                self.bones.push(bone);
            }
        } else {
            // Should be 0 or 4 bones
            let mut bones = Vec::new();

            // Read bone names
            for i in 0..4 {
                let name = reader.read_prefixed_string()?;
                if i == 0 && name.is_empty() {
                    break;
                }

                bones.push(BoneTrans {
                    name,
                    ..Default::default()
                });
            }

            // Read bone transforms
            for mut bone in bones {
                load_matrix(&mut bone.trans, &mut reader)?;

                // Add bone if it has name
                if !bone.name.is_empty() {
                    self.bones.push(bone);
                }
            }
        }

        if version >= 36 {
            self.keep_mesh_data = reader.read_boolean()?;
        }

        if version == 37 {
            self.exclude_from_self_shadow = reader.read_boolean()?;
        } else if version >= 38 {
            self.has_ao_calculation = reader.read_boolean()?;
        }

        // TODO: Parse extra data from previous gen platforms

        Ok(())
    }

    fn save(&self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        let mut stream = Box::new(BinaryStream::from_stream_with_endian(stream, info.endian));

        // TODO: Get version from system info
        let version = 34;
        let is_ng = info.is_next_gen();

        stream.write_uint32(version)?;

        save_object(self, &mut stream, info)?;
        save_trans(self, &mut stream, info, false)?;
        save_draw(self, &mut stream, info, false)?;

        stream.write_prefixed_string(&self.mat)?;
        stream.write_prefixed_string(&self.geom_owner)?;

        stream.write_uint32(self.mutable as u32)?;
        stream.write_uint32(self.volume as u32)?;

        // TODO: Figure out what bsp? Although it's not really used at all...
        stream.write_uint8(0)?;

        stream.write_uint32(self.vertices.len() as u32)?;

        if version >= 36 {
            stream.write_boolean(is_ng)?;

            if is_ng {
                // TODO: Determine if value changes after v37
                let vert_stride = 36;

                stream.write_uint32(vert_stride)?;
                stream.write_uint32(1)?; // Some constant
            }
        }

        // TODO: Remove once next gen vertex format is figured out
        if version >= 36 && is_ng && self.vertices.len() == self.raw_vertices.len() {
            // Write raw vertices
            for raw_vert in self.raw_vertices.iter() {
                stream.write_bytes(raw_vert)?;
            }
        } else {
            // Write vertices
            // TODO: Separate into functions and use conditionals before loop iteration
            for v in &self.vertices {
                // Position
                stream.write_float32(v.pos.x)?;
                stream.write_float32(v.pos.y)?;
                stream.write_float32(v.pos.z)?;
                if version == 34 {
                    stream.write_float32(v.pos.w)?;
                }

                if version < 35 || !is_ng {
                    // Normals
                    stream.write_float32(v.normals.x)?;
                    stream.write_float32(v.normals.y)?;
                    stream.write_float32(v.normals.z)?;
                    if version == 34 {
                        stream.write_float32(v.normals.w)?;
                    }

                    // Weights
                    stream.write_float32(v.weights[0])?;
                    stream.write_float32(v.weights[1])?;
                    stream.write_float32(v.weights[2])?;
                    stream.write_float32(v.weights[3])?;

                    // UVs
                    stream.write_float32(v.uv.u)?;
                    stream.write_float32(v.uv.v)?;

                    if version >= 34 {
                        // Bone indices
                        stream.write_uint16(v.bones[0])?;
                        stream.write_uint16(v.bones[1])?;
                        stream.write_uint16(v.bones[2])?;
                        stream.write_uint16(v.bones[3])?;

                        // Tangent?
                        stream.write_float32(v.tangent.x)?;
                        stream.write_float32(v.tangent.y)?;
                        stream.write_float32(v.tangent.z)?;
                        stream.write_float32(v.tangent.w)?;
                    }
                } else {
                    todo!("Figure out how ng verts are packed in v36 meshes");
                }
            }
        }

        stream.write_uint32(self.faces.len() as u32)?;
        for f in &self.faces {
            stream.write_uint16(f[0])?;
            stream.write_uint16(f[1])?;
            stream.write_uint16(f[2])?;
        }

        stream.write_uint32(self.face_groups.len() as u32)?;
        stream.write_bytes(self.face_groups.as_slice())?;

        if version >= 34 {
            // Write n-bones
            stream.write_uint32(self.bones.len() as u32)?;

            for b in &self.bones {
                stream.write_prefixed_string(&b.name)?;
                save_matrix(&b.trans, &mut stream)?;
            }
        } else {
            if self.bones.is_empty() {
                // Write 0 bones
                stream.write_uint32(0)?;
            } else {
                // Write 4 bones
                for i in 0..4 {
                    if let Some(b) = self.bones.get(i) {
                        // Write bone
                        stream.write_prefixed_string(&b.name)?;
                        save_matrix(&b.trans, &mut stream)?;
                    } else {
                        // Write empty bone
                        stream.write_uint32(0)?;
                        save_matrix(&Matrix::indentity(), &mut stream)?;
                    }
                }

                todo!("Figure out how to calculate additional group info");
            }
        }

        if version >= 36 {
            stream.write_boolean(self.keep_mesh_data)?;
        }

        if version == 37 {
            stream.write_boolean(self.exclude_from_self_shadow)?;
        } else if version >= 38 {
            stream.write_boolean(self.has_ao_calculation)?;
        }

        Ok(())
    }
}

