use std::ffi::CStr;
use std::mem;
use std::ptr;
use std::slice;

use crate::common::*;
use crate::*;

use libc::*;

unsafe fn hash_of_str(s: *const c_char) -> i64 {
    assert!(!s.is_null());

    let s = CStr::from_ptr(s);
    s.to_bytes()
        .iter()
        .fold(0, |marker, ch| marker * (0x7f + ch) as i64)
}

#[no_mangle]
unsafe extern "C" fn psf_get_chunk_iterator(
    psf: *mut SF_PRIVATE,
    marker_str: *const c_char,
) -> *mut SF_CHUNK_ITERATOR {
    assert!(!psf.is_null());

    let psf = &mut *psf;

    let pchk = &mut psf.rchunks;
    let idx = if !marker_str.is_null() {
        psf_find_read_chunk_str(pchk, marker_str)
    } else {
        if pchk.used > 0 {
            0
        } else {
            -1
        }
    };

    if idx < 0 {
        return ptr::null_mut();
    }

    if psf.iterator.is_null() {
        psf.iterator = calloc(1 as size_t, mem::size_of::<SF_CHUNK_ITERATOR>() as size_t)
            as *mut SF_CHUNK_ITERATOR;
        if psf.iterator.is_null() {
            return ptr::null_mut();
        };
    }

    let psf_iterator = &mut *psf.iterator;
    psf_iterator.sndfile = psf as *mut SF_PRIVATE;

    if !marker_str.is_null() {
        let mut hash_bytes = [0u8; 4];
        let marker_str = CStr::from_ptr(marker_str);
        let marker_bytes = marker_str.to_bytes();
        let mut hash_bytes_len = marker_bytes.len();
        if hash_bytes_len > 4 {
            hash_bytes_len = 4
        };
        hash_bytes[0..hash_bytes_len].clone_from_slice(&marker_bytes[0..hash_bytes_len]);
        let mut marker_len = marker_bytes.len();
        if marker_len > 64 {
            marker_len = 64;
        };

        let hash = if marker_len > 4 {
            hash_of_str(marker_str.as_ptr())
        } else {
            u32::from_ne_bytes(hash_bytes) as i64
        };

        psf_iterator.id[0..marker_len]
            .clone_from_slice(slice::from_raw_parts(marker_str.as_ptr(), marker_len));
        psf_iterator.id_size = marker_len as u32;
        psf_iterator.hash = hash;
    }

    psf_iterator.current = idx as u32;

    return psf_iterator;
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

#[no_mangle]
unsafe extern "C" fn psf_find_read_chunk_m32(pchk: *const READ_CHUNKS, marker: u32) -> c_int {
    assert_ne!(pchk.is_null(), true);

    let pchk = &*pchk;
    if pchk.chunks.is_null() || pchk.used == 0 {
        return -1;
    };
    let chunks = slice::from_raw_parts(pchk.chunks, pchk.used as usize);
    chunks
        .iter()
        .position(|chunk| chunk.mark32 == marker)
        .map_or_else(|| -1, |k| k as c_int)
}
