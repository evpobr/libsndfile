use crate::*;

use byte_strings::c_str;
use std::ffi::CStr;

static AIFF_FORMAT_NAME: &CStr = c_str!("AIFF (Apple/SGI 16 bit PCM)");
static AIFF_FORMAT_EXTENSION: &CStr = c_str!("AIFF (Apple/SGI 16 bit PCM)");

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
