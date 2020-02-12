use crate::sfendian::*;
use std::ffi::CString;
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
unsafe extern "C" fn float32_init(psf: *mut SF_PRIVATE) -> c_int {
    assert_ne!(psf.is_null(), true);

    let psf = &mut *psf;

    if psf.sf.channels < 1 {
        psf_log_printf(
            psf,
            CString::new("float32_init : internal error : channels = %d\n")
                .unwrap()
                .as_ptr(),
            psf.sf.channels,
        );
        return SFE_INTERNAL;
    };

    let float_caps = float32_get_capability(psf);

    psf.blockwidth = mem::size_of::<c_float>() as c_int * psf.sf.channels;

    if psf.file.mode == SFM_READ || psf.file.mode == SFM_RDWR {
        match psf.endian + float_caps {
            x if x == SF_ENDIAN_BIG + FLOAT_CAN_RW_BE => {
                psf.data_endswap = SF_FALSE;
                psf.read_short = Some(host_read_f2s);
                psf.read_int = Some(host_read_f2i);
                psf.read_float = Some(host_read_f);
                psf.read_double = Some(host_read_f2d);
            }

            x if x == SF_ENDIAN_LITTLE + FLOAT_CAN_RW_LE => {
                psf.data_endswap = SF_FALSE;
                psf.read_short = Some(host_read_f2s);
                psf.read_int = Some(host_read_f2i);
                psf.read_float = Some(host_read_f);
                psf.read_double = Some(host_read_f2d);
            }

            x if x == SF_ENDIAN_BIG + FLOAT_CAN_RW_LE => {
                psf.data_endswap = SF_TRUE;
                psf.read_short = Some(host_read_f2s);
                psf.read_int = Some(host_read_f2i);
                psf.read_float = Some(host_read_f);
                psf.read_double = Some(host_read_f2d);
            }

            x if x == SF_ENDIAN_LITTLE + FLOAT_CAN_RW_BE => {
                psf.data_endswap = SF_TRUE;
                psf.read_short = Some(host_read_f2s);
                psf.read_int = Some(host_read_f2i);
                psf.read_float = Some(host_read_f);
                psf.read_double = Some(host_read_f2d);
            }

            /* When the CPU is not IEEE compatible. */
            x if x == SF_ENDIAN_BIG + FLOAT_BROKEN_LE => {
                psf.data_endswap = SF_TRUE;
                psf.read_short = Some(replace_read_f2s);
                psf.read_int = Some(replace_read_f2i);
                psf.read_float = Some(replace_read_f);
                psf.read_double = Some(replace_read_f2d);
            }

            x if x == SF_ENDIAN_LITTLE + FLOAT_BROKEN_LE => {
                psf.data_endswap = SF_FALSE;
                psf.read_short = Some(replace_read_f2s);
                psf.read_int = Some(replace_read_f2i);
                psf.read_float = Some(replace_read_f);
                psf.read_double = Some(replace_read_f2d);
            }

            x if x == SF_ENDIAN_BIG + FLOAT_BROKEN_BE => {
                psf.data_endswap = SF_FALSE;
                psf.read_short = Some(replace_read_f2s);
                psf.read_int = Some(replace_read_f2i);
                psf.read_float = Some(replace_read_f);
                psf.read_double = Some(replace_read_f2d);
            }

            x if x == SF_ENDIAN_LITTLE + FLOAT_BROKEN_BE => {
                psf.data_endswap = SF_TRUE;
                psf.read_short = Some(replace_read_f2s);
                psf.read_int = Some(replace_read_f2i);
                psf.read_float = Some(replace_read_f);
                psf.read_double = Some(replace_read_f2d);
            }
            _ => {}
        }
    };

    if psf.file.mode == SFM_WRITE || psf.file.mode == SFM_RDWR {
        match psf.endian + float_caps {
            x if x == SF_ENDIAN_LITTLE + FLOAT_CAN_RW_LE => {
                psf.data_endswap = SF_FALSE;
                psf.write_short = Some(host_write_s2f);
                psf.write_int = Some(host_write_i2f);
                psf.write_float = Some(host_write_f);
                psf.write_double = Some(host_write_d2f);
            }

            x if x == SF_ENDIAN_BIG + FLOAT_CAN_RW_BE => {
                psf.data_endswap = SF_FALSE;
                psf.write_short = Some(host_write_s2f);
                psf.write_int = Some(host_write_i2f);
                psf.write_float = Some(host_write_f);
                psf.write_double = Some(host_write_d2f);
            }

            x if x == SF_ENDIAN_BIG + FLOAT_CAN_RW_LE => {
                psf.data_endswap = SF_TRUE;
                psf.write_short = Some(host_write_s2f);
                psf.write_int = Some(host_write_i2f);
                psf.write_float = Some(host_write_f);
                psf.write_double = Some(host_write_d2f);
            }

            x if x == SF_ENDIAN_LITTLE + FLOAT_CAN_RW_BE => {
                psf.data_endswap = SF_TRUE;
                psf.write_short = Some(host_write_s2f);
                psf.write_int = Some(host_write_i2f);
                psf.write_float = Some(host_write_f);
                psf.write_double = Some(host_write_d2f);
            }

            /* When the CPU is not IEEE compatible. */
            x if x == SF_ENDIAN_BIG + FLOAT_BROKEN_LE => {
                psf.data_endswap = SF_TRUE;
                psf.write_short = Some(replace_write_s2f);
                psf.write_int = Some(replace_write_i2f);
                psf.write_float = Some(replace_write_f);
                psf.write_double = Some(replace_write_d2f);
            }

            x if x == SF_ENDIAN_LITTLE + FLOAT_BROKEN_LE => {
                psf.data_endswap = SF_FALSE;
                psf.write_short = Some(replace_write_s2f);
                psf.write_int = Some(replace_write_i2f);
                psf.write_float = Some(replace_write_f);
                psf.write_double = Some(replace_write_d2f);
            }

            x if x == SF_ENDIAN_BIG + FLOAT_BROKEN_BE => {
                psf.data_endswap = SF_FALSE;
                psf.write_short = Some(replace_write_s2f);
                psf.write_int = Some(replace_write_i2f);
                psf.write_float = Some(replace_write_f);
                psf.write_double = Some(replace_write_d2f);
            }

            x if x == SF_ENDIAN_LITTLE + FLOAT_BROKEN_BE => {
                psf.data_endswap = SF_TRUE;
                psf.write_short = Some(replace_write_s2f);
                psf.write_int = Some(replace_write_i2f);
                psf.write_float = Some(replace_write_f);
                psf.write_double = Some(replace_write_d2f);
            }

            _ => {}
        }
    };

    psf.datalength = if psf.filelength > psf.dataoffset {
        if psf.dataend > 0 {
            psf.dataend - psf.dataoffset
        } else {
            psf.filelength - psf.dataoffset
        }
    } else {
        0
    };

    psf.sf.frames = if psf.blockwidth > 0 {
        psf.datalength / psf.blockwidth as sf_count_t
    } else {
        0
    };

    return 0;
}

