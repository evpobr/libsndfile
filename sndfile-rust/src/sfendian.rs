pub unsafe fn psf_get_be24(ptr: *const u8, offset: isize) -> i32 {
    let mut value = ((ptr.offset(offset).read() as u32) << 24) as i32;
    value += (ptr.offset(offset + 1).read() as i32) << 16;
    value += (ptr.offset(offset + 2).read() as i32) << 8;
    value
}

pub unsafe fn psf_get_le24(ptr: *const u8, offset: isize) -> i32 {
    let mut value = ((ptr.offset(offset + 2).read() as u32) << 24) as i32;
    value += (ptr.offset(offset + 1).read() as i32) << 16;
    value += (ptr.offset(offset).read() as i32) << 8;
    value
}

pub fn endswap_short_array(ptr: &mut [i16]) {
    ptr.iter_mut().for_each(|p| *p = p.swap_bytes());
}

pub fn endswap_short_copy(dest: &mut [i16], src: &[i16]) {
    assert_eq!(dest.len(), src.len());

    dest.iter_mut()
        .zip(src.iter().map(|s| s.swap_bytes()))
        .for_each(|(d, s)| *d = s);
}

pub fn endswap_int_array(ptr: &mut [i32]) {
    ptr.iter_mut().for_each(|p| *p = p.swap_bytes());
}

pub fn endswap_int_copy(dest: &mut [i32], src: &[i32]) {
    assert_eq!(dest.len(), src.len());

    dest.iter_mut()
        .zip(src.iter().map(|i| *i as u32))
        .for_each(|(d, s)| *d = s.swap_bytes() as i32);
}

pub fn endswap_f32_int_copy(dest: &mut [f32], src: &[i32]) {
    assert_eq!(dest.len(), src.len());

    dest.iter_mut()
        .zip(src.iter().map(|i| *i as u32))
        .for_each(|(d, s)| *d = f32::from_bits(s.swap_bytes()));
}

pub fn endswap_int_f32_copy(dest: &mut [i32], src: &[f32]) {
    assert_eq!(dest.len(), src.len());

    dest.iter_mut()
        .zip(src)
        .for_each(|(d, s)| *d = s.to_bits().swap_bytes() as i32);
}

pub fn endswap_int64_t_array(ptr: &mut [i64]) {
    ptr.iter_mut().for_each(|p| *p = p.swap_bytes());
}

pub fn endswap_int64_t_copy(dest: &mut [i64], src: &[i64]) {
    assert_eq!(dest.len(), src.len());

    dest.iter_mut()
        .zip(src.iter().map(|s| s.swap_bytes()))
        .for_each(|(d, s)| *d = s);
}

pub fn endswap_float_array(ptr: &mut [f32]) {
    ptr.iter_mut()
        .for_each(|f| *f = f32::from_bits(f.to_bits().swap_bytes()));
}

pub fn endswap_double_array(ptr: &mut [f64]) {
    ptr.iter_mut()
        .for_each(|f| *f = f64::from_bits(f.to_bits().swap_bytes()));
}

pub fn endswap_float_copy(dest: &mut [f32], src: &[f32]) {
    assert_eq!(dest.len(), src.len());

    dest.iter_mut()
        .zip(src)
        .for_each(|(d, s)| *d = f32::from_ne_bytes(s.to_bits().swap_bytes().to_ne_bytes()));
}

pub fn endswap_double_copy(dest: &mut [f64], src: &[f64]) {
    assert_eq!(dest.len(), src.len());

    dest.iter_mut()
        .zip(src)
        .for_each(|(d, s)| *d = f64::from_ne_bytes(s.to_bits().swap_bytes().to_ne_bytes()));
}
