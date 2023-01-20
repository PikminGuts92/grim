mod io;

use crate::scene::{Quat, Vector3};
use grim_macros::*;
use grim_traits::scene::*;
pub use io::*;

#[derive(Debug, Default)]
pub struct CharBone {
    pub symbol: String, // Bone name + transform property ext
    pub weight: f32,
}

#[derive(Debug, Default)]
pub struct CharBoneSample {
    pub symbol: String, // Bone name
    pub pos: Option<(f32, Vec<Vector3>)>,
    pub quat: Option<(f32, Vec<Quat>)>,
    pub rotz: Option<(f32, Vec<f32>)>,
}

#[derive(Debug)]
pub enum EncodedSamples {
    Compressed(Vec<CharBone>, Vec<Box<[u8]>>), // Raw sample collection of bone transforms
    Uncompressed(Vec<CharBoneSample>) // Collections of samples grouped by bone transforms
}

impl Default for EncodedSamples {
    fn default() -> Self {
        EncodedSamples::Uncompressed(Vec::new())
    }
}

#[derive(Debug, Default)]
pub struct CharBonesSamples { // Sample set
    pub bones: Vec<CharBone>,
    pub compression: u32, // TODO: Convert to enum?
    pub counts: [u32; 7],
    pub computed_sizes: [u32; 7],
    pub computed_flags: u32,

    pub samples: EncodedSamples,
    pub frames: Vec<f32>
}

impl CharBonesSamples {
    pub fn get_type_of<T: AsRef<str>>(name: T) -> u32 {
        let name = name.as_ref();

        let last_dot_idx = name.find('.');
        if last_dot_idx.is_none() {
            return 7;
        }

        let ext = name[last_dot_idx.unwrap()..].to_ascii_lowercase();
        match ext.as_str() {
            ".pos" => 0,
            ".scale" => 1,
            ".quat" => 2,
            ".rotx" => 3,
            ".roty" => 4,
            ".rotz" => 5,
            _ => 7,
        }
    }

    pub fn get_type_size(&self, idx: u32) -> u32 {
        if idx < 2 {
            return if self.compression < 2 { 16 } else { 6 };
        }

        if idx != 2 {
            return if self.compression == 0 { 4 } else { 2 };
        }

        if self.compression > 2 {
            return 4;
        }

        if self.compression == 0 {
            return 16;
        }

        return 8;
    }

    pub fn recompute_sizes(&mut self) {
        self.computed_sizes[0] = 0;

        for i in 0..6 {
            // Next count bleeds into computed sizes
            // In C++ code, the same array was likely shared
            let curr_count = self.counts[i];
            let next_count = self.counts[i + 1];

            let type_size = self.get_type_size(i as u32);

            self.computed_sizes[i + 1] = self.computed_sizes[i] + (next_count - curr_count) * type_size;
        }

        self.computed_flags = (self.computed_sizes.last().unwrap() + 0xF) & 0xFFFF_FFF0;
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::*;

    #[rstest]
    #[case("", 7)]
    #[case("bone", 7)]
    #[case("bone.", 7)]
    #[case("bone.pdf", 7)]
    #[case("bone.pos", 0)]
    #[case("bone.scale", 1)]
    #[case("bone.quat", 2)]
    #[case("bone.rotx", 3)]
    #[case("bone.roty", 4)]
    #[case("bone.rotz", 5)]
    fn char_bones_get_type_of(#[case] input_name: &str, #[case] expected: u32) {
        assert_eq!(expected, CharBonesSamples::get_type_of(input_name));
    }

    #[rstest]
    #[case(0, 0, 16)]
    #[case(0, 1, 16)]
    #[case(0, 2, 16)]
    #[case(0, 3, 4)]
    #[case(0, 4, 4)]
    #[case(0, 5, 4)]
    #[case(0, 6, 4)]
    #[case(1, 0, 16)]
    #[case(1, 1, 16)]
    #[case(1, 2, 8)]
    #[case(1, 3, 2)]
    #[case(1, 4, 2)]
    #[case(1, 5, 2)]
    #[case(1, 6, 2)]
    #[case(2, 0, 6)]
    #[case(2, 1, 6)]
    #[case(2, 2, 8)]
    #[case(2, 3, 2)]
    #[case(2, 4, 2)]
    #[case(2, 5, 2)]
    #[case(2, 6, 2)]
    fn char_bones_get_type_size(#[case] input_compression: u32, #[case] input_idx: u32, #[case] expected: u32) {
        let char_bone = CharBonesSamples {
            compression: input_compression,
            ..Default::default()
        };

        let result = char_bone.get_type_size(input_idx);
        assert_eq!(expected, result);
    }

    #[rstest]
    #[case(1, [0, 1, 1, 22, 22, 22, 32], [0, 16, 16, 184, 184, 184, 204], 208)]
    #[case(2, [0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0], 0)]
    //#[case(2, [0, 27, 27, 37, 37, 37, 37], [0, 216, 216, 352, 352, 352, 352], 352)]
    #[case(2, [0, 36, 36, 53, 53, 53, 53], [0, 216, 216, 352, 352, 352, 352], 352)]
    fn char_bones_recompute_sizes(#[case] input_compression: u32, #[case] input_counts: [u32; 7], #[case] expected_computed_sizes: [u32; 7], #[case] expected_computed_flags: u32) {
        let mut char_bone = CharBonesSamples {
            compression: input_compression,
            counts: input_counts,
            ..Default::default()
        };

        char_bone.recompute_sizes();

        assert_eq!(expected_computed_sizes, char_bone.computed_sizes);
        assert_eq!(expected_computed_flags, char_bone.computed_flags);
    }
}