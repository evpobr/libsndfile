#![allow(non_camel_case_types, unused_macros, non_snake_case, dead_code)]

use std::ptr;

use libc::{c_void, c_char, c_short, c_int, c_uint, c_double};

mod common;
mod pcm;

/// Microsoft WAV format (little endian default).
pub const SF_FORMAT_WAV: c_int = 0x010000;
/// Apple/SGI AIFF format (big endian).
pub const SF_FORMAT_AIFF: c_int = 0x020000;
/// Sun/NeXT AU format (big endian).
pub const SF_FORMAT_AU: c_int = 0x030000;
/// RAW PCM data.
pub const SF_FORMAT_RAW: c_int = 0x040000;
/// Ensoniq PARIS file format.
pub const SF_FORMAT_PAF: c_int = 0x050000;
/// Amiga IFF / SVX8 / SV16 format.
pub const SF_FORMAT_SVX: c_int = 0x060000;
/// Sphere NIST format.
pub const SF_FORMAT_NIST: c_int = 0x070000;
/// VOC files.
pub const SF_FORMAT_VOC: c_int = 0x080000;
/// Berkeley/IRCAM/CARL
pub const SF_FORMAT_IRCAM: c_int = 0x0A0000;
/// Sonic Foundry's 64 bit RIFF/WAV
pub const SF_FORMAT_W64: c_int = 0x0B0000;
/// Matlab (tm) V4.2 / GNU Octave 2.0
pub const SF_FORMAT_MAT4: c_int = 0x0C0000;
/// Matlab (tm) V5.0 / GNU Octave 2.1
pub const SF_FORMAT_MAT5: c_int = 0x0D0000;
/// Portable Voice Format
pub const SF_FORMAT_PVF: c_int = 0x0E0000;
/// Fasttracker 2 Extended Instrument
pub const SF_FORMAT_XI: c_int = 0x0F0000;
/// HMM Tool Kit format
pub const SF_FORMAT_HTK: c_int = 0x100000;
/// Midi Sample Dump Standard
pub const SF_FORMAT_SDS: c_int = 0x110000;
/// Audio Visual Research
pub const SF_FORMAT_AVR: c_int = 0x120000;
/// MS WAVE with WAVEFORMATEX
pub const SF_FORMAT_WAVEX: c_int = 0x130000;
/// Sound Designer 2
pub const SF_FORMAT_SD2: c_int = 0x160000;
/// FLAC lossless file format
pub const SF_FORMAT_FLAC: c_int = 0x170000;
/// Core Audio File format
pub const SF_FORMAT_CAF: c_int = 0x180000;
/// Psion WVE format
pub const SF_FORMAT_WVE: c_int = 0x190000;
/// Xiph OGG container
pub const SF_FORMAT_OGG: c_int = 0x200000;
/// Akai MPC 2000 sampler
pub const SF_FORMAT_MPC2K: c_int = 0x210000;
/// RF64 WAV file
pub const SF_FORMAT_RF64: c_int = 0x220000;
/// Signed 8 bit data
pub const SF_FORMAT_PCM_S8: c_int = 0x0001;
/// Signed 16 bit data
pub const SF_FORMAT_PCM_16: c_int = 0x0002;
/// Signed 24 bit data
pub const SF_FORMAT_PCM_24: c_int = 0x0003;
/// Signed 32 bit data
pub const SF_FORMAT_PCM_32: c_int = 0x0004;
/// Unsigned 8 bit data (WAV and RAW only)
pub const SF_FORMAT_PCM_U8: c_int = 0x0005;
/// 32 bit float data
pub const SF_FORMAT_FLOAT: c_int = 0x0006;
/// 64 bit float data
pub const SF_FORMAT_DOUBLE: c_int = 0x0007;
/// U-Law encoded.
pub const SF_FORMAT_ULAW: c_int = 0x0010;
/// A-Law encoded.
pub const SF_FORMAT_ALAW: c_int = 0x0011;
/// IMA ADPCM.
pub const SF_FORMAT_IMA_ADPCM: c_int = 0x0012;
/// Microsoft ADPCM.
pub const SF_FORMAT_MS_ADPCM: c_int = 0x0013;
/// GSM 6.10 encoding.
pub const SF_FORMAT_GSM610: c_int = 0x0020;
/// OKI / Dialogix ADPCM
pub const SF_FORMAT_VOX_ADPCM: c_int = 0x0021;
/// 16kbs NMS G721-variant encoding.
pub const SF_FORMAT_NMS_ADPCM_16: c_int = 0x0022;
/// 24kbs NMS G721-variant encoding.
pub const SF_FORMAT_NMS_ADPCM_24: c_int = 0x0023;
/// 32kbs NMS G721-variant encoding.
pub const SF_FORMAT_NMS_ADPCM_32: c_int = 0x0024;
/// 32kbs G721 ADPCM encoding.
pub const SF_FORMAT_G721_32: c_int = 0x0030;
/// 24kbs G723 ADPCM encoding.
pub const SF_FORMAT_G723_24: c_int = 0x0031;
/// 40kbs G723 ADPCM encoding.
pub const SF_FORMAT_G723_40: c_int = 0x0032;
/// 12 bit Delta Width Variable Word encoding.
pub const SF_FORMAT_DWVW_12: c_int = 0x0040;
/// 16 bit Delta Width Variable Word encoding.
pub const SF_FORMAT_DWVW_16: c_int = 0x0041;
/// 24 bit Delta Width Variable Word encoding.
pub const SF_FORMAT_DWVW_24: c_int = 0x0042;
/// N bit Delta Width Variable Word encoding.
pub const SF_FORMAT_DWVW_N: c_int = 0x0043;
/// 8 bit differential PCM (XI only)
pub const SF_FORMAT_DPCM_8: c_int = 0x0050;
/// 16 bit differential PCM (XI only)
pub const SF_FORMAT_DPCM_16: c_int = 0x0051;
/// Xiph Vorbis encoding.
pub const SF_FORMAT_VORBIS: c_int = 0x0060;
/// Xiph/Skype Opus encoding.
pub const SF_FORMAT_OPUS: c_int = 0x0064;
/// Apple Lossless Audio Codec (16 bit).
pub const SF_FORMAT_ALAC_16: c_int = 0x0070;
/// Apple Lossless Audio Codec (20 bit).
pub const SF_FORMAT_ALAC_20: c_int = 0x0071;
/// Apple Lossless Audio Codec (24 bit).
pub const SF_FORMAT_ALAC_24: c_int = 0x0072;
/// Apple Lossless Audio Codec (32 bit).
pub const SF_FORMAT_ALAC_32: c_int = 0x0073;
/// Default file endian-ness.
pub const SF_ENDIAN_FILE: c_int = 0x00000000;
/// Force little endian-ness.
pub const SF_ENDIAN_LITTLE: c_int = 0x10000000;
/// Force big endian-ness.
pub const SF_ENDIAN_BIG: c_int = 0x20000000;
/// Force CPU endian-ness.
pub const SF_ENDIAN_CPU: c_int = 0x30000000;
pub const SF_FORMAT_SUBMASK: c_int = 0x0000FFFF;
pub const SF_FORMAT_TYPEMASK: c_int = 0x0FFF0000;
pub const SF_FORMAT_ENDMASK: c_int = 0x30000000;

