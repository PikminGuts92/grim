#[derive(Debug, Default)]
pub struct CharBones {
    pub compression: u32, // TODO: Convert to enum?
    pub counts: [u32; 7],
    pub computed_sizes: [u32; 7],
}

#[derive(Debug, Default)]
pub struct CharBonesSamples {
    pub char_bones: CharBones,
}

impl CharBones {
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
        assert_eq!(expected, CharBones::get_type_of(input_name));
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
        let char_bone = CharBones {
            compression: input_compression,
            ..Default::default()
        };

        let result = char_bone.get_type_size(input_idx);
        assert_eq!(expected, result);
    }
}