
use crate::common::{psf_log_printf, SF_PRIVATE};
use crate::*;

use common::{SFE_BAD_OPEN_FORMAT, SFE_HTK_NO_PIPE};
use libc::*;
use std::ffi::CString;

const SFE_HTK_BAD_FILE_LEN: c_int = 1666;
const SFE_HTK_NOT_WAVEFORM: c_int = 1667;

#[no_mangle]
unsafe fn htk_open(psf: *mut SF_PRIVATE) -> c_int {
    debug_assert!(!psf.is_null());

    let psf = &mut *psf;
    let mut error = 0;

    if psf.is_pipe != 0 {
        return SFE_HTK_NO_PIPE;
    }

    if psf.file.mode == SFM_READ || (psf.file.mode == SFM_RDWR && psf.filelength > 0) {
        error = htk_read_header(psf);
        if error != 0 {
            return error;
        }
    }

    let subformat = SF_CODEC!(psf.sf.format);

    if psf.file.mode == SFM_WRITE || psf.file.mode == SFM_RDWR {
        if (SF_CONTAINER!(psf.sf.format)) != SF_FORMAT_HTK {
            return SFE_BAD_OPEN_FORMAT;
        }

        psf.endian = SF_ENDIAN_BIG;

        if htk_write_header(psf, SF_FALSE) != 0 {
            return psf.error;
        }

        psf.write_header = Some(htk_write_header);
    };

    psf.container_close = Some(htk_close);

    psf.blockwidth = psf.bytewidth * psf.sf.channels;

    match subformat {
        SF_FORMAT_PCM_16 => {
            /* 16-bit linear PCM. */
            error = pcm_init(psf);
        }
        _ => {}
    }

    return error;
}

#[no_mangle]
unsafe extern "C" fn htk_close(psf: *mut SF_PRIVATE) -> c_int {
    debug_assert!(!psf.is_null());

    let psf = &mut *psf;

    if psf.file.mode == SFM_WRITE || psf.file.mode == SFM_RDWR {
        htk_write_header(psf, SF_TRUE);
    }

    0
}

#[no_mangle]
unsafe extern "C" fn htk_write_header(psf: *mut SF_PRIVATE, calc_length: c_int) -> c_int {
    debug_assert!(!psf.is_null());

    let psf = &mut *psf;

    let current = psf_ftell(psf);

    if calc_length != 0 {
        psf.filelength = psf_get_filelen(psf);
    }

    /* Reset the current header length to zero. */
    psf.header.ptr.write(0);
    psf.header.indx = 0;
    psf_fseek(psf, 0, SEEK_SET);

    let sample_count = if psf.filelength > 12 {
        (psf.filelength - 12) / 2
    } else {
        0
    };

    let sample_period = 10000000 / psf.sf.samplerate;

    let format = CString::new("E444").unwrap();
    psf_binheader_writef(
        psf,
        format.as_ptr(),
        BHW4!(sample_count),
        BHW4!(sample_period),
        BHW4!(0x20000),
    );

    /* Header construction complete so write it out. */
    psf_fwrite(psf.header.ptr as *const c_void, psf.header.indx, 1, psf);

    if psf.error != 0 {
        return psf.error;
    }

    psf.dataoffset = psf.header.indx;

    if current > 0 {
        psf_fseek(psf, current, SEEK_SET);
    }

    return psf.error;
}

#[no_mangle]
unsafe fn htk_read_header(psf: *mut SF_PRIVATE) -> c_int {
    // int		sample_count, sample_period, marker ;

    debug_assert!(!psf.is_null());

    let psf = &mut *psf;

    let mut sample_count: c_int = 0;
    let mut sample_period: c_int = 0;
    let mut marker: c_int = 0;
    let format = CString::new("pE444").unwrap();
    psf_binheader_readf(
        psf,
        format.as_ptr(),
        0,
        &mut sample_count,
        &mut sample_period,
        &mut marker,
    );

    if (2 * sample_count + 12) as sf_count_t != psf.filelength {
        return SFE_HTK_BAD_FILE_LEN;
    }

    if marker != 0x20000 {
        return SFE_HTK_NOT_WAVEFORM;
    }

    psf.sf.channels = 1;

    if sample_period > 0 {
        psf.sf.samplerate = 10000000 / sample_period;
        let format = CString::new(
            "HTK Waveform file\n  Sample Count  : %d\n  Sample Period : %d => %d Hz\n",
        )
        .unwrap();
        psf_log_printf(
            psf,
            format.as_ptr(),
            sample_count,
            sample_period,
            psf.sf.samplerate,
        );
    } else {
        psf.sf.samplerate = 16000;
        let format = CString::new("HTK Waveform file\n  Sample Count  : %d\n  Sample Period : %d (should be > 0) => Guessed sample rate %d Hz\n").unwrap();
        psf_log_printf(
            psf,
            format.as_ptr(),
            sample_count,
            sample_period,
            psf.sf.samplerate,
        );
    }

    psf.sf.format = SF_FORMAT_HTK | SF_FORMAT_PCM_16;
    psf.bytewidth = 2;

    /* HTK always has a 12 byte header. */
    psf.dataoffset = 12;
    psf.endian = SF_ENDIAN_BIG;

    psf.datalength = psf.filelength - psf.dataoffset;

    psf.blockwidth = psf.sf.channels * psf.bytewidth;

    if psf.sf.frames == 0 && psf.blockwidth != 0 {
        psf.sf.frames = (psf.filelength - psf.dataoffset) / psf.blockwidth as sf_count_t;
    }

    0
}
