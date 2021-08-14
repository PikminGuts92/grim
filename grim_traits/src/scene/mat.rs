use super::{Color3, Matrix};

#[allow(non_camel_case_types)]
pub enum Blend {
    kBlendDest,
    kBlendSrc,
    kBlendAdd,
    kBlendSrcAlpha,
    kBlendSubtract,
    kBlendMultiply,
    kPreMultAlpha,
}

#[allow(non_camel_case_types)]
pub enum PerPixel {
    kPerPixelOff,
    kPerPixelXbox360Only,
    kPerPixelPs3Only,
    kPerPixelAllNgPlatforms,
}

#[allow(non_camel_case_types)]
pub enum ShaderVariation {
    kShaderVariationNone,
    kShaderVariationSkin,
    kShaderVariationHair,
}

#[allow(non_camel_case_types)]
pub enum StencilMode {
    kStencilIgnore,
    kStencilWrite,
    kStencilTest
}

#[allow(non_camel_case_types)]
pub enum TexGen {
    kTexGenNone,
    kTexGenXfm,
    kTexGenSphere,
    kTexGenProjected,
    kTexGenXfmOrigin,
    kTexGenEnviron,
}

#[allow(non_camel_case_types)]
pub enum TexWrap {
    kTexWrapClamp,
    kTexWrapRepeat,
    kTexBorderBlack,
    kTexBorderWhite,
    kTexWrapMirror,
}

#[allow(non_camel_case_types)]
pub enum ZMode {
    kZModeDisable,
    kZModeNormal,
    kZModeTransparent,
    kZModeForce,
    kZModeDecal,
}

pub trait Mat {
    fn get_blend(&self) -> &Blend;
    fn get_blend_mut(&mut self) -> &mut Blend;
    fn set_blend(&mut self, blend: Blend);

    fn get_color(&self) -> &Color3;
    fn get_color_mut(&mut self) -> &mut Color3;
    fn set_color(&mut self, color: Color3);

    fn get_alpha(&self) -> f32;
    fn set_alpha(&mut self, alpha: f32);

    fn get_prelit(&self) -> bool;
    fn set_prelit(&mut self, prelit: bool);

    fn get_use_environ(&self) -> bool;
    fn set_use_environ(&mut self, use_environ: bool);

    fn get_z_mode(&self) -> &ZMode;
    fn get_z_mode_mut(&mut self) -> &mut ZMode;
    fn set_z_mode(&mut self, z_mode: ZMode);

    fn get_alpha_cut(&self) -> bool;
    fn set_alpha_cut(&mut self, alpha_cut: bool);

    fn get_alpha_threshold(&self) -> u8;
    fn set_alpha_threshold(&mut self, alpha_threshold: u8);

    fn get_alpha_write(&self) -> bool;
    fn set_alpha_write(&mut self, alpha_write: bool);

    fn get_tex_xfm(&self) -> &Matrix;
    fn get_tex_xfm_mut(&mut self) -> &mut Matrix;
    fn set_tex_xfm(&mut self, tex_xfm: Matrix);

    fn get_diffuse_tex(&self) -> &String;
    fn get_diffuse_tex_mut(&mut self) -> &mut String;
    fn set_diffuse_tex(&mut self, diffuse_tex: String);

    fn get_next_pass(&self) -> &String;
    fn get_next_pass_mut(&mut self) -> &mut String;
    fn set_next_pass(&mut self, next_pass: String);

    fn get_intensify(&self) -> bool;
    fn set_intensify(&mut self, intensify: bool);

    fn get_cull(&self) -> bool;
    fn set_cull(&mut self, cull: bool);

    fn get_emissive_multiplier(&self) -> f32;
    fn set_emissive_multiplier(&mut self, emissive_multiplier: f32);

    fn get_specular_rgb(&self) -> &Color3;
    fn get_specular_rgb_mut(&mut self) -> &mut Color3;
    fn set_specular_rgb(&mut self, specular_rgb: Color3);

    fn get_specular_power(&self) -> f32;
    fn set_specular_power(&mut self, specular_power: f32);

