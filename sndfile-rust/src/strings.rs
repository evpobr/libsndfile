use crate::common::SF_PRIVATE;

use libc::c_int;

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
