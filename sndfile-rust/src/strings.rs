use std::ffi::{CStr, CString};
use std::{borrow::Cow, ptr};

use libc::*;

use crate::common::*;
use crate::*;

use byte_strings::c_str;

#[no_mangle]
unsafe fn psf_store_string(psf: *mut SF_PRIVATE, str_type: c_int, str: *const c_char) -> c_int {
    assert!(!psf.is_null());
    let psf = &mut *psf;

    if str.is_null() {
        return SFE_STR_BAD_STRING;
    }
    let mut str = CStr::from_ptr(str).to_string_lossy();

    let str_len = str.len();

    /* A few extra checks for write mode. */
    if psf.file.mode == SFM_OPEN_MODE::WRITE || psf.file.mode == SFM_OPEN_MODE::RDWR {
        if psf.strings.flags & SF_STR_ALLOW_START == 0 {
            return SFE_STR_NO_SUPPORT;
        }
        if psf.have_written != 0 && psf.strings.flags & SF_STR_ALLOW_END == 0 {
            return SFE_STR_NO_SUPPORT;
        }
        /* Only allow zero length strings for software. */
        if str_type != SF_STR_SOFTWARE && str_len == 0 {
            return SFE_STR_BAD_STRING;
        }
    }

    /* Find the next free slot in table. */
    let mut k = 0;
    loop {
        /* If we find a matching entry clear it. */
        if psf.strings.data[k].r#type == str_type {
            psf.strings.data[k].r#type = -1;
        }

        if psf.strings.data[k].r#type == 0 {
            break;
        }

        k += 1;
        if k >= SF_MAX_STRINGS {
            return SFE_STR_MAX_COUNT;
        }
    }

    /* Determine flags */
    let mut str_flags = SF_STR_LOCATE_START;
    if psf.file.mode == SFM_OPEN_MODE::RDWR || psf.have_written != SF_FALSE {
        if psf.strings.flags & SF_STR_ALLOW_END == 0 {
            return SFE_STR_NO_ADD_END;
        }
        str_flags = SF_STR_LOCATE_END;
    }

    if k == 0 && psf.strings.storage_used != 0 {
        psf_log_printf(
            psf,
            c_str!("SFE_STR_WEIRD : k == 0 && psf->strings.storage_used != 0\n").as_ptr(),
        );
        return SFE_STR_WEIRD;
    }

    if k != 0 && psf.strings.storage_used == 0 {
        psf_log_printf(
            psf,
            c_str!("SFE_STR_WEIRD : k != 0 && psf->strings.storage_used == 0\n").as_ptr(),
        );
        return SFE_STR_WEIRD;
    }

    /* Special case for the first string. */
    if k == 0 {
        psf.strings.storage_used = 0;
    }

    let new_str: String;
    match str_type {
        SF_STR_SOFTWARE => {
            /* In write mode, want to append libsndfile-version to string. */
            if psf.file.mode == SFM_OPEN_MODE::WRITE || psf.file.mode == SFM_OPEN_MODE::RDWR {
                let package_name = PACKAGE_NAME.to_string_lossy().to_string();
                let package_version = PACKAGE_VERSION.to_string_lossy().to_string();
                if !str.contains(&package_name) {
                    /*
                    	** If the supplied string does not already contain a
                    	** libsndfile-X.Y.Z component, then add it.
                    	*/
                    if str.is_empty() {
                        new_str = format!("{}-{}", package_name, package_version);
                    } else {
                        new_str = format!("{} ({}-{})", str, package_name, package_version);
                    }
                } else {
                    new_str = str.to_string();
                }

                str = Cow::from(new_str);
            };
        }
        SF_STR_TITLE | SF_STR_COPYRIGHT | SF_STR_ARTIST | SF_STR_COMMENT | SF_STR_DATE
        | SF_STR_ALBUM | SF_STR_LICENSE | SF_STR_TRACKNUMBER | SF_STR_GENRE => {}
        _ => {
            psf_log_printf(
                psf,
                c_str!("psf_store_string : SFE_STR_BAD_TYPE\n").as_ptr(),
            );
            return SFE_STR_BAD_TYPE;
        }
    };

    /* Plus one to catch string terminator. */
    let str_len = str.len() + 1;

    if psf.strings.storage_used + str_len + 1 > psf.strings.storage_len {
        let temp = psf.strings.storage;
        let mut newlen = 2 * psf.strings.storage_len + str_len + 1;

        newlen = if newlen < 256 { 256 } else { newlen };

        psf.strings.storage = realloc(temp as *mut c_void, newlen) as *mut c_char;
        if psf.strings.storage.is_null() {
            psf.strings.storage = temp;
            return SFE_MALLOC_FAILED;
        }

        psf.strings.storage_len = newlen;
    }

    psf.strings.data[k].r#type = str_type;
    psf.strings.data[k].offset = psf.strings.storage_used;
    psf.strings.data[k].flags = str_flags as c_int;

    let str = str.to_string();
    let str = CString::new(str).unwrap();
    ptr::copy_nonoverlapping(
        str.as_ptr(),
        psf.strings
            .storage
            .offset(psf.strings.storage_used as isize),
        str_len,
    );
    psf.strings.storage_used += str_len;

    psf.strings.flags |= str_flags;

    0
}

#[no_mangle]
pub(crate) unsafe fn psf_set_string(psf: *mut SF_PRIVATE, str_type: c_int, str: *const c_char) -> c_int {
    assert!(!psf.is_null());
    let psf = &mut *psf;

    if psf.file.mode == SFM_OPEN_MODE::READ {
        return SFE_STR_NOT_WRITE;
    }

    psf_store_string(psf, str_type, str)
}

#[no_mangle]
pub(crate) unsafe fn psf_get_string(psf: *mut SF_PRIVATE, str_type: c_int) -> *const c_char {
    assert!(!psf.is_null());
    let psf = &mut *psf;

    for k in 0..SF_MAX_STRINGS {
        if str_type == psf.strings.data[k].r#type {
            return psf
                .strings
                .storage
                .offset(psf.strings.data[k].offset as isize);
        }
    }

    ptr::null()
}

#[no_mangle]
unsafe fn psf_location_string_count(psf: *const SF_PRIVATE, location: c_int) -> c_int {
    assert!(!psf.is_null());
    let psf = &*psf;

    let mut count = 0;
    for k in 0..SF_MAX_STRINGS {
        if psf.strings.data[k].r#type > 0 && psf.strings.data[k].flags & location != 0 {
            count += 1;
        }
    }

    count
}
