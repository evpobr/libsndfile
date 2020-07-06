use crate::*;

use byte_strings::c_str;

use std::ffi::CStr;
use std::slice;

#[derive(Debug, Copy, Clone)]
pub struct FormatInfo<'a> {
    pub format: c_int,
    pub name: &'a CStr,
    pub extension: &'a CStr,
}

impl<'a> Into<SF_FORMAT_INFO> for FormatInfo<'a> {
    fn into(self) -> SF_FORMAT_INFO {
        SF_FORMAT_INFO {
            format: self.format,
            name: self.name.as_ptr(),
            extension: self.extension.as_ptr(),
        }
    }
}

#[cfg(not(feature = "external-xiph-libs"))]
const SIMPLE_FORMATS_COUNT: usize = 13;
#[cfg(feature = "external-xiph-libs")]
const SIMPLE_FORMATS_COUNT: usize = 16;

static SIMPLE_FORMATS: [FormatInfo; SIMPLE_FORMATS_COUNT] = [
    FormatInfo {
        format: SF_FORMAT_AIFF | SF_FORMAT_PCM_16,
        name: c_str!("AIFF (Apple/SGI 16 bit PCM)"),
        extension: c_str!("AIFF (Apple/SGI 16 bit PCM)"),
    },
    FormatInfo {
        format: SF_FORMAT_AIFF | SF_FORMAT_FLOAT,
        name: c_str!("AIFF (Apple/SGI 32 bit float)"),
        extension: c_str!("aifc"),
    },
    FormatInfo {
        format: SF_FORMAT_AIFF | SF_FORMAT_PCM_S8,
        name: c_str!("AIFF (Apple/SGI 8 bit PCM)"),
        extension: c_str!("aiff"),
    },
    FormatInfo {
        format: SF_FORMAT_AU | SF_FORMAT_PCM_16,
        name: c_str!("AU (Sun/Next 16 bit PCM)"),
        extension: c_str!("au"),
    },
    FormatInfo {
        format: SF_FORMAT_AU | SF_FORMAT_ULAW,
        name: c_str!("AU (Sun/Next 8-bit u-law)"),
        extension: c_str!("au"),
    },
    FormatInfo {
        format: SF_FORMAT_CAF | SF_FORMAT_ALAC_16,
        name: c_str!("CAF (Apple 16 bit ALAC)"),
        extension: c_str!("caf"),
    },
    FormatInfo {
        format: SF_FORMAT_CAF | SF_FORMAT_PCM_16,
        name: c_str!("CAF (Apple 16 bit PCM)"),
        extension: c_str!("caf"),
    },
    #[cfg(feature = "external-xiph-libs")]
    FormatInfo {
        format: SF_FORMAT_FLAC | SF_FORMAT_PCM_16,
        name: c_str!("FLAC 16 bit"),
        extension: c_str!("flac"),
    },
    FormatInfo {
        format: SF_FORMAT_RAW | SF_FORMAT_VOX_ADPCM,
        name: c_str!("OKI Dialogic VOX ADPCM"),
        extension: c_str!("vox"),
    },
    #[cfg(feature = "external-xiph-libs")]
    FormatInfo {
        format: SF_FORMAT_OGG | SF_FORMAT_OPUS,
        name: c_str!("Ogg Opus (Xiph Foundation)"),
        extension: c_str!("opus"),
    },
    #[cfg(feature = "external-xiph-libs")]
    FormatInfo {
        format: SF_FORMAT_OGG | SF_FORMAT_VORBIS,
        name: c_str!("Ogg Vorbis (Xiph Foundation)"),
        extension: c_str!("oga"),
    },
    FormatInfo {
        format: SF_FORMAT_WAV | SF_FORMAT_PCM_16,
        name: c_str!("WAV (Microsoft 16 bit PCM)"),
        extension: c_str!("wav"),
    },
    FormatInfo {
        format: SF_FORMAT_WAV | SF_FORMAT_FLOAT,
        name: c_str!("WAV (Microsoft 32 bit float)"),
        extension: c_str!("wav"),
    },
    FormatInfo {
        format: SF_FORMAT_WAV | SF_FORMAT_IMA_ADPCM,
        name: c_str!("WAV (Microsoft 4 bit IMA ADPCM)"),
        extension: c_str!("wav"),
    },
    FormatInfo {
        format: SF_FORMAT_WAV | SF_FORMAT_MS_ADPCM,
        name: c_str!("WAV (Microsoft 4 bit MS ADPCM)"),
        extension: c_str!("wav"),
    },
    FormatInfo {
        format: SF_FORMAT_WAV | SF_FORMAT_PCM_U8,
        name: c_str!("WAV (Microsoft 8 bit PCM)"),
        extension: c_str!("wav"),
    },
];

