use crate::io::{BinaryStream, SeekFrom, Stream};
use crate::scene::*;
use crate::SystemInfo;
use grim_traits::scene::*;
use std::error::Error;

fn is_version_supported(version: u32) -> bool {
    match version {
        27 | 28 => true, // GH2/GH2 360
        41 | 47 => true, // RB1/RB2
        55 | 56 => true, // TBRB/GDRB
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

        if version > 37 {
            self.alpha_threshold = reader.read_uint32()?;
        }
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

        if version < 51 {
            // Seems to match diffuse_tex
            let _some_string = reader.read_prefixed_string()?;
            /*if !some_string.is_empty() {
                panic!("Some string is note empty with value: {}", &some_string);
            }*/
        }

        self.environ_map = reader.read_prefixed_string()?;

        // TODO: Figure out what this does
        /*if version < 37 {

        }*/

        if version > 25 {
            if version <= 55 {
                self.per_pixel_lit = match reader.read_boolean()? {
                    false => PerPixel::kPerPixelOff,
                    _ => PerPixel::kPerPixelAllNgPlatforms,
                }
            } else {
                self.per_pixel_lit = reader.read_uint32()?.into();
            }
        }

        if version >= 27 && version < 50 {
            // Ignore bool
            reader.read_boolean()?;
        }

        if version > 27 {
            self.stencil_mode = reader.read_uint32()?.into();
        }

        if version >= 29 && version < 41 {
            // Ignore string
            reader.read_prefixed_string()?;
        }

        if version >= 33 {
            self.fur = reader.read_prefixed_string()?;
        } else if version > 29 {
            // TODO: Load fur from "{mat_base_name}.fur" file
        }

        if version >= 34 && version < 49 {
            // Ignore bool, color, alpha
            reader.seek(SeekFrom::Current(17))?;

            if version > 34 {
                let some_string = reader.read_prefixed_string()?;
                if !some_string.is_empty() {
                    panic!("Some string is note empty with value: {}", &some_string);
                }
            }
        }

        if version > 35 {
            self.de_normal = reader.read_float32()?;
            self.anisotropy = reader.read_float32()?;
        }

        if version > 38 {
            if version < 42 {
                // Ignore bool
                reader.read_boolean()?;
            }

            self.norm_detail_tiling = reader.read_float32()?;
            self.norm_detail_strength = reader.read_float32()?;

            if version < 42 {
                // Ignore 5 floats
                reader.seek(SeekFrom::Current(20))?;
            }

            self.norm_detail_map = reader.read_prefixed_string()?;

            if version < 42 {
                let some_string = reader.read_prefixed_string()?;
                if !some_string.is_empty() {
                    panic!("Some string is note empty with value: {}", &some_string);
                }
            }

            // TODO: Figure out if this logic is right
            /*if version < 40 {
                self.norm_detail_strength = 0.0;
            }*/
        }

        if version > 42 {
            if version < 45 {
                let bitfield = reader.read_uint32()?;
                todo!("Parse point_lights from {}", bitfield);
            } else {
                self.point_lights = reader.read_boolean()?;
            }

            self.proj_lights = reader.read_boolean()?;
            self.fog = reader.read_boolean()?;
            self.fade_out = reader.read_boolean()?;

            if version > 46 {
                self.color_adjust = reader.read_boolean()?;
            }
        }

        if version > 47 {
            load_color3(&mut self.rim_rgb, &mut reader)?;
            self.rim_power = reader.read_float32()?;
            self.rim_map = reader.read_prefixed_string()?;
            self.rim_always_show = reader.read_boolean()?;
        }

        if version > 48 {
            self.screen_aligned = reader.read_boolean()?;
        }

        if version == 50 {
            let legacy_shader_variation = reader.read_uint8()?;

            self.shader_variation = match legacy_shader_variation {
                0 => ShaderVariation::kShaderVariationNone,
                _ => ShaderVariation::kShaderVariationSkin,
            };
        } else if version > 50 {
            self.shader_variation = reader.read_uint32()?.into();
            load_color3(&mut self.specular2_rgb, &mut reader)?;
            self.specular2_power = reader.read_float32()?;
        }

        if version > 51 {
            if version < 53 {
                // Ignore bool
                reader.read_boolean()?;
            } else {
                self.unknown_1 = reader.read_float32()?;
            }

            if version > 54 {
                self.unknown_2 = reader.read_float32()?;
                self.unknown_3 = reader.read_float32()?;
                self.unknown_4 = reader.read_float32()?;
                self.unknown_5 = reader.read_float32()?;
            }
        }

        if version > 53 {
            self.alpha_mask = reader.read_prefixed_string()?;
        }

        if version > 54 {
            self.ps3_force_trilinear = reader.read_boolean()?;
        }

        Ok(())
    }

    fn save(&self, stream: &mut dyn Stream, info: &SystemInfo) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}