pub const SFC_GET_LIB_VERSION: c_int = 0x1000;
pub const SFC_GET_LOG_INFO: c_int = 0x1001;
pub const SFC_GET_CURRENT_SF_INFO: c_int = 0x1002;

pub const SFC_GET_NORM_DOUBLE: c_int = 0x1010;
pub const SFC_GET_NORM_FLOAT: c_int = 0x1011;
pub const SFC_SET_NORM_DOUBLE: c_int = 0x1012;
pub const SFC_SET_NORM_FLOAT: c_int = 0x1013;
pub const SFC_SET_SCALE_FLOAT_INT_READ: c_int = 0x1014;
pub const SFC_SET_SCALE_INT_FLOAT_WRITE: c_int = 0x1015;

pub const SFC_GET_SIMPLE_FORMAT_COUNT: c_int = 0x1020;
pub const SFC_GET_SIMPLE_FORMAT: c_int = 0x1021;

pub const SFC_GET_FORMAT_INFO: c_int = 0x1028;

pub const SFC_GET_FORMAT_MAJOR_COUNT: c_int = 0x1030;
pub const SFC_GET_FORMAT_MAJOR: c_int = 0x1031;
pub const SFC_GET_FORMAT_SUBTYPE_COUNT: c_int = 0x1032;
pub const SFC_GET_FORMAT_SUBTYPE: c_int = 0x1033;

