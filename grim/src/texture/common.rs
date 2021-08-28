pub fn calculate_texture_height(tex_size: usize, width: u32, bpp: u32) -> u32 {
    ((tex_size * 8) / (width * bpp) as usize) as u32
}

pub fn read_as_u16(data: &[u8]) -> u16 {
    (data[0] as u16) | (data[1] as u16) << 8
}

pub fn read_as_u16_be(data: &[u8]) -> u16 {
    (data[1] as u16) | (data[0] as u16) << 8
}

pub fn mix_colors_66_33(color_0: &[u8; 4], color_1: &[u8; 4], mixed: &mut [u8; 4]) {
    // m = c0 66% + c1 33%
    for (i, m) in mixed.iter_mut().take(3).enumerate() {
        *m = (((color_0[i] as u16 * 2) + color_1[i] as u16) / 3) as u8;
    }

    // Set alpha to max
    mixed[3] = 0xff;
}

pub fn mix_colors_50_50(color_0: &[u8; 4], color_1: &[u8; 4], mixed: &mut [u8; 4]) {
    // m = c0 50% + c1 50%
    for (i, m) in mixed.iter_mut().enumerate() {
        *m = (((color_0[i] as u16 * 2) + color_1[i] as u16) / 3) as u8;
    }
}

pub fn unpack_rgb565(c: u16, rgba: &mut [u8; 4]) {
    let c = c as u32;

    rgba[0] = ((((c & 0b1111_1000_0000_0000) << 16) | ((c & 0b1110_0000_0000_0000) << 11)) >> 24) as u8;
    rgba[1] = ((((c & 0b0000_0111_1110_0000) << 13) | ((c & 0b0000_0110_0000_0000) <<  7)) >> 16) as u8;
    rgba[2] = ((((c & 0b0000_0000_0001_1111) << 11) | ((c & 0b0000_0000_0001_1100) <<  6)) >>  8) as u8;
    rgba[3] = 0xff;
}

pub fn unpack_indicies(packed: &[u8], indices: &mut [u8; 16]) {
    for (i, ind) in indices.chunks_mut(4).enumerate() {
        ind[0] =  packed[i] & 0b00_00_00_11;
        ind[1] = (packed[i] & 0b00_00_11_00) >> 2;
        ind[2] = (packed[i] & 0b00_11_00_00) >> 4;
        ind[3] = (packed[i] & 0b11_00_00_00) >> 6;
    }
}

pub fn unpack_indicies_360(packed: &[u8], indices: &mut [u8; 16]) {
    for (i, ind) in indices.chunks_mut(4).enumerate() {
        ind[1] =  packed[i] & 0b00_00_00_11;
        ind[0] = (packed[i] & 0b00_00_11_00) >> 2;
        ind[3] = (packed[i] & 0b00_11_00_00) >> 4;
        ind[2] = (packed[i] & 0b11_00_00_00) >> 6;
    }
}

pub fn unpack_indicies_be(packed: &[u8], indices: &mut [u8; 16]) {
    for (i, ind) in indices.chunks_mut(4).enumerate() {
        ind[3] =  packed[i] & 0b00_00_00_11;
        ind[2] = (packed[i] & 0b00_00_11_00) >> 2;
        ind[1] = (packed[i] & 0b00_11_00_00) >> 4;
        ind[0] = (packed[i] & 0b11_00_00_00) >> 6;
    }
}

pub fn zero_out(color: &mut [u8; 4]) {
    for c in color {
        *c = 0x00;
    }
}

pub fn copy_unpacked_pixels(rgba: &mut [u8], colors: &[&[u8; 4]; 4], indices: &[u8; 16], x: u32, y: u32, width: u32) {
    let x = x as usize;
    let y = y as usize;
    let w = width as usize;

    for (y_i, y_inds) in indices.chunks(4).enumerate() {
        for (x_i, ind) in y_inds.iter().enumerate() {
            let rgba_offset = linear_offset(x + x_i, y + y_i, w);
            rgba[rgba_offset..(rgba_offset + 4)].copy_from_slice(colors[*ind as usize]);
        }
    }
}

fn linear_offset(x: usize, y: usize, w: usize) -> usize {
    (y * (w << 2)) + (x << 2)
}