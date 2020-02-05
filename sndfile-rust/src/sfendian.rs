use std::slice;

use libc::c_int;

#[no_mangle]
pub unsafe extern "C" fn endswap_short_array(ptr: *mut i16, len: c_int) {
    assert_ne!(ptr.is_null(), true);
    assert!(len >= 0);

    let len = len as usize;
    let ptr = slice::from_raw_parts_mut(ptr, len);

    ptr.iter_mut().take(len).for_each(|p| *p = p.swap_bytes());
}

#[no_mangle]
pub unsafe extern "C" fn endswap_short_copy(dest: *mut i16, src: *const i16, len: c_int) {
    assert_ne!(src.is_null(), true);
    assert_ne!(dest.is_null(), true);
    assert!(len >= 0);

    let len = len as usize;
    let src = slice::from_raw_parts(src, len);
    let dest = slice::from_raw_parts_mut(dest, len);

    dest.iter_mut()
        .zip(src.iter().map(|s| s.swap_bytes()))
        .take(len)
        .for_each(|(d, s)| *d = s);
}

#[no_mangle]
pub unsafe extern "C" fn endswap_int_array(ptr: *mut i32, len: c_int) {
    assert_ne!(ptr.is_null(), true);
    assert!(len >= 0);

    let len = len as usize;
    let ptr = slice::from_raw_parts_mut(ptr, len);

    ptr.iter_mut().take(len).for_each(|p| *p = p.swap_bytes());
}

#[no_mangle]
pub unsafe extern "C" fn endswap_int_copy(dest: *mut i32, src: *const i32, len: c_int) {
    assert_ne!(src.is_null(), true);
    assert_ne!(dest.is_null(), true);
    assert!(len >= 0);

    let len = len as usize;
    let src = slice::from_raw_parts(src, len);
    let dest = slice::from_raw_parts_mut(dest, len);

    dest.iter_mut()
        .zip(src.iter().map(|s| s.swap_bytes()))
        .take(len)
        .for_each(|(d, s)| *d = s);
}

#[no_mangle]
pub unsafe extern "C" fn endswap_int64_t_array(ptr: *mut i64, len: c_int) {
    assert_ne!(ptr.is_null(), true);
    assert!(len >= 0);

    let len = len as usize;
    let ptr = slice::from_raw_parts_mut(ptr, len);

    ptr.iter_mut().take(len).for_each(|p| *p = p.swap_bytes());
}

#[no_mangle]
pub unsafe extern "C" fn endswap_int64_t_copy(dest: *mut i64, src: *const i64, len: c_int) {
    assert_ne!(src.is_null(), true);
    assert_ne!(dest.is_null(), true);
    assert!(len >= 0);

    let len = len as usize;
    let src = slice::from_raw_parts(src, len);
    let dest = slice::from_raw_parts_mut(dest, len);

    dest.iter_mut()
        .zip(src.iter().map(|s| s.swap_bytes()))
        .take(len)
        .for_each(|(d, s)| *d = s);
}

#[no_mangle]
pub unsafe extern "C" fn endswap_float_array(ptr: *mut f32, len: c_int) {
    endswap_int_array(ptr as *mut i32, len);
}

#[no_mangle]
pub unsafe extern "C" fn endswap_double_array(ptr: *mut f64, len: c_int) {
    endswap_int64_t_array(ptr as *mut i64, len);
}

#[no_mangle]
pub unsafe extern "C" fn endswap_float_copy(dest: *mut f32, src: *const f32, len: c_int) {
    endswap_int_copy(dest as *mut i32, src as *const i32, len);
}

#[no_mangle]
pub unsafe extern "C" fn endswap_double_copy(dest: *mut f64, src: *const f64, len: c_int) {
    endswap_int64_t_copy(dest as *mut i64, src as *const i64, len);
}
