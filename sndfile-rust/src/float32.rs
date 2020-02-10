use crate::sfendian::*;
use std::mem;
use std::slice;

use libc::*;

use crate::common::*;
use crate::*;

const FLOAT_UNKNOWN: c_int = 0x00;
const FLOAT_CAN_RW_LE: c_int = 0x12;
const FLOAT_CAN_RW_BE: c_int = 0x23;
const FLOAT_BROKEN_LE: c_int = 0x34;
const FLOAT_BROKEN_BE: c_int = 0x45;

#[no_mangle]
unsafe extern "C" fn f2s_array(
    src: *const c_float,
    count: c_int,
    dest: *mut c_short,
    scale: c_float,
) {
    assert_ne!(src.is_null(), true);
    assert_ne!(dest.is_null(), true);
    assert!(count >= 0);

    let count = count as usize;
    let src = slice::from_raw_parts(src, count);
    let dest = slice::from_raw_parts_mut(dest, count);

    dest.iter_mut()
        .zip(src.iter().map(|s| (scale * (*s)).round() as c_short))
        .take(count)
        .for_each(|(d, s)| *d = s);
}

#[no_mangle]
unsafe extern "C" fn f2s_clip_array(
    src: *const c_float,
    count: c_int,
    dest: *mut c_short,
    scale: c_float,
) {
    assert_ne!(src.is_null(), true);
    assert_ne!(dest.is_null(), true);
    assert!(count >= 0);

    let count = count as usize;
    let src = slice::from_raw_parts(src, count);
    let dest = slice::from_raw_parts_mut(dest, count);

    dest.iter_mut().zip(src.iter()).for_each(|(d, s)| {
        let tmp = scale * (*s);

        if !cfg!(cpu_clips_positive) && tmp > 32767.0 {
            *d = std::i16::MAX;
        } else if !cfg!(cpu_clips_negative) && tmp < -32768.0 {
            *d = std::i16::MIN;
        } else {
            *d = tmp.round() as i16;
        };
    });
}

#[no_mangle]
unsafe extern "C" fn f2i_array(
    src: *const c_float,
    count: c_int,
    dest: *mut c_int,
    scale: c_float,
) {
    assert_ne!(src.is_null(), true);
    assert_ne!(dest.is_null(), true);
    assert!(count >= 0);

    let count = count as usize;
    let src = slice::from_raw_parts(src, count);
    let dest = slice::from_raw_parts_mut(dest, count);

    dest.iter_mut()
        .zip(src.iter().map(|s| (scale * (*s)).round() as c_int))
        .take(count)
        .for_each(|(d, s)| *d = s);
}

#[no_mangle]
unsafe extern "C" fn f2i_clip_array(
    src: *const c_float,
    count: c_int,
    dest: *mut c_int,
    scale: c_float,
) {
    assert_ne!(src.is_null(), true);
    assert_ne!(dest.is_null(), true);
    assert!(count >= 0);

    let count = count as usize;
    let src = slice::from_raw_parts(src, count);
    let dest = slice::from_raw_parts_mut(dest, count);

    dest.iter_mut().zip(src.iter()).for_each(|(d, s)| {
        let tmp = scale * (*s);

        if !cfg!(cpu_clips_positive) && tmp > 1.0 * std::i32::MAX as c_float {
            *d = std::i32::MAX;
        } else if !cfg!(cpu_clips_negative) && tmp < 1.0 * std::i32::MIN as c_float {
            *d = std::i32::MIN;
        } else {
            *d = tmp.round() as i32;
        };
    });
}

#[no_mangle]
unsafe extern "C" fn f2d_array(src: *const c_float, count: c_int, dest: *mut c_double) {
    assert_ne!(src.is_null(), true);
    assert_ne!(dest.is_null(), true);
    assert!(count >= 0);

    let count = count as usize;
    let src = slice::from_raw_parts(src, count);
    let dest = slice::from_raw_parts_mut(dest, count);

    dest.iter_mut()
        .zip(src.iter().map(|s| *s as c_double))
        .for_each(|(d, s)| *d = s);
}

#[no_mangle]
unsafe extern "C" fn s2f_array(
    src: *const c_short,
    dest: *mut c_float,
    count: c_int,
    scale: c_float,
) {
    assert_ne!(src.is_null(), true);
    assert_ne!(dest.is_null(), true);
    assert!(count >= 0);

    let count = count as usize;
    let src = slice::from_raw_parts(src, count);
    let dest = slice::from_raw_parts_mut(dest, count);

    dest.iter_mut()
        .zip(src.iter().map(|s| scale * ((*s) as c_float)))
        .for_each(|(d, s)| *d = s);
}

#[no_mangle]
unsafe extern "C" fn i2f_array(
    src: *const c_int,
    dest: *mut c_float,
    count: c_int,
    scale: c_float,
) {
    assert_ne!(src.is_null(), true);
    assert_ne!(dest.is_null(), true);
    assert!(count >= 0);

    let count = count as usize;
    let src = slice::from_raw_parts(src, count);
    let dest = slice::from_raw_parts_mut(dest, count);

    dest.iter_mut()
        .zip(src.iter().map(|s| scale * (*s) as c_float))
        .take(count)
        .for_each(|(d, s)| *d = s);
}