pub const SFC_CALC_SIGNAL_MAX: c_int = 0x1040;
pub const SFC_CALC_NORM_SIGNAL_MAX: c_int = 0x1041;
pub const SFC_CALC_MAX_ALL_CHANNELS: c_int = 0x1042;
pub const SFC_CALC_NORM_MAX_ALL_CHANNELS: c_int = 0x1043;
pub const SFC_GET_SIGNAL_MAX: c_int = 0x1044;
pub const SFC_GET_MAX_ALL_CHANNELS: c_int = 0x1045;

pub const SFC_SET_ADD_PEAK_CHUNK: c_int = 0x1050;

pub const SFC_UPDATE_HEADER_NOW: c_int = 0x1060;
pub const SFC_SET_UPDATE_HEADER_AUTO: c_int = 0x1061;

pub const SFC_FILE_TRUNCATE: c_int = 0x1080;

pub const SFC_SET_RAW_START_OFFSET: c_int = 0x1090;

pub const SFC_SET_DITHER_ON_WRITE: c_int = 0x10A0;
pub const SFC_SET_DITHER_ON_READ: c_int = 0x10A1;

pub const SFC_GET_DITHER_INFO_COUNT: c_int = 0x10A2;
pub const SFC_GET_DITHER_INFO: c_int = 0x10A3;

pub const SFC_GET_EMBED_FILE_INFO: c_int = 0x10B0;

pub const SFC_SET_CLIPPING: c_int = 0x10C0;
pub const SFC_GET_CLIPPING: c_int = 0x10C1;

pub const SFC_GET_CUE_COUNT: c_int = 0x10CD;
pub const SFC_GET_CUE: c_int = 0x10CE;
pub const SFC_SET_CUE: c_int = 0x10CF;

pub const SFC_GET_INSTRUMENT: c_int = 0x10D0;
pub const SFC_SET_INSTRUMENT: c_int = 0x10D1;

pub const SFC_GET_LOOP_INFO: c_int = 0x10E0;

pub const SFC_GET_BROADCAST_INFO: c_int = 0x10F0;
pub const SFC_SET_BROADCAST_INFO: c_int = 0x10F1;

pub const SFC_GET_CHANNEL_MAP_INFO: c_int = 0x1100;
pub const SFC_SET_CHANNEL_MAP_INFO: c_int = 0x1101;

pub const SFC_RAW_DATA_NEEDS_ENDSWAP: c_int = 0x1110;

// Support for Wavex Ambisonics Format
pub const SFC_WAVEX_SET_AMBISONIC: c_int = 0x1200;
pub const SFC_WAVEX_GET_AMBISONIC: c_int = 0x1201;

/*
 * RF64 files can be set so that on-close; writable files that have less
 * than 4GB of data in them are converted to RIFF/WAV; as per EBU
 * recommendations.
 */
pub const SFC_RF64_AUTO_DOWNGRADE: c_int = 0x1210;

pub const SFC_SET_VBR_ENCODING_QUALITY: c_int = 0x1300;
pub const SFC_SET_COMPRESSION_LEVEL: c_int = 0x1301;

