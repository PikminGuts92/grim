use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::scene::*;
use crate::SystemInfo;
use grim_traits::scene::*;
use std::error::Error;

fn is_version_supported(version: u32) -> bool {
    match version {
        55..=56 => true, // TBRB/GDRB
        _ => false
    }
}

impl ObjectReadWrite for MatObject {
    fn load(&mut self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        let mut reader = Box::new(BinaryStream::from_stream_with_endian(stream, info.endian));

        let version = reader.read_uint32()?;
        if !is_version_supported(version) {
            // TODO: Switch to custom error
            panic!("Mat version \"{}\" is not supported!", version);
        }

        load_object(self, &mut reader, info)?;

        self.blend = reader.read_uint32()?.into();
        load_color3(&mut self.color, &mut reader)?;
        self.alpha = reader.read_float32()?;

        self.prelit = reader.read_boolean()?;
        self.use_environ = reader.read_boolean()?;

        self.z_mode = reader.read_uint32()?.into();
        self.alpha_cut = reader.read_boolean()?;
        self.alpha_threshold = reader.read_uint32()?;
        self.alpha_write = reader.read_boolean()?;

        self.tex_gen = reader.read_uint32()?.into();
        self.tex_wrap = reader.read_uint32()?.into();

        load_matrix(&mut self.tex_xfm, &mut reader)?;
        self.diffuse_tex = reader.read_prefixed_string()?;

        self.next_pass = reader.read_prefixed_string()?;
        self.intensify = reader.read_boolean()?;
        self.cull = reader.read_boolean()?;

        self.emissive_multiplier = reader.read_float32()?;
        load_color3(&mut self.specular_rgb, &mut reader)?;
        self.specular_power = reader.read_float32()?;

        self.normal_map = reader.read_prefixed_string()?;
        self.emissive_map = reader.read_prefixed_string()?;
        self.specular_map = reader.read_prefixed_string()?;
        self.environ_map = reader.read_prefixed_string()?;

        if version <= 55 {
            self.per_pixel_lit = match reader.read_boolean()? {
                false => PerPixel::kPerPixelOff,
                _ => PerPixel::kPerPixelAllNgPlatforms,
            }
        } else {
            self.per_pixel_lit = reader.read_uint32()?.into();
        }
        self.stencil_mode = reader.read_uint32()?.into();

        self.fur = reader.read_prefixed_string()?;
        self.de_normal = reader.read_float32()?;
        self.anisotropy = reader.read_float32()?;

        self.norm_detail_tiling = reader.read_float32()?;
        self.norm_detail_strength = reader.read_float32()?;
        self.norm_detail_map = reader.read_prefixed_string()?;

        self.point_lights = reader.read_boolean()?;
        self.proj_lights = reader.read_boolean()?;
        self.fog = reader.read_boolean()?;
        self.fade_out = reader.read_boolean()?;
        self.color_adjust = reader.read_boolean()?;

        load_color3(&mut self.rim_rgb, &mut reader)?;
        self.rim_power = reader.read_float32()?;
        self.rim_map = reader.read_prefixed_string()?;
        self.rim_always_show = reader.read_boolean()?;

        self.screen_aligned = reader.read_boolean()?;
        self.shader_variation = reader.read_uint32()?.into();

        load_color3(&mut self.specular2_rgb, &mut reader)?;
        self.specular2_power = reader.read_float32()?;

        self.unknown_1 = reader.read_float32()?;
        self.unknown_2 = reader.read_float32()?;
        self.unknown_3 = reader.read_float32()?;
        self.unknown_4 = reader.read_float32()?;
        self.unknown_5 = reader.read_float32()?;

        self.alpha_mask = reader.read_prefixed_string()?;
        self.ps3_force_trilinear = reader.read_boolean()?;

        Ok(())
    }

    fn save(&self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}