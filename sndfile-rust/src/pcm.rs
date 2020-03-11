use std::slice;

use libc::{c_int, c_schar, c_short, c_uchar};

type tribyte = c_uchar;

#[no_mangle]
unsafe extern "C" fn sc2s_array(src: *const c_schar, count: c_int, dest: *mut c_short) {
    assert!(!src.is_null());
    assert!(!dest.is_null());
    assert!(count >= 0);

    let count = count as usize;
    let src = slice::from_raw_parts(src, count);
    let dest = slice::from_raw_parts_mut(dest, count);

    for (d, s) in dest.iter_mut().zip(src) {
        *d = ((*s as u16) << 8) as c_short;
    }
}

#[no_mangle]
unsafe extern "C" fn uc2s_array(src: *const c_uchar, count: c_int, dest: *mut c_short) {
    assert!(!src.is_null());
    assert!(!dest.is_null());
    assert!(count >= 0);

    let count = count as usize;
    let src = slice::from_raw_parts(src, count);
    let dest = slice::from_raw_parts_mut(dest, count);

    for (d, s) in dest.iter_mut().zip(src) {
        let (value, _) = (*s as u32).overflowing_sub(0x80);
        *d = (value << 8) as c_short;
    }
}

#[no_mangle]
unsafe extern "C" fn let2s_array(src: *const tribyte, count: c_int, dest: *mut c_short) {
    assert!(!src.is_null());
    assert!(!dest.is_null());
    assert!(count >= 0);

    let count = count as usize;
    let src = slice::from_raw_parts(src, count * 3);
    let dest = slice::from_raw_parts_mut(dest, count);

    for (d, s) in dest.iter_mut().zip(src.chunks_exact(3)) {
        let mut s_bytes = [0u8; 2];
        s_bytes.clone_from_slice(&s[1..=2]);
        *d = u16::from_le_bytes(s_bytes) as c_short;
    }
}

#[no_mangle]
unsafe extern "C" fn bet2s_array(src: *const tribyte, count: c_int, dest: *mut c_short) {
    assert!(!src.is_null());
    assert!(!dest.is_null());
    assert!(count >= 0);

    let count = count as usize;
    let src = slice::from_raw_parts(src, count * 3);
    let dest = slice::from_raw_parts_mut(dest, count);

    for (d, s) in dest.iter_mut().zip(src.chunks_exact(3)) {
        let mut s_bytes = [0u8; 2];
        s_bytes.clone_from_slice(&s[0..=1]);
        *d = u16::from_be_bytes(s_bytes) as c_short;
    }
}

#[no_mangle]
unsafe extern "C" fn lei2s_array(src: *const c_int, count: c_int, dest: *mut c_short) {
    assert!(!src.is_null());
    assert!(!dest.is_null());
    assert!(count >= 0);

    let count = count as usize;
    let src = slice::from_raw_parts(src, count);
    let dest = slice::from_raw_parts_mut(dest, count);

    for (d, s) in dest.iter_mut().zip(src) {
        let x = u32::from_le(*s as u32);
        *d = (x >> 16) as c_short;
    }
}

#[no_mangle]
unsafe extern "C" fn bei2s_array(src: *const c_int, count: c_int, dest: *mut c_short) {
    let mut count = count as isize;

    count -= 1;
    while count >= 0 {
        let value = c_int::from_be(src.offset(count).read());
        dest.offset(count).write((value >> 16) as c_short);
        count -= 1;
    }
}

#[no_mangle]
unsafe extern "C" fn sc2i_array(src: *const c_schar, count: c_int, dest: *mut c_int) {
    assert!(src.is_null() != true);
    assert!(dest.is_null() != true);
    assert!(count >= 0);

    let count = count as usize;
    let src = slice::from_raw_parts(src, count);
    let dest = slice::from_raw_parts_mut(dest, count);
    for (d, s) in dest.iter_mut().zip(src).take(count) {
        *d = (*s as i32) << 24;
    }
}

#[no_mangle]
unsafe extern "C" fn uc2i_array(src: *const c_uchar, count: c_int, dest: *mut c_int) {
    let mut count = count as isize;

    count -= 1;
    while count >= 0 {
        dest.offset(count)
            .write(src.offset(count).read() as c_int - 128 << 24);
        count -= 1;
    }
}

#[no_mangle]
unsafe extern "C" fn bes2i_array(src: *const c_short, count: c_int, dest: *mut c_int) {
    let mut count = count as isize;

    count -= 1;
    while count >= 0 {
        let value = c_short::from_be(src.offset(count).read()) as c_int;
        dest.offset(count).write(value << 16);
        count -= 1;
    }
}

#[no_mangle]
unsafe extern "C" fn les2i_array(src: *const c_short, count: c_int, dest: *mut c_int) {
    let mut count = count as isize;

    count -= 1;
    while count >= 0 {
        let value = c_short::from_le(src.offset(count).read()) as c_int;
        dest.offset(count).write(value << 16);
        count -= 1;
    }
}