/* Cart Chunk support */
pub const SFC_SET_CART_INFO: c_int = 0x1400;
pub const SFC_GET_CART_INFO: c_int = 0x1401;

/* Opus files original samplerate metadata */
pub const SFC_SET_ORIGINAL_SAMPLERATE: c_int = 0x1500;
pub const SFC_GET_ORIGINAL_SAMPLERATE: c_int = 0x1501;

/* Following commands for testing only. */
pub const SFC_TEST_IEEE_FLOAT_REPLACE: c_int = 0x6001;

/*
 * These SFC_SET_ADD_* values are deprecated and will disappear at some
 * time in the future. They are guaranteed to be here up to and
 * including version 1.0.8 to avoid breakage of existing software.
 * They currently do nothing and will continue to do nothing.
 */
pub const SFC_SET_ADD_HEADER_PAD_CHUNK: c_int = 0x1051;

pub const SFC_SET_ADD_DITHER_ON_WRITE: c_int = 0x1070;
pub const SFC_SET_ADD_DITHER_ON_READ: c_int = 0x1071;

/*
 * String types that can be set and read from files. Not all file types
 * support this and even the file types which support one, may not support
 * all string types.
 */

pub const SF_STR_TITLE: c_int = 0x01;
pub const SF_STR_COPYRIGHT: c_int = 0x02;
pub const SF_STR_SOFTWARE: c_int = 0x03;
pub const SF_STR_ARTIST: c_int = 0x04;
pub const SF_STR_COMMENT: c_int = 0x05;
pub const SF_STR_DATE: c_int = 0x06;
pub const SF_STR_ALBUM: c_int = 0x07;
pub const SF_STR_LICENSE: c_int = 0x08;
pub const SF_STR_TRACKNUMBER: c_int = 0x09;
pub const SF_STR_GENRE: c_int = 0x10;

/*
 * Use the following as the start and end index when doing metadata
 * transcoding.
 */

pub const SF_STR_FIRST: c_int = SF_STR_TITLE;
pub const SF_STR_LAST: c_int = SF_STR_GENRE;

// True and false

pub const SF_FALSE: c_int = 0;
pub const SF_TRUE: c_int = 1;

// Modes for opening files.

pub const SFM_READ: c_int = 0x10;
pub const SFM_WRITE: c_int = 0x20;
pub const SFM_RDWR: c_int = 0x30;

pub const SF_AMBISONIC_NONE: c_int = 0x40;
pub const SF_AMBISONIC_B_FORMAT: c_int = 0x41;

/* Public error values. These are guaranteed to remain unchanged for the duration
 * of the library major version number.
 * There are also a large number of private error numbers which are internal to
 * the library which can change at any time.
 */

pub const SF_ERR_NO_ERROR: c_int = 0;
pub const SF_ERR_UNRECOGNISED_FORMAT: c_int = 1;
pub const SF_ERR_SYSTEM: c_int = 2;
pub const SF_ERR_MALFORMED_FILE: c_int = 3;
pub const SF_ERR_UNSUPPORTED_ENCODING: c_int = 4;

// Channel map values (used with SFC_SET/GET_CHANNEL_MAP).