#[no_mangle]
unsafe extern "C" fn float32_be_read(cptr: *mut u8) -> f32 {
    assert_ne!(cptr.is_null(), true);

    let cptr = slice::from_raw_parts_mut(cptr, 4);

    let mut fbytes = [0; 4];
    fbytes.clone_from_slice(&cptr[0..4]);
    f32::from_be_bytes(fbytes)
}

#[no_mangle]
unsafe extern "C" fn float32_le_read(cptr: *mut u8) -> f32 {
    assert_ne!(cptr.is_null(), true);

    let cptr = slice::from_raw_parts_mut(cptr, 4);

    let mut fbytes = [0; 4];
    fbytes.clone_from_slice(&cptr[0..4]);
    f32::from_le_bytes(fbytes)
}

#[no_mangle]
unsafe extern "C" fn float32_le_write(input: f32, output: *mut u8) {
    assert_ne!(output.is_null(), true);

    let output = slice::from_raw_parts_mut(output, 4);
    output.clone_from_slice(&input.to_le_bytes());
}

#[no_mangle]
unsafe extern "C" fn float32_be_write(input: f32, output: *mut u8) {
    assert_ne!(output.is_null(), true);

    let output = slice::from_raw_parts_mut(output, 4);
    output.clone_from_slice(&input.to_be_bytes());
}

