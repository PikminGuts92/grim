use super::*;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TPLEncoding {
    RGBA8 = 64, //  6
    CMP   = 72, // 14
}

impl Default for TPLEncoding {
    fn default() -> TPLEncoding {
        TPLEncoding::CMP
    }
}

impl From<u32> for TPLEncoding {
    fn from(num: u32) -> TPLEncoding {
        match num {
            64 => TPLEncoding::RGBA8,
            72 => TPLEncoding::CMP,
            // Default
            _ => TPLEncoding::CMP,
        }
    }
}

pub fn decode_tpl_image(tpl_img: &[u8], rgba: &mut [u8], width: u32, encoding: TPLEncoding) {
    match &encoding {
        TPLEncoding::RGBA8 => todo!("Implement wii RGBA8 texture decoding"),
        TPLEncoding::CMP => decode_cmp_image(tpl_img, rgba, width),
    };
}

fn decode_cmp_image(tpl_img: &[u8], rgba: &mut [u8], width: u32) {
    let bpp = get_tpl_bpp(&TPLEncoding::RGBA8);

    // Get block counts
    let block_x = width >> 2;
    let block_y = calculate_texture_height(tpl_img.len(), width, bpp) >> 2;
    let block_size = ((16 * bpp) / 8) as usize;

    let mut packed_0;
    let mut packed_1;

    let mut color_0 = [0u8; 4];
    let mut color_1 = [0u8; 4];
    let mut color_2 = [0u8; 4];
    let mut color_3 = [0u8; 4];

    let mut indices = [0u8; 16];

    let mut i = 0usize; // Block index
    let mut x;
    let mut y;

    for by in 0..block_y {
        for bx in 0..block_x {
            if (bx >> 1) & 1 == 1 {
                x = by << 1;
                y = (bx ^ 1) << 1;
            } else {
                x = bx << 2;
                y = by << 2;
            }

            // Read packed bytes (Wii is reverse endian compared to DXT)
            packed_0 = read_as_u16_be(&tpl_img[i..(i + 2)]);
            packed_1 = read_as_u16_be(&tpl_img[(i + 2)..(i + 4)]);

            // Unpack colors to rgba
            unpack_rgb565(packed_0, &mut color_0);
            unpack_rgb565(packed_1, &mut color_1);

            // Interpolate other colors
            if packed_0 > packed_1 {
                // 4 colors
                mix_colors_66_33(&color_0, &color_1, &mut color_2);
                mix_colors_66_33(&color_1, &color_0, &mut color_3);
            } else {
                // 3 colors + transparent
                mix_colors_50_50(&color_0, &color_1, &mut color_2);
                zero_out(&mut color_3);
            }

            // Unpack color indicies
            unpack_indices_be(&tpl_img[(i + 4)..(i + 8)], &mut indices);

            // Copy colors to pixel data
            let colors = [&color_0, &color_1, &color_2, &color_3];
            copy_unpacked_pixels(rgba, &colors, &indices, x, y, width);

            i += block_size;
        }
    }
}

fn get_tpl_bpp(encoding: &TPLEncoding) -> u32 {
    match encoding {
        TPLEncoding::RGBA8 =>  4,
        TPLEncoding::CMP => 32,
    }
}