pub const SF_CHANNEL_MAP_INVALID: c_int = 0;
pub const SF_CHANNEL_MAP_MONO: c_int = 1;
/* Apple calls this 'Left' */
pub const SF_CHANNEL_MAP_LEFT: c_int = 2;
/* Apple calls this 'Right' */
pub const SF_CHANNEL_MAP_RIGHT: c_int = 3;
/* Apple calls this 'Center' */
pub const SF_CHANNEL_MAP_CENTER: c_int = 4;
pub const SF_CHANNEL_MAP_FRONT_LEFT: c_int = 5;
pub const SF_CHANNEL_MAP_FRONT_RIGHT: c_int = 6;
pub const SF_CHANNEL_MAP_FRONT_CENTER: c_int = 7;
/* Apple calls this 'Center Surround', Msft calls this 'Back Center' */
pub const SF_CHANNEL_MAP_REAR_CENTER: c_int = 8;
/* Apple calls this 'Left Surround', Msft calls this 'Back Left' */
pub const SF_CHANNEL_MAP_REAR_LEFT: c_int = 9;
/* Apple calls this 'Right Surround', Msft calls this 'Back Right' */
pub const SF_CHANNEL_MAP_REAR_RIGHT: c_int = 10;
/* Apple calls this 'LFEScreen', Msft calls this 'Low Frequency'  */
pub const SF_CHANNEL_MAP_LFE: c_int = 11;
/* Apple calls this 'Left Center' */
pub const SF_CHANNEL_MAP_FRONT_LEFT_OF_CENTER: c_int = 12;
/* Apple calls this 'Right Center */
pub const SF_CHANNEL_MAP_FRONT_RIGHT_OF_CENTER: c_int = 13;
/* Apple calls this 'Left Surround Direct' */
pub const SF_CHANNEL_MAP_SIDE_LEFT: c_int = 14;
/* Apple calls this 'Right Surround Direct' */
pub const SF_CHANNEL_MAP_SIDE_RIGHT: c_int = 15;
/* Apple calls this 'Top Center Surround' */
pub const SF_CHANNEL_MAP_TOP_CENTER: c_int = 16;
/* Apple calls this 'Vertical Height Left' */
pub const SF_CHANNEL_MAP_TOP_FRONT_LEFT: c_int = 17;
/* Apple calls this 'Vertical Height Right' */
pub const SF_CHANNEL_MAP_TOP_FRONT_RIGHT: c_int = 18;
/* Apple calls this 'Vertical Height Center' */
pub const SF_CHANNEL_MAP_TOP_FRONT_CENTER: c_int = 19;
/* Apple and MS call this 'Top Back Left' */
pub const SF_CHANNEL_MAP_TOP_REAR_LEFT: c_int = 20;
/* Apple and MS call this 'Top Back Right' */
pub const SF_CHANNEL_MAP_TOP_REAR_RIGHT: c_int = 21;
/* Apple and MS call this 'Top Back Center' */
pub const SF_CHANNEL_MAP_TOP_REAR_CENTER: c_int = 22;

pub const SF_CHANNEL_MAP_AMBISONIC_B_W: c_int = 23;
pub const SF_CHANNEL_MAP_AMBISONIC_B_X: c_int = 24;
pub const SF_CHANNEL_MAP_AMBISONIC_B_Y: c_int = 25;
pub const SF_CHANNEL_MAP_AMBISONIC_B_Z: c_int = 26;

pub const SF_CHANNEL_MAP_MAX: c_int = 27;

// A SNDFILE* pointer can be passed around much like stdio.h's FILE* pointer.

pub type SNDFILE = c_void;

/* The following typedef is system specific and is defined when libsndfile is
 * compiled. sf_count_t will be a 64 bit value when the underlying OS allows
 * 64 bit file offsets.
 * On windows, we need to allow the same header file to be compiler by both GCC
 * and the Microsoft compiler.
 */

pub type sf_count_t = i64;

/// A pointer to a `SF_INFO` structure is passed to sf_open () and filled in.
/// On write, the `SF_INFO` structure is filled in by the user and passed into
/// `sf_open()`.
#[repr(C)]
#[derive(Default, Debug)]
pub struct SF_INFO {
    /// Used to be called samples.  Changed to avoid confusion. */
    pub frames: sf_count_t,
    pub samplerate: c_int,
    pub channels: c_int,
    pub format: c_int,
    pub sections: c_int,
    pub seekable: c_int,
}

/// The `SF_FORMAT_INFO` struct is used to retrieve information about the sound
/// file formats libsndfile supports using the sf_command () interface.
///
/// Using this interface will allow applications to support new file formats
/// and encoding types when libsndfile is upgraded, without requiring
/// re-compilation of the application.
///
/// Please consult the libsndfile documentation (particularly the information
/// on the `sf_command ()` interface) for examples of its use.
#[repr(C)]
pub struct SF_FORMAT_INFO {
    format: c_int,
    pub name: *const c_char,
    pub extension: *const c_char,
}

