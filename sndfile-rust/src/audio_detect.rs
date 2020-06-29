use libc::{c_int, c_uchar};

use crate::{
    common::{psf_log_printf, SF_PRIVATE},
    SF_ENDIAN_LITTLE, SF_FORMAT_FLOAT, SF_FORMAT_PCM_32,
};
use std::{ffi::CString, slice};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AUDIO_DETECT {
    pub channels: c_int,
    pub endianness: c_int,
}

#[derive(Debug, Default, Copy, Clone)]
struct VOTE {
    pub le_float: c_int,
    pub be_float: c_int,
    pub le_int_24_32: c_int,
    pub be_int_24_32: c_int,
}

#[no_mangle]
unsafe fn audio_detect(
    psf: *mut SF_PRIVATE,
    ad: *mut AUDIO_DETECT,
    data: *const c_uchar,
    datalen: c_int,
) -> c_int {
    if psf.is_null() {
        return 0;
    }
    let psf = &mut *psf;

    if ad.is_null() || datalen < 256 {
        return 0;
    }
    let ad = &mut *ad;

    assert!(!data.is_null());
    let data = slice::from_raw_parts(data, datalen as usize);

    let mut vote = VOTE::default();
    vote_for_format(&mut vote, data);

    let format = CString::new(
        r#"audio_detect :\n
    le_float     : %d\n
    be_float     : %d\n
    le_int_24_32 : %d\n
    be_int_24_32 : %d\n"#,
    )
    .unwrap();
    psf_log_printf(
        psf,
        format.as_ptr(),
        vote.le_float,
        vote.be_float,
        vote.le_int_24_32,
        vote.be_int_24_32,
    );

    if ad.endianness == SF_ENDIAN_LITTLE && vote.le_float > (3 * datalen) / 4 {
        /* Almost certainly 32 bit floats. */
        return SF_FORMAT_FLOAT;
    };

    if ad.endianness == SF_ENDIAN_LITTLE && vote.le_int_24_32 > (3 * datalen) / 4 {
        /* Almost certainly 24 bit data stored in 32 bit ints. */
        return SF_FORMAT_PCM_32;
    };

    return 0;
}

fn vote_for_format(vote: &mut VOTE, data: &[u8]) {
    *vote = VOTE::default();

    for chunk in data.chunks(4) {
        if chunk[0] == 0 && chunk[1] != 0 {
            vote.le_int_24_32 += 4;
        }

        if chunk[2] != 0 && chunk[3] == 0 {
            vote.le_int_24_32 += 4;
        }

        if chunk[0] != 0 && chunk[3] > 0x43 && chunk[3] < 0x4B {
            vote.le_float += 4;
        }

        if chunk[3] != 0 && chunk[0] > 0x43 && chunk[0] < 0x4B {
            vote.be_float += 4;
        }
    }
}