#[no_mangle]
unsafe extern "C" fn d2f_array(src: *const c_double, dest: *mut c_float, count: c_int) {
    assert_ne!(src.is_null(), true);
    assert_ne!(dest.is_null(), true);
    assert!(count >= 0);

    let count = count as usize;
    let src = slice::from_raw_parts(src, count);
    let dest = slice::from_raw_parts_mut(dest, count);

    dest.iter_mut()
        .zip(src.iter().map(|s| *s as c_float))
        .take(count)
        .for_each(|(d, s)| *d = s);
}

#[no_mangle]
unsafe extern "C" fn host_read_f2s(
    psf: *mut SF_PRIVATE,
    ptr: *mut c_short,
    len: sf_count_t,
) -> sf_count_t {
    assert_ne!(psf.is_null(), true);
    assert_ne!(ptr.is_null(), true);
    assert!(len >= 0);

    let mut len = len as usize;
    let ptr = slice::from_raw_parts_mut(ptr, len);
    let psf = &mut *psf;

    let mut ubuf = BUF_UNION {
        fbuf: [0.0; SF_BUFFER_LEN / mem::size_of::<f32>()],
    };
    let convert = if psf.add_clipping != SF_FALSE {
        f2s_clip_array
    } else {
        f2s_array
    };
    let mut bufferlen = ubuf.fbuf.len();
    let scale = if psf.float_int_mult == SF_FALSE {
        1.0
    } else {
        0x7FFF as c_float / psf.float_max
    };

    let mut total: sf_count_t = 0;
    while len > 0 {
        if len < bufferlen {
            bufferlen = len;
        }
        let readcount = psf_fread(
            ubuf.fbuf.as_mut_ptr() as *mut c_void,
            mem::size_of::<c_float>() as sf_count_t,
            bufferlen as sf_count_t,
            psf,
        );

        /* Fix me : Need lef2s_array */
        if psf.data_endswap == SF_TRUE {
            endswap_int_array(ubuf.ibuf.as_mut_ptr(), readcount as c_int);
        }

        convert(
            ubuf.fbuf.as_ptr(),
            readcount as c_int,
            ptr[total as usize..].as_mut_ptr(),
            scale,
        );
        total += readcount;
        if readcount < bufferlen as sf_count_t {
            break;
        }
        len -= readcount as usize;
    }

    return total;
}

#[no_mangle]
unsafe extern "C" fn host_read_f2i(
    psf: *mut SF_PRIVATE,
    ptr: *mut c_int,
    len: sf_count_t,
) -> sf_count_t {
    assert_ne!(psf.is_null(), true);
    assert_ne!(ptr.is_null(), true);
    assert!(len >= 0);

    let mut len = len as usize;
    let ptr = slice::from_raw_parts_mut(ptr, len);
    let psf = &mut *psf;

    let mut ubuf = BUF_UNION {
        fbuf: [0.0; SF_BUFFER_LEN / mem::size_of::<f32>()],
    };
    let convert = if psf.add_clipping != SF_FALSE {
        f2i_clip_array
    } else {
        f2i_array
    };
    let mut bufferlen = ubuf.fbuf.len();
    let scale = if psf.float_int_mult == SF_FALSE {
        1.0
    } else {
        0x7FFFFFFF as c_float / psf.float_max
    };

    let mut total: sf_count_t = 0;
    while len > 0 {
        if len < bufferlen {
            bufferlen = len;
        }
        let readcount = psf_fread(
            ubuf.fbuf.as_mut_ptr() as *mut c_void,
            mem::size_of::<c_int>() as sf_count_t,
            bufferlen as sf_count_t,
            psf,
        );

        /* Fix me : Need lef2s_array */
        if psf.data_endswap == SF_TRUE {
            endswap_int_array(ubuf.ibuf.as_mut_ptr(), readcount as c_int);
        }

        convert(
            ubuf.fbuf.as_ptr(),
            readcount as c_int,
            ptr[total as usize..].as_mut_ptr(),
            scale,
        );
        total += readcount;
        if readcount < bufferlen as sf_count_t {
            break;
        }
        len -= readcount as usize;
    }

    return total;
}

#[no_mangle]
unsafe extern "C" fn host_read_f(
    psf: *mut SF_PRIVATE,
    ptr: *mut c_float,
    len: sf_count_t,
) -> sf_count_t {
    assert_ne!(psf.is_null(), true);
    assert_ne!(ptr.is_null(), true);
    assert!(len >= 0);

    let mut len = len as usize;
    let psf = &mut *psf;

    let mut ubuf = BUF_UNION {
        fbuf: [0.0; SF_BUFFER_LEN / mem::size_of::<f32>()],
    };

    if psf.data_endswap != SF_TRUE {
        return psf_fread(
            ptr as *mut c_void,
            mem::size_of::<c_float>() as sf_count_t,
            len as sf_count_t,
            psf,
        );
    }

    let mut bufferlen = ubuf.fbuf.len();

    let mut total: sf_count_t = 0;
    while len > 0 {
        if len < bufferlen {
            bufferlen = len;
        }
        let readcount = psf_fread(
            ubuf.fbuf.as_mut_ptr() as *mut c_void,
            mem::size_of::<c_float>() as sf_count_t,
            bufferlen as sf_count_t,
            psf,
        );

        endswap_int_copy(
            ptr.offset(total as isize) as *mut c_int,
            ubuf.ibuf.as_ptr(),
            readcount as c_int,
        );

        total += readcount;
        if readcount < bufferlen as sf_count_t {
            break;
        }
        len -= readcount as usize;
    }

    return total;
}