/*
 * Enums and typedefs for adding dither on read and write.
 * See the html documentation for sf_command(), SFC_SET_DITHER_ON_WRITE
 * and SFC_SET_DITHER_ON_READ.
 */

pub const SFD_DEFAULT_LEVEL: c_int = 0;
pub const SFD_CUSTOM_LEVEL: c_int = 0x40000000;

pub const SFD_NO_DITHER: c_int = 500;
pub const SFD_WHITE: c_int = 501;
pub const SFD_TRIANGULAR_PDF: c_int = 502;

#[repr(C)]
pub struct SF_DITHER_INFO {
    pub r#type: c_int,
    pub level: c_double,
    pub name: *mut c_char,
}

impl Default for SF_DITHER_INFO {
    fn default() -> Self {
        SF_DITHER_INFO {
            r#type: 0,
            level: 0.0,
            name: ptr::null_mut(),
        }
    }
}

/// Struct used to retrieve information about a file embedded within a larger file.
/// See SFC_GET_EMBED_FILE_INFO.
#[repr(C)]
pub struct SF_EMBED_FILE_INFO {
    pub offset: sf_count_t,
	pub length: sf_count_t,
}

/// Struct used to retrieve cue marker information from a file
#[repr(C)]
pub struct SF_CUE_POINT {
    pub indx: i32,
    pub position: u32,
    pub fcc_chunk: i32,
    pub chunk_start: i32,
    pub block_start: i32,
    pub sample_offset: u32,
    pub name: [c_char; 256],
}

#[macro_export]
macro_rules! SF_CUES_VAR {
    ($count: expr, $name: ident) => {
        #[repr(C)]
        pub struct $name {
            pub cue_count: u32,
            pub cue_points: [SF_CUE_POINT; $count],
        }
    };
}

SF_CUES_VAR!(100, SF_CUES);

/*
 * Structs used to retrieve music sample information from a file.
*/

/*
 * The loop mode field in SF_INSTRUMENT will be one of the following.
 */
pub const SF_LOOP_NONE: c_int = 800;
pub const SF_LOOP_FORWARD: c_int = 801;
pub const SF_LOOP_BACKWARD: c_int = 802;
pub const SF_LOOP_ALTERNATING: c_int = 803;

#[repr(C)]
pub struct SF_INSTRUMENT_LOOPS {
    pub mode: c_int,
    pub start: u32,
    pub end: u32,
    pub count: u32,
}

#[repr(C)]
pub struct SF_INSTRUMENT {
    pub gain: c_int,
    pub basenote: c_char,
    pub detune: c_char,
    pub velocity_lo: c_char,
    pub velocity_hi: c_char,
    pub key_lo: c_char,
    pub key_hi: c_char,
    pub loop_count: c_int,
    pub loops: [SF_INSTRUMENT_LOOPS; 16],
}

/* Struct used to retrieve loop information from a file.*/
#[repr(C)]
pub struct SF_LOOP_INFO {
    pub time_sig_num: c_short, /* any positive integer    > 0  */
    pub time_sig_den: c_short, /* any positive power of 2 > 0  */
    pub loop_mode: c_int,      /* see SF_LOOP enum             */

    pub num_beats: c_int, /* this is NOT the amount of quarter notes !!!*/
    /* a full bar of 4/4 is 4 beats */
    /* a full bar of 7/8 is 7 beats */
    pub bpm: f32, /* suggestion, as it can be calculated using other fields:*/
    /* file's length, file's sampleRate and our time_sig_den*/
    /* -> bpms are always the amount of _quarter notes_ per minute */
    pub root_key: c_int, /* MIDI note, or -1 for None */
    pub future: [c_int; 6],
}

