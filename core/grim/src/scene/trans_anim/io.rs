use crate::dta::{DataArray, RootData, save_array};
use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::scene::*;
use crate::SystemInfo;
use grim_traits::scene::*;
use thiserror::Error as ThisError;
use std::error::Error;

#[derive(Debug, ThisError)]
pub enum TransAnimReadError {
    #[error("TransAnim version of {version} not supported")]
    TransAnimVersionNotSupported {
        version: u32
    },
}

fn is_version_supported(version: u32) -> bool {
    match version {
        6 | 7 => true,
        _ => false
    }
}

impl ObjectReadWrite for TransAnim {
    fn load(&mut self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        let mut reader = Box::new(BinaryStream::from_stream_with_endian(stream, info.endian));

        let version = reader.read_uint32()?;

        // If not valid, return unsupported error
        if !is_version_supported(version) {
            // TODO: Switch to custom error
            return Err(Box::new(TransAnimReadError::TransAnimVersionNotSupported {
                version
            }));
        }

        if version >= 4 {
            load_object(self, &mut reader, info)?;
        }
        load_anim(self, &mut reader, info, false)?;

        self.trans_object = reader.read_prefixed_string()?;

        // Reset keys
        self.rot_keys.clear();
        self.trans_keys.clear();
        self.scale_keys.clear();

        if version != 2 {
            self.rot_keys = load_keys_quat(&mut reader)?;
            self.trans_keys = load_keys_vector3(&mut reader)?;
        }

        self.trans_anim_owner = reader.read_prefixed_string()?;

        if version < 4 {
            // TODO: Parse bitfield?
            self.trans_spline = false;
            reader.seek(SeekFrom::Current(4))?;
        } else {
            self.trans_spline = reader.read_boolean()?;
        }

        self.repeat_trans = reader.read_boolean()?;

        if version < 4 {
            self.scale_spline = false;
            todo!()
        } else {
            self.scale_keys = load_keys_vector3(&mut reader)?;
            self.scale_spline = reader.read_boolean()?;
        }

        if version < 2 {
            self.follow_path = false;
            todo!()
        } else {
            self.follow_path = reader.read_boolean()?;
        }

        if version > 3 {
            self.rot_slerp = reader.read_boolean()?;
        }

        if version > 6 {
            self.rot_spline = reader.read_boolean()?;
        }

        Ok(())
    }

    fn save(&self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        let mut writer = Box::new(BinaryStream::from_stream_with_endian(stream, info.endian));

        // TODO: Get version from system info
        let version = 7;

        writer.write_uint32(version)?;

        save_object(self, &mut writer, info)?;
        save_anim(self, &mut writer, info, false)?;

        todo!()

        //Ok(())
    }
}

fn load_keys_vector3(reader: &mut Box<BinaryStream>) -> Result<Vec<AnimEvent<Vector3>>, Box<dyn Error>> {
    let count = reader.read_uint32()?;
    let mut keys = Vec::new();

    for _ in 0..count {
        let mut value = Vector3::default();
        load_vector3(&mut value, reader)?;

        let pos = reader.read_float32()?;

        keys.push(AnimEvent {
            value,
            pos
        })
    }

    Ok(keys)
}

fn load_keys_quat(reader: &mut Box<BinaryStream>) -> Result<Vec<AnimEvent<Quat>>, Box<dyn Error>> {
    let count = reader.read_uint32()?;
    let mut keys = Vec::new();

    for _ in 0..count {
        let mut value = Quat::default();
        load_quat(&mut value, reader)?;

        let pos = reader.read_float32()?;

        keys.push(AnimEvent {
            value,
            pos
        })
    }

    Ok(keys)
}