#[no_mangle]
unsafe fn psf_get_format_simple_count() -> c_int {
    return SIMPLE_FORMATS.len() as c_int;
}

#[no_mangle]
unsafe fn psf_get_format_simple(data: *mut SF_FORMAT_INFO) -> c_int {
    assert!(!data.is_null());
    let data = &mut *data;

    if data.format < 0 || (data.format as usize) >= SIMPLE_FORMATS.len() {
        return SFE_BAD_COMMAND_PARAM;
    }

    let indx = data.format as usize;
    *data = SIMPLE_FORMATS[indx].into();

    return 0;
}

#[cfg(not(feature = "external-xiph-libs"))]
const MAJOR_FORMATS_COUNT: usize = 23;
#[cfg(feature = "external-xiph-libs")]
const MAJOR_FORMATS_COUNT: usize = 25;

static MAJOR_FORMATS: [FormatInfo; 25] = [
    FormatInfo {
        format: SF_FORMAT_AIFF,
        name: c_str!("AIFF (Apple/SGI)"),
        extension: c_str!("aiff"),
    },
    FormatInfo {
        format: SF_FORMAT_AU,
        name: c_str!("AU (Sun/NeXT)"),
        extension: c_str!("au"),
    },
    FormatInfo {
        format: SF_FORMAT_AVR,
        name: c_str!("AVR (Audio Visual Research)"),
        extension: c_str!("avr"),
    },
    FormatInfo {
        format: SF_FORMAT_CAF,
        name: c_str!("CAF (Apple Core Audio File)"),
        extension: c_str!("caf"),
    },
    #[cfg(feature = "external-xiph-libs")]
    FormatInfo {
        format: SF_FORMAT_FLAC,
        name: c_str!("FLAC (Free Lossless Audio Codec)"),
        extension: c_str!("flac"),
    },
    FormatInfo {
        format: SF_FORMAT_HTK,
        name: c_str!("HTK (HMM Tool Kit)"),
        extension: c_str!("htk"),
    },
    FormatInfo {
        format: SF_FORMAT_SVX,
        name: c_str!("IFF (Amiga IFF/SVX8/SV16)"),
        extension: c_str!("iff"),
    },
    FormatInfo {
        format: SF_FORMAT_MAT4,
        name: c_str!("MAT4 (GNU Octave 2.0 / Matlab 4.2)"),
        extension: c_str!("mat"),
    },
    FormatInfo {
        format: SF_FORMAT_MAT5,
        name: c_str!("MAT5 (GNU Octave 2.1 / Matlab 5.0)"),
        extension: c_str!("mat"),
    },
    FormatInfo {
        format: SF_FORMAT_MPC2K,
        name: c_str!("MPC (Akai MPC 2k)"),
        extension: c_str!("mpc"),
    },
    #[cfg(feature = "external-xiph-libs")]
    FormatInfo {
        format: SF_FORMAT_OGG,
        name: c_str!("OGG (OGG Container format)"),
        extension: c_str!("oga"),
    },
    FormatInfo {
        format: SF_FORMAT_PAF,
        name: c_str!("PAF (Ensoniq PARIS)"),
        extension: c_str!("paf"),
    },
    FormatInfo {
        format: SF_FORMAT_PVF,
        name: c_str!("PVF (Portable Voice Format)"),
        extension: c_str!("pvf"),
    },
    FormatInfo {
        format: SF_FORMAT_RAW,
        name: c_str!("RAW (header-less)"),
        extension: c_str!("raw"),
    },
    FormatInfo {
        format: SF_FORMAT_RF64,
        name: c_str!("RF64 (RIFF 64)"),
        extension: c_str!("rf64"),
    },
    FormatInfo {
        format: SF_FORMAT_SD2,
        name: c_str!("SD2 (Sound Designer II)"),
        extension: c_str!("sd2"),
    },
    FormatInfo {
        format: SF_FORMAT_SDS,
        name: c_str!("SDS (Midi Sample Dump Standard)"),
        extension: c_str!("sds"),
    },
    FormatInfo {
        format: SF_FORMAT_IRCAM,
        name: c_str!("SF (Berkeley/IRCAM/CARL)"),
        extension: c_str!("sf"),
    },
    FormatInfo {
        format: SF_FORMAT_VOC,
        name: c_str!("VOC (Creative Labs)"),
        extension: c_str!("voc"),
    },
    FormatInfo {
        format: SF_FORMAT_W64,
        name: c_str!("W64 (SoundFoundry WAVE 64)"),
        extension: c_str!("w64"),
    },
    FormatInfo {
        format: SF_FORMAT_WAV,
        name: c_str!("WAV (Microsoft)"),
        extension: c_str!("wav"),
    },
    FormatInfo {
        format: SF_FORMAT_NIST,
        name: c_str!("WAV (NIST Sphere)"),
        extension: c_str!("wav"),
    },
    FormatInfo {
        format: SF_FORMAT_WAVEX,
        name: c_str!("WAVEX (Microsoft)"),
        extension: c_str!("wav"),
    },
    FormatInfo {
        format: SF_FORMAT_WVE,
        name: c_str!("WVE (Psion Series 3)"),
        extension: c_str!("wve"),
    },
    FormatInfo {
        format: SF_FORMAT_XI,
        name: c_str!("XI (FastTracker 2)"),
        extension: c_str!("xi"),
    },
];

