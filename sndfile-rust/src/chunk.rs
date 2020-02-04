use std::ffi::CStr;
use std::slice;

use crate::common::*;

use libc::*;

unsafe fn hash_of_str(s: *const c_char) -> i64 {
    assert!(!s.is_null());

    let s = CStr::from_ptr(s);
    s.to_bytes()
        .iter()
        .fold(0, |marker, ch| marker * (0x7f + ch) as i64)
}

#[no_mangle]
unsafe extern "C" fn psf_find_read_chunk_str(
    pchk: *const READ_CHUNKS,
    marker_str: *const c_char,
) -> c_int {
    assert!(!pchk.is_null());
    assert!(!marker_str.is_null());

    let marker_str = CStr::from_ptr(marker_str);

    let hash = if marker_str.to_bytes().len() > 4 {
        hash_of_str(marker_str.as_ptr())
    } else {
        let marker_bytes = marker_str.to_bytes();
        let mut hash_bytes = [0u8; 4];
        hash_bytes[0..4].clone_from_slice(&marker_bytes[0..4]);
        u32::from_ne_bytes(hash_bytes) as i64
    };

    let pchk = &*pchk;
    let chunks = slice::from_raw_parts(pchk.chunks, pchk.used as usize);
    chunks
        .iter()
        .position(|chunk| chunk.hash == hash as u64)
        .map_or_else(|| -1, |k| k as c_int)
}