    fn get_normal_map(&self) -> &String;
    fn get_normal_map_mut(&mut self) -> &mut String;
    fn set_normal_map(&mut self, normal_map: String);

    fn get_emissive_map(&self) -> &String;
    fn get_emissive_map_mut(&mut self) -> &mut String;
    fn set_emissive_map(&mut self, emissive_map: String);

    fn get_specular_map(&self) -> &String;
    fn get_specular_map_mut(&mut self) -> &mut String;
    fn set_specular_map(&mut self, specular_map: String);

    fn get_environ_map(&self) -> &String;
    fn get_environ_map_mut(&mut self) -> &mut String;
    fn set_environ_map(&mut self, environ_map: String);

    fn get_per_pixel_lit(&self) -> &PerPixel;
    fn get_per_pixel_lit_mut(&mut self) -> &mut PerPixel;
    fn set_per_pixel_lit(&mut self, per_pixel_lit: PerPixel);

    fn get_fur(&self) -> &String;
    fn get_fur_mut(&mut self) -> &mut String;
    fn set_fur(&mut self, fur: String);

    fn get_de_normal(&self) -> f32;
    fn set_de_normal(&mut self, de_normal: f32);

    fn get_anisotropy(&self) -> f32;
    fn set_anisotropy(&mut self, anisotropy: f32);

    fn get_norm_detail_tiling(&self) -> f32;
    fn set_norm_detail_tiling(&mut self, norm_detail_tiling: f32);

    fn get_norm_detail_strength(&self) -> f32;
    fn set_norm_detail_strength(&mut self, norm_detail_strength: f32);

    fn get_norm_detail_map(&self) -> &String;
    fn get_norm_detail_map_mut(&mut self) -> &mut String;
    fn set_norm_detail_map(&mut self, norm_detail_map: String);

    fn get_point_lights(&self) -> bool;
    fn set_point_lights(&mut self, point_lights: bool);

    fn get_proj_lights(&self) -> bool;
    fn set_proj_lights(&mut self, proj_lights: bool);

    fn get_fog(&self) -> bool;
    fn set_fog(&mut self, fog: bool);

    fn get_fade_out(&self) -> bool;
    fn set_fade_out(&mut self, fade_out: bool);

    fn get_color_adjust(&self) -> bool;
    fn set_color_adjust(&mut self, color_adjust: bool);

    fn get_rim_rgb(&self) -> &Color3;
    fn get_rim_rgb_mut(&mut self) -> &mut Color3;
    fn set_rim_rgb(&mut self, rim_rgb: Color3);

    fn get_rim_power(&self) -> f32;
    fn set_rim_power(&mut self, rim_power: f32);

    fn get_rim_map(&self) -> &String;
    fn get_rim_map_mut(&mut self) -> &mut String;
    fn set_rim_map(&mut self, rim_map: String);

    fn get_rim_always_show(&self) -> bool;
    fn set_rim_always_show(&mut self, rim_always_show: bool);

    fn get_screen_aligned(&self) -> bool;
    fn set_screen_aligned(&mut self, screen_aligned: bool);

    fn get_shader_variation(&self) -> &ShaderVariation;
    fn get_shader_variation_mut(&mut self) -> &mut ShaderVariation;
    fn set_shader_variation(&mut self, shader_variation: ShaderVariation);

    fn get_specular2_rgb(&self) -> &Color3;
    fn get_specular2_rgb_mut(&mut self) -> &mut Color3;
    fn set_specular2_rgb(&mut self, specular2_rgb: Color3);

    fn get_specular2_power(&self) -> f32;
    fn set_specular2_power(&mut self, specular2_power: f32);

    fn get_alpha_mask(&self) -> &String;
    fn get_alpha_mask_mut(&mut self) -> &mut String;
    fn set_alpha_mask(&mut self, alpha_mask: String);

    fn get_ps3_force_trilinear(&self) -> bool;
    fn set_ps3_force_trilinear(&mut self, ps3_force_trilinear: bool);
}