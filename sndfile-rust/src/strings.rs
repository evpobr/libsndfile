use std::ptr;

use crate::common::{SF_MAX_STRINGS, SF_PRIVATE};

use libc::{c_char, c_int};

#[no_mangle]
pub unsafe extern "C" fn psf_get_string(psf: *const SF_PRIVATE, str_type: c_int) -> *const c_char {
    assert!(!psf.is_null());

    let psf = &*psf;

    for k in 0..SF_MAX_STRINGS {
        if str_type == psf.strings.data[k].r#type {
            return psf
                .strings
                .storage
                .offset(psf.strings.data[k].offset as isize);
        }
    }

    return ptr::null();
}

#[no_mangle]
pub unsafe extern "C" fn psf_location_string_count(
    psf: *const SF_PRIVATE,
    location: c_int,
) -> c_int {
    assert!(!psf.is_null());

    let psf = &*psf;

    psf.strings
        .data
        .iter()
        .filter(|x| (x.r#type > 0) & (x.flags & location == location))
        .count() as c_int
}