#[no_mangle]
unsafe extern "C" fn float32_get_capability(psf: *mut SF_PRIVATE) -> c_int {
    assert_ne!(psf.is_null(), true);

    let psf = &mut *psf;

    let f = 1.23456789_f32; /* Some abitrary value. */
    let data = f.to_ne_bytes();

    if psf.ieee_replace == SF_FALSE {
        /* If this test is true ints and floats are compatible and little endian. */
        if data[0] == 0x52 && data[1] == 0x06 && data[2] == 0x9e && data[3] == 0x3f {
            return FLOAT_CAN_RW_LE;
        }

        /* If this test is true ints and floats are compatible and big endian. */
        if data[3] == 0x52 && data[2] == 0x06 && data[1] == 0x9e && data[0] == 0x3f {
            return FLOAT_CAN_RW_BE;
        };
    }

    /* Floats are broken. Don't expect reading or writing to be fast. */
    psf_log_printf(
        psf,
        CString::new("Using IEEE replacement code for float.\n")
            .unwrap()
            .as_ptr(),
    );

    if cfg!(target_endian = "little") {
        FLOAT_BROKEN_LE
    } else {
        FLOAT_BROKEN_BE
    }
}

fn f2s_array(src: &[f32], dest: &mut [i16], scale: f32) {
    assert_eq!(src.len(), dest.len());

    dest.iter_mut()
        .zip(src.iter().map(|s| (scale * (*s)).round() as i16))
        .for_each(|(d, s)| *d = s);
}