#[no_mangle]
unsafe extern "C" fn host_read_f2d(
    psf: *mut SF_PRIVATE,
    ptr: *mut c_double,
    len: sf_count_t,
) -> sf_count_t {
    assert_ne!(psf.is_null(), true);
    assert_ne!(ptr.is_null(), true);
    assert!(len >= 0);

    let mut len = len as usize;
    let psf = &mut *psf;

    let mut ubuf = BUF_UNION {
        fbuf: [0.0; SF_BUFFER_LEN / mem::size_of::<f32>()],
    };

    let mut bufferlen = ubuf.fbuf.len();

    let mut total: sf_count_t = 0;
    while len > 0 {
        if len < bufferlen {
            bufferlen = len;
        }
        let readcount = psf_fread(
            ubuf.fbuf.as_mut_ptr() as *mut c_void,
            mem::size_of::<c_float>() as sf_count_t,
            bufferlen as sf_count_t,
            psf,
        );

        if psf.data_endswap == SF_TRUE {
            endswap_int_array(ubuf.ibuf.as_mut_ptr(), bufferlen as c_int);
        }

        /* Fix me : Need lef2d_array */
        f2d_array(
            ubuf.fbuf.as_ptr(),
            readcount as c_int,
            ptr.offset(total as isize),
        );
        total += readcount;
        if readcount < bufferlen as sf_count_t {
            break;
        }
        len -= readcount as usize;
    }

    return total;
}

#[no_mangle]
unsafe extern "C" fn host_write_s2f(
    psf: *mut SF_PRIVATE,
    ptr: *const c_short,
    len: sf_count_t,
) -> sf_count_t {
    assert!(len >= 0);
    assert_ne!(ptr.is_null(), true);
    assert_ne!(psf.is_null(), true);

    let mut len = len as usize;
    let ptr = slice::from_raw_parts(ptr, len);
    let psf = &mut *psf;

    /* Erik */
    let scale = if psf.scale_int_float == 0 {
        1.0
    } else {
        1.0 / 0x8000 as c_float
    };
    let mut ubuf = BUF_UNION {
        fbuf: [0.0; SF_BUFFER_LEN / mem::size_of::<c_float>()],
    };
    let mut bufferlen = ubuf.fbuf.len();
    let mut total = 0;

    while len > 0 {
        if len < bufferlen {
            bufferlen = len;
        }
        s2f_array(
            ptr[total..].as_ptr(),
            ubuf.fbuf.as_mut_ptr(),
            bufferlen as c_int,
            scale,
        );

        if psf_peak_info_exists(psf) != 0 {
            float32_peak_update(
                psf,
                ubuf.fbuf.as_mut_ptr(),
                bufferlen as c_int,
                (total / psf.sf.channels as usize) as sf_count_t,
            );
        }

        if psf.data_endswap == SF_TRUE {
            endswap_int_array(ubuf.ibuf.as_mut_ptr(), bufferlen as c_int);
        }

        let writecount = psf_fwrite(
            ubuf.fbuf.as_ptr() as *const c_void,
            mem::size_of::<c_float>() as sf_count_t,
            bufferlen as sf_count_t,
            psf,
        );
        total += writecount as usize;
        if (writecount as usize) < bufferlen {
            break;
        }
        len -= writecount as usize;
    }

    return total as sf_count_t;
}

#[no_mangle]
unsafe extern "C" fn float32_peak_update(
    psf: *mut SF_PRIVATE,
    buffer: *const c_float,
    count: c_int,
    indx: sf_count_t,
) {
    assert!(indx >= 0);
    assert!(count >= 0);
    assert_ne!(buffer.is_null(), true);
    assert_ne!(psf.is_null(), true);

    let count = count as usize;
    let buffer = slice::from_raw_parts(buffer, count);
    let psf = &mut *psf;

    for chan in 0..psf.sf.channels as usize {
        let mut fmaxval = buffer[chan];
        let mut position = 0;
        let mut k = chan;
        while k < count {
            if fmaxval < buffer[k].abs() {
                fmaxval = buffer[k].abs();
                position = k;
            };
            k += psf.sf.channels as usize;
        }

        let peak_pos = psf_peak_info_get_peak_pos(psf, chan);
        assert_ne!(peak_pos.is_null(), true);
        let peak_pos = &mut *peak_pos;
        if fmaxval as f64 > peak_pos.value {
            peak_pos.value = fmaxval as f64;
            peak_pos.position =
                psf.write_current + indx + (position / psf.sf.channels as usize) as sf_count_t;
        };
    }

    return;
}
