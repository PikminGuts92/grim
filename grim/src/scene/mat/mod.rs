use grim_macros::*;
use grim_traits::scene::*;

#[milo]
pub struct MatObject {
    pub blend: Blend,
    pub color: Color3,
    pub alpha: f32,

    pub prelit: bool,
    pub use_environ: bool,

    pub z_mode: ZMode,
    pub alpha_threshold: u32,
    pub alpha_write: bool,

    pub tex_gen: TexGen,
    pub tex_wrap: TexWrap,

    pub tex_xfm: Matrix,
    pub diffuse_tex: String,

    pub next_pass: String,
    pub intentify: bool,
    pub cull: bool,

    pub emissive_multiplier: f32,
    pub specular_rgb: Color3,
    pub specular_power: f32,

    pub normal_map: String,
    pub emissive_map: String,
    pub specular_map: String,
    pub environ_map: String,

    pub per_pixel_lit: PerPixel,
    pub stencil_mode: StencilMode,

    pub fur: String,
    pub de_normal: f32,
    pub anisotropy: f32,

    pub norm_detail_tiling: f32,
    pub norm_detail_strength: f32,
    pub norm_detail_map: String,

    pub point_lights: bool,
    pub proj_lights: bool,
    pub fog: bool,
    pub fade_out: bool,
    pub color_adjust: bool,

    pub rim_rgb: Color3,
    pub rim_power: f32,
    pub rim_map: String,
    pub rim_always_show: bool,

    pub screen_aligned: bool,
    pub shader_variation: ShaderVariation,

    pub specular2_rgb: Color3,
    pub specular2_power: f32,

    pub unknown_1: f32,
    pub unknown_2: f32,
    pub unknown_3: f32,
    pub unknown_4: f32,
    pub unknown_5: f32,

    pub alpha_mask: String,
    pub ps3_force_trilinear: bool, 
}