#[no_mangle]
unsafe fn psf_get_format_major_count() {
    MAJOR_FORMATS.len() as c_int;
}

#[no_mangle]
unsafe fn psf_get_format_major(data: *mut SF_FORMAT_INFO) -> c_int {
    assert!(!data.is_null());
    let data = &mut *data;

    if data.format < 0 || data.format as usize >= MAJOR_FORMATS.len() {
        return SFE_BAD_COMMAND_PARAM;
    }

    let indx = data.format;
    *data = MAJOR_FORMATS[indx as usize].into();

    return 0;
}

#[cfg(not(feature = "external-xiph-libs"))]
const SUBTYPE_FORMATS_COUNT: usize = 28;
#[cfg(feature = "external-xiph-libs")]
const SUBTYPE_FORMATS_COUNT: usize = 30;

static SUBTYPE_FORMATS: [FormatInfo; SUBTYPE_FORMATS_COUNT] = [
    FormatInfo {
        format: SF_FORMAT_PCM_S8,
        name: c_str!("Signed 8 bit PCM"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_PCM_16,
        name: c_str!("Signed 16 bit PCM"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_PCM_24,
        name: c_str!("Signed 24 bit PCM"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_PCM_32,
        name: c_str!("Signed 32 bit PCM"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_PCM_U8,
        name: c_str!("Unsigned 8 bit PCM"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_FLOAT,
        name: c_str!("32 bit float"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_DOUBLE,
        name: c_str!("64 bit float"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_ULAW,
        name: c_str!("U-Law"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_ALAW,
        name: c_str!("A-Law"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_IMA_ADPCM,
        name: c_str!("IMA ADPCM"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_MS_ADPCM,
        name: c_str!("Microsoft ADPCM"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_GSM610,
        name: c_str!("GSM 6.10"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_G721_32,
        name: c_str!("32kbs G721 ADPCM"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_G723_24,
        name: c_str!("24kbs G723 ADPCM"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_G723_40,
        name: c_str!("40kbs G723 ADPCM"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_DWVW_12,
        name: c_str!("12 bit DWVW"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_DWVW_16,
        name: c_str!("16 bit DWVW"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_DWVW_24,
        name: c_str!("24 bit DWVW"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_VOX_ADPCM,
        name: c_str!("VOX ADPCM"),
        extension: c_str!("vox"),
    },
    FormatInfo {
        format: SF_FORMAT_NMS_ADPCM_16,
        name: c_str!("16kbs NMS ADPCM"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_NMS_ADPCM_24,
        name: c_str!("24kbs NMS ADPCM"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_NMS_ADPCM_32,
        name: c_str!("32kbs NMS ADPCM"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_DPCM_16,
        name: c_str!("16 bit DPCM"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_DPCM_8,
        name: c_str!("8 bit DPCM"),
        extension: c_str!(""),
    },
    #[cfg(feature = "external-xiph-libs")]
    FormatInfo {
        format: SF_FORMAT_VORBIS,
        name: c_str!("Vorbis"),
        extension: c_str!(""),
    },
    #[cfg(feature = "external-xiph-libs")]
    FormatInfo {
        format: SF_FORMAT_OPUS,
        name: c_str!("Opus"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_ALAC_16,
        name: c_str!("16 bit ALAC"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_ALAC_20,
        name: c_str!("20 bit ALAC"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_ALAC_24,
        name: c_str!("24 bit ALAC"),
        extension: c_str!(""),
    },
    FormatInfo {
        format: SF_FORMAT_ALAC_32,
        name: c_str!("32 bit ALAC"),
        extension: c_str!(""),
    },
];

#[no_mangle]
unsafe fn psf_get_format_subtype_count() -> c_int {
    return SUBTYPE_FORMATS.len() as c_int;
}

#[no_mangle]
unsafe fn psf_get_format_subtype(data: *mut SF_FORMAT_INFO) -> c_int {
    assert!(!data.is_null());
    let data = &mut *data;

    if data.format < 0 || data.format as usize >= SUBTYPE_FORMATS.len() {
        data.format = 0;
        return SFE_BAD_COMMAND_PARAM;
    }

    let indx = data.format as usize;
    *data = SUBTYPE_FORMATS[indx].into();

    return 0;
}

#[no_mangle]
unsafe fn psf_get_format_info(data: *mut SF_FORMAT_INFO) -> c_int {
    assert!(!data.is_null());
    let data = &mut *data;

    if SF_CONTAINER(data.format) != SF_MAJOR_FORMAT::UNKNOWN {
        let format = SF_CONTAINER(data.format);

        for k in 0..MAJOR_FORMATS.len() {
            if format == SF_CONTAINER(MAJOR_FORMATS[k].format) {
                *data = MAJOR_FORMATS[k].into();
                return 0;
            }
        }
    } else if SF_CODEC(data.format) != SF_MINOR_FORMAT::UNKNOWN {
        let format = SF_CODEC(data.format);

        for k in 0..SUBTYPE_FORMATS.len() {
            if format == SF_CODEC(SUBTYPE_FORMATS[k].format) {
                *data = SUBTYPE_FORMATS[k].into();
                return 0;
            }
        }
    }

    *data = SF_FORMAT_INFO::default();

    return SFE_BAD_COMMAND_PARAM;
}

#[no_mangle]
unsafe fn psf_calc_signal_max(psf: *mut SF_PRIVATE, normalize: c_int) -> c_double {
    assert!(!psf.is_null());
    let psf = &mut *psf;

    // If the file is not seekable, there is nothing we can do.
    if psf.sf.seekable == SF_FALSE {
        psf.error = SFE::NOT_SEEKABLE;
        return 0.0;
    }

    if psf.read_double.is_none() {
        psf.error = SFE::UNIMPLEMENTED;
        return 0.0;
    }

    let save_state = sf_command(psf, SFC_GET_NORM_DOUBLE, ptr::null_mut(), 0);
    sf_command(psf, SFC_SET_NORM_DOUBLE, ptr::null_mut(), normalize);

    // Brute force. Read the whole file and find the biggest sample.
    // Get current position in file
    let position = sf_seek(psf, 0, SEEK_CUR);
    // Go to start of file.
    sf_seek(psf, 0, SEEK_SET);

    let mut data = [0f64; SF_BUFFER_LEN / mem::size_of::<f64>()];
    // Make sure len is an integer multiple of the channel count.
    let len = data.len() as c_int - (data.len() as c_int % psf.sf.channels);

    let mut max_val = 0.0;
    loop {
        let readcount = sf_read_double(psf, data.as_mut_ptr(), len as sf_count_t);
        if readcount <= 0 {
            break;
        }
        for k in 0..readcount {
            let temp = data[k as usize].abs();
            max_val = if temp > max_val { temp } else { max_val };
        }
    }

    /* Return to SNDFILE to original state. */
    sf_seek(psf, position, SEEK_SET);
    sf_command(psf, SFC_SET_NORM_DOUBLE, ptr::null_mut(), save_state);

    return max_val;
}

#[no_mangle]
unsafe fn psf_calc_max_all_channels(
    psf: *mut SF_PRIVATE,
    peaks: *mut c_double,
    normalize: c_int,
) -> SFE {
    assert!(!psf.is_null());
    let psf = &mut *psf;

    // If the file is not seekable, there is nothing we can do.
    if psf.sf.seekable == SF_FALSE {
        psf.error = SFE::NOT_SEEKABLE;
        return psf.error;
    }

    if psf.read_double.is_none() {
        psf.error = SFE::UNIMPLEMENTED;
        return psf.error;
    }

    let save_state = sf_command(psf, SFC_GET_NORM_DOUBLE, ptr::null_mut(), 0);
    sf_command(psf, SFC_SET_NORM_DOUBLE, ptr::null_mut(), normalize);

    assert!(psf.sf.channels >= 0);
    let peaks = slice::from_raw_parts_mut(peaks, psf.sf.channels as usize);
    for c in peaks.iter_mut() {
        *c = 0.0;
    }

    // Brute force. Read the whole file and find the biggest sample for each channel. */
    let position = sf_seek(psf, 0, SEEK_CUR); // Get current position in file
    sf_seek(psf, 0, SEEK_SET); // Go to start of file.

    let mut data = [0f64; SF_BUFFER_LEN / mem::size_of::<f64>()];
    let len = data.len() as c_int - (data.len() as c_int % psf.sf.channels);

    let mut chan: c_int = 0;
    let mut readcount = len;
    while readcount > 0 {
        readcount = sf_read_double(psf, data.as_mut_ptr(), len as sf_count_t) as c_int;
        for k in 0..readcount as usize {
            let temp = data[k].abs();
            let peak = peaks[chan as usize];
            peaks[chan as usize] = if temp > peak { temp } else { peak };
            chan = (chan + 1) % psf.sf.channels;
        }
    }

    // Return to original position.
    sf_seek(psf, position, SEEK_SET);

    sf_command(psf, SFC_SET_NORM_DOUBLE, ptr::null_mut(), save_state);

    return SFE::NO_ERROR;
}

#[no_mangle]
unsafe fn psf_get_signal_max(psf: *mut SF_PRIVATE, peak: *mut c_double) -> c_int {
    assert!(!psf.is_null());
    let psf = &mut *psf;

    if psf.peak_info.is_null() {
        return SF_FALSE;
    }

    assert!(!peak.is_null());
    let peak = &mut *peak;

    *peak = (*psf.peak_info).peaks[0].value;
    let peaks = (*psf.peak_info).peaks.as_mut_ptr();
    let peaks = slice::from_raw_parts_mut(peaks, psf.sf.channels as usize);

    for k in 1..psf.sf.channels {
        if peaks[k as usize].value > *peak {
            *peak = peaks[k as usize].value;
        }
    }

    SF_TRUE
}

#[no_mangle]
unsafe fn psf_get_max_all_channels(psf: *mut SF_PRIVATE, peaks: *mut c_double) -> c_int {
    assert!(!psf.is_null());
    let psf = &mut *psf;

    if psf.peak_info.is_null() {
        return SF_FALSE;
    }

    assert!(psf.sf.channels >= 0);
    let peaks = slice::from_raw_parts_mut(peaks, psf.sf.channels as usize);
    let peak_info_peaks = slice::from_raw_parts_mut(
        (*psf.peak_info).peaks.as_mut_ptr(),
        psf.sf.channels as usize,
    );
    for k in 0..psf.sf.channels as usize {
        peaks[k] = peak_info_peaks[k].value;
    }

    return SF_TRUE;
}

#[no_mangle]
pub(crate) unsafe fn psf_strlcpy(dest: *mut c_char, n: size_t, src: *const c_char) {
    strncpy (dest, src, n - 1) ;
	*dest.add(n - 1) = 0 ;
}