fn f2s_clip_array(src: &[f32], dest: &mut [i16], scale: f32) {
    assert_eq!(src.len(), dest.len());

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

    let mut total = 0;
    while len > 0 {
        if len < bufferlen {
            bufferlen = len;
        }
        let readcount = psf_fread(
            ubuf.fbuf.as_mut_ptr() as *mut c_void,
            mem::size_of::<c_float>() as sf_count_t,
            bufferlen as sf_count_t,
            psf,
        ) as usize;

        /* Fix me : Need lef2s_array */
        if psf.data_endswap == SF_TRUE {
            endswap_int_array(&mut ubuf.ibuf[..readcount]);
        }

        convert(
            &ubuf.fbuf[..readcount],
            &mut ptr[total..total + readcount],
            scale,
        );
        total += readcount;
        if readcount < bufferlen {
            break;
        }
        len -= readcount;
    }

    return total as sf_count_t;
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

    let mut total = 0;
    while len > 0 {
        if len < bufferlen {
            bufferlen = len;
        }
        let readcount = psf_fread(
            ubuf.fbuf.as_mut_ptr() as *mut c_void,
            mem::size_of::<c_int>() as sf_count_t,
            bufferlen as sf_count_t,
            psf,
        ) as usize;

        /* Fix me : Need lef2s_array */
        if psf.data_endswap == SF_TRUE {
            endswap_int_array(&mut ubuf.ibuf[..readcount]);
        }

        convert(
            ubuf.fbuf.as_ptr(),
            readcount as c_int,
            ptr[total as usize..].as_mut_ptr(),
            scale,
        );
        total += readcount;
        if readcount < bufferlen {
            break;
        }
        len -= readcount;
    }

    return total as sf_count_t;
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
    let ptr = slice::from_raw_parts_mut(ptr, len);
    let psf = &mut *psf;

    let mut ubuf = BUF_UNION {
        fbuf: [0.0; SF_BUFFER_LEN / mem::size_of::<f32>()],
    };

    if psf.data_endswap != SF_TRUE {
        return psf_fread(
            ptr.as_mut_ptr() as *mut c_void,
            mem::size_of::<c_float>() as sf_count_t,
            len as sf_count_t,
            psf,
        );
    }

    let mut bufferlen = ubuf.fbuf.len();

    let mut total = 0;
    while len > 0 {
        if len < bufferlen {
            bufferlen = len;
        }
        let readcount = psf_fread(
            ubuf.fbuf.as_mut_ptr() as *mut c_void,
            mem::size_of::<c_float>() as sf_count_t,
            bufferlen as sf_count_t,
            psf,
        ) as usize;

        endswap_f32_int_copy(&mut ptr[total..total + readcount], &ubuf.ibuf[..readcount]);

        total += readcount;
        if readcount < bufferlen {
            break;
        }
        len -= readcount as usize;
    }

    return total as sf_count_t;
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
            endswap_int_array(&mut ubuf.ibuf[..bufferlen]);
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
            endswap_int_array(&mut ubuf.ibuf[..bufferlen]);
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
unsafe extern "C" fn host_write_i2f(
    psf: *mut SF_PRIVATE,
    ptr: *const c_int,
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
        1.0 / (8.0 * 0x10000000 as c_float)
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
        i2f_array(
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
            endswap_int_array(&mut ubuf.ibuf[..bufferlen]);
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

#[no_mangle]
unsafe extern "C" fn host_write_f(
    psf: *mut SF_PRIVATE,
    ptr: *const c_float,
    len: sf_count_t,
) -> sf_count_t {
    assert!(len >= 0);
    assert_ne!(ptr.is_null(), true);
    assert_ne!(psf.is_null(), true);

    let mut len = len as usize;
    let ptr = slice::from_raw_parts(ptr, len);
    let psf = &mut *psf;

    if psf_peak_info_exists(psf) != 0 {
        float32_peak_update(psf, ptr.as_ptr(), len as c_int, 0);
    }

    if psf.data_endswap != SF_TRUE {
        return psf_fwrite(
            ptr.as_ptr() as *const c_void,
            mem::size_of::<c_float>() as sf_count_t,
            len as sf_count_t,
            psf,
        );
    }

    let mut ubuf = BUF_UNION {
        fbuf: [0.0; SF_BUFFER_LEN / mem::size_of::<c_float>()],
    };
    let mut bufferlen = ubuf.fbuf.len();
    let mut total = 0;

    while len > 0 {
        if len < bufferlen {
            bufferlen = len;
        }

        endswap_int_f32_copy(&mut ubuf.ibuf[..bufferlen], &ptr[total..total + bufferlen]);

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
unsafe extern "C" fn host_write_d2f(
    psf: *mut SF_PRIVATE,
    ptr: *const c_double,
    len: sf_count_t,
) -> sf_count_t {
    assert!(len >= 0);
    assert_ne!(ptr.is_null(), true);
    assert_ne!(psf.is_null(), true);

    let mut len = len as usize;
    let ptr = slice::from_raw_parts(ptr, len);
    let psf = &mut *psf;

    let mut ubuf = BUF_UNION {
        fbuf: [0.0; SF_BUFFER_LEN / mem::size_of::<c_float>()],
    };

    let mut bufferlen = ubuf.fbuf.len();
    let mut total = 0;

    while len > 0 {
        if len < bufferlen {
            bufferlen = len;
        }

        d2f_array(
            ptr.as_ptr().add(total),
            ubuf.fbuf.as_mut_ptr(),
            bufferlen as c_int,
        );

        if psf_peak_info_exists(psf) != 0 {
            float32_peak_update(
                psf,
                ubuf.fbuf.as_ptr(),
                bufferlen as c_int,
                (total / psf.sf.channels as usize) as sf_count_t,
            );
        }

        if psf.data_endswap == SF_TRUE {
            endswap_int_array(&mut ubuf.ibuf[..bufferlen]);
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
unsafe extern "C" fn replace_read_f2s(
    psf: *mut SF_PRIVATE,
    ptr: *mut c_short,
    len: sf_count_t,
) -> sf_count_t {
    assert!(len >= 0);
    assert_ne!(ptr.is_null(), true);
    assert_ne!(psf.is_null(), true);

    let mut len = len as usize;
    let ptr = slice::from_raw_parts_mut(ptr, len);
    let psf = &mut *psf;

    let mut ubuf = BUF_UNION {
        fbuf: [0.0; SF_BUFFER_LEN / mem::size_of::<c_float>()],
    };
    let mut bufferlen = ubuf.fbuf.len();
    let scale = if psf.float_int_mult == 0 {
        1.0
    } else {
        0x7FFF as c_float / psf.float_max
    };

    let mut total = 0;
    while len > 0 {
        if len < bufferlen {
            bufferlen = len;
        }
        let readcount = psf_fread(
            ubuf.fbuf.as_mut_ptr() as *mut c_void,
            mem::size_of::<c_float>() as sf_count_t,
            bufferlen as sf_count_t,
            psf,
        ) as usize;

        if psf.data_endswap == SF_TRUE {
            endswap_int_array(&mut ubuf.ibuf[..bufferlen]);
        }

        bf2f_array(&mut ubuf.fbuf[..bufferlen]);

        f2s_array(
            &ubuf.fbuf[..readcount],
            &mut ptr[total..total + readcount],
            scale,
        );
        total += readcount;
        if (readcount as usize) < bufferlen {
            break;
        }
        len -= readcount;
    }

    return total as sf_count_t;
}

#[no_mangle]
unsafe extern "C" fn replace_read_f2i(
    psf: *mut SF_PRIVATE,
    ptr: *mut c_int,
    len: sf_count_t,
) -> sf_count_t {
    assert!(len >= 0);
    assert_ne!(ptr.is_null(), true);
    assert_ne!(psf.is_null(), true);

    let mut len = len as usize;
    let ptr = slice::from_raw_parts_mut(ptr, len);
    let psf = &mut *psf;

    let mut ubuf = BUF_UNION {
        fbuf: [0.0; SF_BUFFER_LEN / mem::size_of::<c_float>()],
    };
    let mut bufferlen = ubuf.fbuf.len();
    let scale = if psf.float_int_mult == 0 {
        1.0
    } else {
        0x7FFF as c_float / psf.float_max
    };

    let mut total = 0;
    while len > 0 {
        if len < bufferlen {
            bufferlen = len;
        }
        let readcount = psf_fread(
            ubuf.fbuf.as_mut_ptr() as *mut c_void,
            mem::size_of::<c_float>() as sf_count_t,
            bufferlen as sf_count_t,
            psf,
        ) as usize;

        if psf.data_endswap == SF_TRUE {
            endswap_int_array(&mut ubuf.ibuf[..bufferlen]);
        }

        bf2f_array(&mut ubuf.fbuf[..bufferlen]);

        f2i_array(
            ubuf.fbuf.as_mut_ptr(),
            readcount as c_int,
            ptr[total..].as_mut_ptr(),
            scale,
        );
        total += readcount;
        if (readcount as usize) < bufferlen {
            break;
        }
        len -= readcount;
    }

    return total as sf_count_t;
}

#[no_mangle]
unsafe extern "C" fn replace_read_f(
    psf: *mut SF_PRIVATE,
    ptr: *mut c_float,
    len: sf_count_t,
) -> sf_count_t {
    assert!(len >= 0);
    assert_ne!(ptr.is_null(), true);
    assert_ne!(psf.is_null(), true);

    let mut len = len as usize;
    let ptr = slice::from_raw_parts_mut(ptr, len);
    let psf = &mut *psf;

    let mut ubuf = BUF_UNION {
        fbuf: [0.0; SF_BUFFER_LEN / mem::size_of::<c_float>()],
    };
    let mut bufferlen = ubuf.fbuf.len();
    let mut total = 0;

    while len > 0 {
        if len < bufferlen {
            bufferlen = len;
        }
        let readcount = psf_fread(
            ubuf.fbuf.as_mut_ptr() as *mut c_void,
            mem::size_of::<c_float>() as sf_count_t,
            bufferlen as sf_count_t,
            psf,
        ) as usize;

        if psf.data_endswap == SF_TRUE {
            endswap_int_array(&mut ubuf.ibuf[..bufferlen]);
        }

        bf2f_array(&mut ubuf.fbuf[..bufferlen]);
        ptr[total..total + readcount].clone_from_slice(&ubuf.fbuf[0..readcount]);

        total += readcount;
        if readcount < bufferlen {
            break;
        }
        len -= readcount;
    }

    return total as sf_count_t;
}

#[no_mangle]
unsafe extern "C" fn replace_read_f2d(
    psf: *mut SF_PRIVATE,
    ptr: *mut c_double,
    len: sf_count_t,
) -> sf_count_t {
    assert!(len >= 0);
    assert_ne!(ptr.is_null(), true);
    assert_ne!(psf.is_null(), true);

    let mut len = len as usize;
    let ptr = slice::from_raw_parts_mut(ptr, len);
    let psf = &mut *psf;

    let mut ubuf = BUF_UNION {
        fbuf: [0.0; SF_BUFFER_LEN / mem::size_of::<c_float>()],
    };
    let mut bufferlen = ubuf.fbuf.len();
    let mut total = 0;

    while len > 0 {
        if len < bufferlen {
            bufferlen = len;
        }
        let readcount = psf_fread(
            ubuf.fbuf.as_mut_ptr() as *mut c_void,
            mem::size_of::<c_float>() as sf_count_t,
            bufferlen as sf_count_t,
            psf,
        ) as usize;

        if psf.data_endswap == SF_TRUE {
            endswap_int_array(&mut ubuf.ibuf[..bufferlen]);
        }

        bf2f_array(&mut ubuf.fbuf[..bufferlen]);

        f2d_array(
            ubuf.fbuf.as_mut_ptr(),
            readcount as c_int,
            ptr[total..].as_mut_ptr(),
        );
        total += readcount;
        if readcount < bufferlen {
            break;
        }
        len -= readcount;
    }

    return total as sf_count_t;
}

#[no_mangle]
unsafe extern "C" fn replace_write_s2f(
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

    let mut ubuf = BUF_UNION {
        fbuf: [0.0; SF_BUFFER_LEN / mem::size_of::<c_float>()],
    };
    let mut bufferlen = ubuf.fbuf.len();

    let scale = if psf.scale_int_float == 0 {
        1.0
    } else {
        1.0 / 0x8000 as c_float
    };

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
                total as sf_count_t / psf.sf.channels as sf_count_t,
            );
        }

        f2bf_array(&mut ubuf.fbuf[..bufferlen]);

        if psf.data_endswap == SF_TRUE {
            endswap_int_array(&mut ubuf.ibuf[..bufferlen]);
        }

        let writecount = psf_fwrite(
            ubuf.fbuf.as_ptr() as *const c_void,
            mem::size_of::<c_float>() as sf_count_t,
            bufferlen as sf_count_t,
            psf,
        ) as usize;
        total += writecount;
        if writecount < bufferlen {
            break;
        }
        len -= writecount;
    }

    return total as sf_count_t;
}

#[no_mangle]
unsafe extern "C" fn replace_write_i2f(
    psf: *mut SF_PRIVATE,
    ptr: *const c_int,
    len: sf_count_t,
) -> sf_count_t {
    assert!(len >= 0);
    assert_ne!(ptr.is_null(), true);
    assert_ne!(psf.is_null(), true);

    let mut len = len as usize;
    let ptr = slice::from_raw_parts(ptr, len);
    let psf = &mut *psf;

    let mut ubuf = BUF_UNION {
        fbuf: [0.0; SF_BUFFER_LEN / mem::size_of::<c_float>()],
    };
    let mut bufferlen = ubuf.fbuf.len();

    let scale = if psf.scale_int_float == 0 {
        1.0
    } else {
        1.0 / (8.0 * 0x10000000 as c_float)
    };

    let mut total = 0;

    while len > 0 {
        if len < bufferlen {
            bufferlen = len;
        }
        i2f_array(
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
                total as sf_count_t / psf.sf.channels as sf_count_t,
            );
        }

        f2bf_array(&mut ubuf.fbuf[..bufferlen]);

        if psf.data_endswap == SF_TRUE {
            endswap_int_array(&mut ubuf.ibuf[..bufferlen]);
        }

        let writecount = psf_fwrite(
            ubuf.fbuf.as_ptr() as *const c_void,
            mem::size_of::<c_float>() as sf_count_t,
            bufferlen as sf_count_t,
            psf,
        ) as usize;
        total += writecount;
        if writecount < bufferlen {
            break;
        }
        len -= writecount;
    }

    return total as sf_count_t;
}

#[no_mangle]
unsafe extern "C" fn replace_write_f(
    psf: *mut SF_PRIVATE,
    ptr: *const c_float,
    len: sf_count_t,
) -> sf_count_t {
    assert!(len >= 0);
    assert_ne!(ptr.is_null(), true);
    assert_ne!(psf.is_null(), true);

    let mut len = len as usize;
    let ptr = slice::from_raw_parts(ptr, len);
    let psf = &mut *psf;

    let mut ubuf = BUF_UNION {
        fbuf: [0.0; SF_BUFFER_LEN / mem::size_of::<c_float>()],
    };
    let mut bufferlen = ubuf.fbuf.len();

    if psf_peak_info_exists(psf) != 0 {
        float32_peak_update(psf, ptr.as_ptr(), len as c_int, 0);
    }

    let mut total = 0;

    while len > 0 {
        if len < bufferlen {
            bufferlen = len;
        }

        ubuf.fbuf[..bufferlen].clone_from_slice(&ptr[total..total + bufferlen]);

        f2bf_array(&mut ubuf.fbuf[..bufferlen]);

        if psf.data_endswap == SF_TRUE {
            endswap_int_array(&mut ubuf.ibuf[..bufferlen]);
        }

        let writecount = psf_fwrite(
            ubuf.fbuf.as_ptr() as *const c_void,
            mem::size_of::<c_float>() as sf_count_t,
            bufferlen as sf_count_t,
            psf,
        ) as usize;
        total += writecount;
        if writecount < bufferlen {
            break;
        }
        len -= writecount;
    }

    return total as sf_count_t;
}

#[no_mangle]
unsafe extern "C" fn replace_write_d2f(
    psf: *mut SF_PRIVATE,
    ptr: *const c_double,
    len: sf_count_t,
) -> sf_count_t {
    assert!(len >= 0);
    assert_ne!(ptr.is_null(), true);
    assert_ne!(psf.is_null(), true);

    let mut len = len as usize;
    let ptr = slice::from_raw_parts(ptr, len);
    let psf = &mut *psf;

    let mut ubuf = BUF_UNION {
        fbuf: [0.0; SF_BUFFER_LEN / mem::size_of::<c_float>()],
    };
    let mut bufferlen = ubuf.fbuf.len();
    let mut total = 0;

    while len > 0 {
        if len < bufferlen {
            bufferlen = len;
        }
        d2f_array(
            ptr[total..].as_ptr(),
            ubuf.fbuf.as_mut_ptr(),
            bufferlen as c_int,
        );

        if psf_peak_info_exists(psf) != 0 {
            float32_peak_update(
                psf,
                ubuf.fbuf.as_ptr(),
                bufferlen as c_int,
                total as sf_count_t / psf.sf.channels as sf_count_t,
            );
        }

        f2bf_array(&mut ubuf.fbuf[..bufferlen]);

        if psf.data_endswap == SF_TRUE {
            endswap_int_array(&mut ubuf.ibuf[..bufferlen]);
        }

        let writecount = psf_fwrite(
            ubuf.fbuf.as_ptr() as *const c_void,
            mem::size_of::<c_float>() as sf_count_t,
            bufferlen as sf_count_t,
            psf,
        ) as usize;
        total += writecount;
        if writecount < bufferlen {
            break;
        }
        len -= writecount;
    }

    return total as sf_count_t;
}

fn bf2f_array(buffer: &mut [f32]) {
    for f in buffer {
        if cfg!(target_endian = "little") {
            *f = f32::from_ne_bytes(f.to_le_bytes());
        } else {
            *f = f32::from_ne_bytes(f.to_be_bytes());
        }
    }
}

fn f2bf_array(buffer: &mut [f32]) {
    for f in buffer {
        if cfg!(target_endian = "little") {
            *f = f32::from_ne_bytes(f.to_le_bytes());
        } else {
            *f = f32::from_ne_bytes(f.to_be_bytes());
        }
    }
}