/*	Struct used to retrieve broadcast (EBU) information from a file.
**	Strongly (!) based on EBU "bext" chunk format used in Broadcast WAVE.
*/
#[macro_export]
macro_rules! SF_BROADCAST_INFO_VAR {
    ($coding_hist_size:expr, $name:ident) => {
        #[repr(C)]
        pub struct $name {
            pub description: [c_char; 256],
            pub originator: [c_char; 32],
            pub originator_reference: [c_char; 32],
            pub origination_date: [c_char; 10],
            pub origination_time: [c_char; 8],
            pub time_reference_low: u32,
            pub time_reference_high: u32,
            pub version: c_short,
            pub umid: [c_char; 64],
            pub loudness_value: i16,
            pub loudness_range: i16,
            pub max_true_peak_level: i16,
            pub max_momentary_loudness: i16,
            pub max_shortterm_loudness: i16,
            pub reserved: [c_char; 180],
            pub coding_history_size: u32,
            pub coding_history: [c_char; $coding_hist_size],
        }
    };
}

/* SF_BROADCAST_INFO is the above struct with coding_history field of 256 bytes. */
SF_BROADCAST_INFO_VAR!(256, SF_BROADCAST_INFO);

#[repr(C)]
pub struct SF_CART_TIMER {
    pub usage: [c_char; 4],
    pub value: i32,
}

#[macro_export]
macro_rules! SF_CART_INFO_VAR {
    ($p_tag_text_size:expr, $name:ident) => {
        #[repr(C)]
        pub struct $name {
            pub version: [c_char; 4],
            pub title: [c_char; 64],
            pub artist: [c_char; 64],
            pub cut_id: [c_char; 64],
            pub client_id: [c_char; 64],
            pub category: [c_char; 64],
            pub classification: [c_char; 64],
            pub out_cue: [c_char; 64],
            pub start_date: [c_char; 10],
            pub start_time: [c_char; 8],
            pub end_date: [c_char; 10],
            pub end_time: [c_char; 8],
            pub producer_app_id: [c_char; 64],
            pub producer_app_version: [c_char; 64],
            pub user_def: [c_char; 64],
            pub level_reference: i32,
            pub post_timers: [SF_CART_TIMER; 8],
            pub reserved: [c_char; 276],
            pub url: [c_char; 1024],
            pub tag_text_size: u32,
            pub tag_text: [c_char; $p_tag_text_size],
        }
    };
}

SF_CART_INFO_VAR!(256, SF_CART_INFO);

// Virtual I/O functionality.

pub type sf_vio_get_filelen = unsafe extern "C" fn(user_data: *mut c_void) -> sf_count_t;
pub type sf_vio_seek =
    unsafe extern "C" fn(offset: sf_count_t, whence: c_int, user_data: *mut c_void) -> sf_count_t;
pub type sf_vio_read =
    unsafe extern "C" fn(ptr: *mut c_void, count: sf_count_t, user_data: *mut c_void) -> sf_count_t;
pub type sf_vio_write = unsafe extern "C" fn(
    ptr: *const c_void,
    count: sf_count_t,
    user_data: *mut c_void,
) -> sf_count_t;
pub type sf_vio_tell = unsafe extern "C" fn(user_data: *mut c_void) -> sf_count_t;

#[repr(C)]
#[derive(Default)]
pub struct SF_VIRTUAL_IO {
    pub get_filelen: Option<sf_vio_get_filelen>,
    pub seek: Option<sf_vio_seek>,
    pub read: Option<sf_vio_read>,
    pub write: Option<sf_vio_write>,
    pub tell: Option<sf_vio_tell>,
}

const SF_SEEK_SET: c_int = libc::SEEK_SET;
const SF_SEEK_CUR: c_int = libc::SEEK_CUR;
const SF_SEEK_END: c_int = libc::SEEK_END;

#[repr(C)]
pub struct SF_CHUNK_INFO {
    pub id: [c_char; 64],  /* The chunk identifier. */
    pub id_size: c_uint,   /* The size of the chunk identifier. */
    pub datalen: c_uint,   /* The size of that data. */
    pub data: *mut c_void, /* Pointer to the data. */
}
