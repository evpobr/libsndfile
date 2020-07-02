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
