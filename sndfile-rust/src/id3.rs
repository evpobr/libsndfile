use crate::{
    common::{psf_log_printf, SF_PRIVATE},
    psf_binheader_readf, sf_count_t,
};
use libc::c_int;

use byte_strings::c_str;

#[no_mangle]
unsafe fn id3_skip(psf: *mut SF_PRIVATE) -> c_int {
    assert!(!psf.is_null());
    let psf = &mut *psf;

    let mut buf = [0u8; 10];

    psf_binheader_readf(psf, c_str!("pb").as_ptr(), 0, buf.as_mut_ptr(), 10);

    if buf[0] == b'I' && buf[1] == b'D' && buf[2] == b'3' {
        let mut offset = (buf[6] & 0x7f) as c_int;
        offset = (offset << 7) | (buf[7] & 0x7f) as c_int;
        offset = (offset << 7) | (buf[8] & 0x7f) as c_int;
        offset = (offset << 7) | (buf[9] & 0x7f) as c_int;

        psf_log_printf(
            psf,
            c_str!("ID3 length : %d\n--------------------\n").as_ptr(),
            offset,
        );

        /* Never want to jump backwards in a file. */
        if offset < 0 {
            return 0;
        }

        /* Calculate new file offset and position ourselves there. */
        psf.fileoffset = psf.fileoffset + (offset as sf_count_t) + 10;

        if psf.fileoffset < psf.filelength {
            psf_binheader_readf(psf, c_str!("p").as_ptr(), psf.fileoffset);
            return 1;
        }
    }

    0
}
