#![allow(non_camel_case_types, unused_macros, non_snake_case, dead_code)]

use std::cmp;
use std::mem;
use std::{ffi::CStr, ptr};

use common::*;
use libc::*;

use byte_strings::c_str;

mod audio_detect;
#[macro_use]
mod common;
mod file_io;
mod htk;
mod id3;
mod strings;

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

#[repr(C)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum SFM_OPEN_MODE {
    READ = 0x10,
    WRITE = 0x20,
    RDWR = 0x30,
}

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
pub type sf_vio_seek = unsafe extern "C" fn(
    offset: sf_count_t,
    whence: SF_SEEK_MODE,
    user_data: *mut c_void,
) -> sf_count_t;
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

const INITIAL_HEADER_SIZE: sf_count_t = 256;

/* Allocate and initialize the SF_PRIVATE struct. */
#[no_mangle]
unsafe fn psf_allocate() -> *mut SF_PRIVATE {
    let psf = calloc(1, mem::size_of::<SF_PRIVATE>()) as *mut SF_PRIVATE;
    if psf.is_null() {
        return ptr::null_mut();
    }

    (*psf).header.ptr = calloc(1, INITIAL_HEADER_SIZE as size_t) as *mut u8;
    if (*psf).header.ptr.is_null() {
        free(psf as *mut c_void);
        return ptr::null_mut();
    }
    (*psf).header.len = INITIAL_HEADER_SIZE;

    return psf;
}

#[no_mangle]
unsafe fn psf_bump_header_allocation(psf: *mut SF_PRIVATE, needed: sf_count_t) -> c_int {
    assert!(!psf.is_null());
    let psf = &mut *psf;

    // sf_count_t newlen
    let smallest = INITIAL_HEADER_SIZE;
    // void * ptr ;

    let newlen = if needed > psf.header.len {
        2 * cmp::max(needed, smallest)
    } else {
        2 * psf.header.len
    };

    if newlen > 100 * 1024 {
        psf_log_printf(
            psf,
            c_str!("Request for header allocation of %D denied.\n").as_ptr(),
            newlen,
        );
        return 1;
    }

    let ptr = realloc(psf.header.ptr as _, newlen as size_t);
    if ptr.is_null() {
        psf_log_printf(
            psf,
            c_str!("realloc (%p, %D) failed\n").as_ptr(),
            psf.header.ptr,
            newlen,
        );
        psf.error = SFE_MALLOC_FAILED;
        return 1;
    }

    /* Always zero-out new header memory to avoid un-initializer memory accesses. */
    if newlen > psf.header.len {
        memset(
            ptr.offset(psf.header.len as isize),
            0,
            (newlen - psf.header.len) as size_t,
        );
    }

    psf.header.ptr = ptr as _;
    psf.header.len = newlen;

    0
}

#[no_mangle]
unsafe fn psf_close(psf: *mut SF_PRIVATE) -> c_int {
    assert!(!psf.is_null());
    let psf = &mut *psf;

    let mut error: c_int;

    if let Some(codec_close) = psf.codec_close {
        error = codec_close(psf);
        /* To prevent it being called in psf->container_close(). */
        psf.codec_close = None;
    }

    if let Some(container_close) = psf.container_close {
        error = container_close(psf);
    }

    error = psf_fclose(psf);
    psf_close_rsrc(psf);

    /* For an ISO C compliant implementation it is ok to free a NULL pointer. */
    free(psf.header.ptr as *mut c_void);
    free(psf.container_data as *mut c_void);
    free(psf.codec_data as *mut c_void);
    free(psf.interleave as *mut c_void);
    free(psf.dither as *mut c_void);
    free(psf.peak_info as *mut c_void);
    free(psf.broadcast_16k as *mut c_void);
    free(psf.loop_info as *mut c_void);
    free(psf.instrument as *mut c_void);
    free(psf.cues as *mut c_void);
    free(psf.channel_map as *mut c_void);
    free(psf.format_desc as *mut c_void);
    free(psf.strings.storage as *mut c_void);

    if !psf.wchunks.chunks.is_null() {
        for k in 0..psf.wchunks.used as isize {
            free((*psf.wchunks.chunks.offset(k)).data);
        }
    }

    free(psf.rchunks.chunks as *mut c_void);
    free(psf.wchunks.chunks as *mut c_void);
    free(psf.iterator as *mut c_void);
    free(psf.cart_16k as *mut c_void);

    free(psf as *mut SF_PRIVATE as *mut c_void);

    error
}

const SNDFILE_MAGICK: c_int = 0x1234C0DE;

#[repr(C)]
struct ErrorStruct<'a> {
    pub error: c_int,
    pub r#str: &'a CStr,
}

impl<'a> ErrorStruct<'a> {
    pub fn new(error: c_int, r#str: &'a CStr) -> ErrorStruct {
        ErrorStruct { error, r#str }
    }
}

static SndfileErrors: [ErrorStruct; 179] = [
    ErrorStruct { error: SF_ERR_NO_ERROR, r#str: c_str!("No Error.")},
	ErrorStruct {	error: SF_ERR_UNRECOGNISED_FORMAT	, r#str: c_str!("Format not recognised.") },
	ErrorStruct {	error: SF_ERR_SYSTEM				, r#str: c_str!("System error.") /* Often replaced. */ 	},
	ErrorStruct {	error: SF_ERR_MALFORMED_FILE		, r#str: c_str!("Supported file format but file is malformed.") },
	ErrorStruct {	error: SF_ERR_UNSUPPORTED_ENCODING	, r#str: c_str!("Supported file format but unsupported encoding.") },

	/* Private error values and their associated strings. */
	ErrorStruct {	error: SFE_ZERO_MAJOR_FORMAT	, r#str: c_str!("Error : major format is 0.") },
	ErrorStruct {	error: SFE_ZERO_MINOR_FORMAT	, r#str: c_str!("Error : minor format is 0.") },
	ErrorStruct {	error: SFE_BAD_FILE			, r#str: c_str!("File does not exist or is not a regular file (possibly a pipe?).") },
	ErrorStruct {	error: SFE_BAD_FILE_READ		, r#str: c_str!("File exists but no data could be read.") },
	ErrorStruct {	error: SFE_OPEN_FAILED			, r#str: c_str!("Could not open file.") },
	ErrorStruct {	error: SFE_BAD_SNDFILE_PTR		, r#str: c_str!("Not a valid SNDFILE* pointer.") },
	ErrorStruct {	error: SFE_BAD_SF_INFO_PTR		, r#str: c_str!("NULL SF_INFO pointer passed to libsndfile.") },
	ErrorStruct {	error: SFE_BAD_SF_INCOMPLETE	, r#str: c_str!("SF_PRIVATE struct incomplete and end of header parsing.") },
	ErrorStruct {	error: SFE_BAD_FILE_PTR		, r#str: c_str!("Bad FILE pointer.") },
	ErrorStruct {	error: SFE_BAD_INT_PTR			, r#str: c_str!("Internal error, Bad pointer.") },
	ErrorStruct {	error: SFE_BAD_STAT_SIZE		, r#str: c_str!("Error : software was misconfigured at compile time (sizeof statbuf.st_size).") },
	ErrorStruct {	error: SFE_NO_TEMP_DIR			, r#str: c_str!("Error : Could not file temp dir.") },

	ErrorStruct {	error: SFE_MALLOC_FAILED		, r#str: c_str!("Internal malloc () failed.") },
	ErrorStruct {	error: SFE_UNIMPLEMENTED		, r#str: c_str!("File contains data in an unimplemented format.") },
	ErrorStruct {	error: SFE_BAD_READ_ALIGN		, r#str: c_str!("Attempt to read a non-integer number of channels.") },
	ErrorStruct {	error: SFE_BAD_WRITE_ALIGN 	    , r#str: c_str!("Attempt to write a non-integer number of channels.") },
	ErrorStruct {	error: SFE_NOT_READMODE		    , r#str: c_str!("Read attempted on file currently open for write.") },
	ErrorStruct {	error: SFE_NOT_WRITEMODE		, r#str: c_str!("Write attempted on file currently open for read.") },
	ErrorStruct {	error: SFE_BAD_MODE_RW			, r#str: c_str!("Error : This file format does not support read/write mode.") },
	ErrorStruct {	error: SFE_BAD_SF_INFO			, r#str: c_str!("Internal error : SF_INFO struct incomplete.") },
	ErrorStruct {	error: SFE_BAD_OFFSET			, r#str: c_str!("Error : supplied offset beyond end of file.") },
	ErrorStruct {	error: SFE_NO_EMBED_SUPPORT     , r#str: c_str!("Error : embedding not supported for this file format.") },
	ErrorStruct {	error: SFE_NO_EMBEDDED_RDWR     , r#str: c_str!("Error : cannot open embedded file read/write.") },
	ErrorStruct {	error: SFE_NO_PIPE_WRITE		, r#str: c_str!("Error : this file format does not support pipe write.") },
	ErrorStruct {	error: SFE_BAD_VIRTUAL_IO		, r#str: c_str!("Error : bad pointer on SF_VIRTUAL_IO struct.") },
	ErrorStruct {	error: SFE_BAD_BROADCAST_INFO_SIZE, r#str: c_str!("Error : bad coding_history_size in SF_BROADCAST_INFO struct.") },
    ErrorStruct {	error: SFE_BAD_BROADCAST_INFO_TOO_BIG, r#str: c_str!("Error : SF_BROADCAST_INFO struct too large.") },
	ErrorStruct {	error: SFE_BAD_CART_INFO_SIZE				, r#str: c_str!("Error: SF_CART_INFO struct too large.") },
	ErrorStruct {	error: SFE_BAD_CART_INFO_TOO_BIG			, r#str: c_str!("Error: bad tag_text_size in SF_CART_INFO struct.") },
	ErrorStruct {	error: SFE_INTERLEAVE_MODE		, r#str: c_str!("Attempt to write to file with non-interleaved data.") },
	ErrorStruct {	error: SFE_INTERLEAVE_SEEK		, r#str: c_str!("Bad karma in seek during interleave read operation.") },
	ErrorStruct {	error: SFE_INTERLEAVE_READ		, r#str: c_str!("Bad karma in read during interleave read operation.") },

	ErrorStruct {	error: SFE_INTERNAL			, r#str: c_str!("Unspecified internal error.") },
	ErrorStruct {	error: SFE_BAD_COMMAND_PARAM	, r#str: c_str!("Bad parameter passed to function sf_command.") },
	ErrorStruct {	error: SFE_BAD_ENDIAN			, r#str: c_str!("Bad endian-ness. Try default endian-ness") },
	ErrorStruct {	error: SFE_CHANNEL_COUNT_ZERO	, r#str: c_str!("Channel count is zero.") },
	ErrorStruct {	error: SFE_CHANNEL_COUNT		, r#str: c_str!("Too many channels specified.") },
	ErrorStruct {	error: SFE_CHANNEL_COUNT_BAD	, r#str: c_str!("Bad channel count.") },

	ErrorStruct {	error: SFE_BAD_SEEK			    , r#str: c_str!("Internal psf_fseek() failed.") },
	ErrorStruct {	error: SFE_NOT_SEEKABLE		    , r#str: c_str!("Seek attempted on unseekable file type.") },
	ErrorStruct {	error: SFE_AMBIGUOUS_SEEK		, r#str: c_str!("Error : combination of file open mode and seek command is ambiguous.") },
	ErrorStruct {	error: SFE_WRONG_SEEK			, r#str: c_str!("Error : invalid seek parameters.") },
	ErrorStruct {	error: SFE_SEEK_FAILED			, r#str: c_str!("Error : parameters OK, but psf_seek() failed.") },

	ErrorStruct {	error: SFE_BAD_OPEN_MODE		, r#str: c_str!("Error : bad mode parameter for file open.") },
	ErrorStruct {	error: SFE_OPEN_PIPE_RDWR		, r#str: c_str!("Error : attempt to open a pipe in read/write mode.") },
	ErrorStruct {	error: SFE_RDWR_POSITION		, r#str: c_str!("Error on RDWR position (cryptic).") },
	ErrorStruct {	error: SFE_RDWR_BAD_HEADER		, r#str: c_str!("Error : Cannot open file in read/write mode due to string data in header.") },
	ErrorStruct {	error: SFE_CMD_HAS_DATA		    , r#str: c_str!("Error : Command fails because file already has audio data.") },

	ErrorStruct {	error: SFE_STR_NO_SUPPORT		, r#str: c_str!("Error : File type does not support string data.") },
	ErrorStruct {	error: SFE_STR_NOT_WRITE		, r#str: c_str!("Error : Trying to set a string when file is not in write mode.") },
	ErrorStruct {	error: SFE_STR_MAX_DATA		    , r#str: c_str!("Error : Maximum string data storage reached.") },
	ErrorStruct {	error: SFE_STR_MAX_COUNT		, r#str: c_str!("Error : Maximum string data count reached.") },
	ErrorStruct {	error: SFE_STR_BAD_TYPE		    , r#str: c_str!("Error : Bad string data type.") },
	ErrorStruct {	error: SFE_STR_NO_ADD_END		, r#str: c_str!("Error : file type does not support strings added at end of file.") },
	ErrorStruct {	error: SFE_STR_BAD_STRING		, r#str: c_str!("Error : bad string.") },
	ErrorStruct {	error: SFE_STR_WEIRD			, r#str: c_str!("Error : Weird string error.") },

	ErrorStruct {	error: SFE_WAV_NO_RIFF			, r#str: c_str!("Error in WAV file. No 'RIFF' chunk marker.") },
	ErrorStruct {	error: SFE_WAV_NO_WAVE			, r#str: c_str!("Error in WAV file. No 'WAVE' chunk marker.") },
	ErrorStruct {	error: SFE_WAV_NO_FMT			, r#str: c_str!("Error in WAV/W64/RF64 file. No 'fmt ' chunk marker.") },
	ErrorStruct {	error: SFE_WAV_BAD_FMT			, r#str: c_str!("Error in WAV/W64/RF64 file. Malformed 'fmt ' chunk.") },
	ErrorStruct {	error: SFE_WAV_FMT_SHORT		, r#str: c_str!("Error in WAV/W64/RF64 file. Short 'fmt ' chunk.") },

	ErrorStruct {	error: SFE_WAV_BAD_FACT		    , r#str: c_str!("Error in WAV file. 'fact' chunk out of place.") },
	ErrorStruct {	error: SFE_WAV_BAD_PEAK		    , r#str: c_str!("Error in WAV file. Bad 'PEAK' chunk.") },
	ErrorStruct {	error: SFE_WAV_PEAK_B4_FMT		, r#str: c_str!("Error in WAV file. 'PEAK' chunk found before 'fmt ' chunk.") },

	ErrorStruct {	error: SFE_WAV_BAD_FORMAT		, r#str: c_str!("Error in WAV file. Errors in 'fmt ' chunk.") },
	ErrorStruct {	error: SFE_WAV_BAD_BLOCKALIGN	, r#str: c_str!("Error in WAV file. Block alignment in 'fmt ' chunk is incorrect.") },
	ErrorStruct {	error: SFE_WAV_NO_DATA			, r#str: c_str!("Error in WAV file. No 'data' chunk marker.") },
	ErrorStruct {	error: SFE_WAV_BAD_LIST		    , r#str: c_str!("Error in WAV file. Malformed LIST chunk.") },
	ErrorStruct {	error: SFE_WAV_UNKNOWN_CHUNK	, r#str: c_str!("Error in WAV file. File contains an unknown chunk marker.") },
	ErrorStruct {	error: SFE_WAV_WVPK_DATA		, r#str: c_str!("Error in WAV file. Data is in WAVPACK format.") },

	ErrorStruct {	error: SFE_WAV_ADPCM_NOT4BIT	, r#str: c_str!("Error in ADPCM WAV file. Invalid bit width.") },
	ErrorStruct {	error: SFE_WAV_ADPCM_CHANNELS	, r#str: c_str!("Error in ADPCM WAV file. Invalid number of channels.") },
	ErrorStruct {	error: SFE_WAV_ADPCM_SAMPLES	, r#str: c_str!("Error in ADPCM WAV file. Invalid number of samples per block.") },
	ErrorStruct {	error: SFE_WAV_GSM610_FORMAT	, r#str: c_str!("Error in GSM610 WAV file. Invalid format chunk.") },
	ErrorStruct {	error: SFE_WAV_NMS_FORMAT		, r#str: c_str!("Error in NMS ADPCM WAV file. Invalid format chunk.") },

	ErrorStruct {	error: SFE_AIFF_NO_FORM		    , r#str: c_str!("Error in AIFF file, bad 'FORM' marker.") },
	ErrorStruct {	error: SFE_AIFF_AIFF_NO_FORM	, r#str: c_str!("Error in AIFF file, 'AIFF' marker without 'FORM'.") },
	ErrorStruct {	error: SFE_AIFF_COMM_NO_FORM	, r#str: c_str!("Error in AIFF file, 'COMM' marker without 'FORM'.") },
	ErrorStruct {	error: SFE_AIFF_SSND_NO_COMM	, r#str: c_str!("Error in AIFF file, 'SSND' marker without 'COMM'.") },
	ErrorStruct {	error: SFE_AIFF_UNKNOWN_CHUNK	, r#str: c_str!("Error in AIFF file, unknown chunk.") },
	ErrorStruct {	error: SFE_AIFF_COMM_CHUNK_SIZE , r#str: c_str!("Error in AIFF file, bad 'COMM' chunk size.") },
	ErrorStruct {	error: SFE_AIFF_BAD_COMM_CHUNK  , r#str: c_str!("Error in AIFF file, bad 'COMM' chunk.") },
	ErrorStruct {	error: SFE_AIFF_PEAK_B4_COMM	, r#str: c_str!("Error in AIFF file. 'PEAK' chunk found before 'COMM' chunk.") },
	ErrorStruct {	error: SFE_AIFF_BAD_PEAK		, r#str: c_str!("Error in AIFF file. Bad 'PEAK' chunk.") },
	ErrorStruct {	error: SFE_AIFF_NO_SSND		    , r#str: c_str!("Error in AIFF file, bad 'SSND' chunk.") },
	ErrorStruct {	error: SFE_AIFF_NO_DATA		    , r#str: c_str!("Error in AIFF file, no sound data.") },
	ErrorStruct {	error: SFE_AIFF_RW_SSND_NOT_LAST, r#str: c_str!("Error in AIFF file, RDWR only possible if SSND chunk at end of file.") },

	ErrorStruct {	error: SFE_AU_UNKNOWN_FORMAT	, r#str: c_str!("Error in AU file, unknown format.") },
	ErrorStruct {	error: SFE_AU_NO_DOTSND		    , r#str: c_str!("Error in AU file, missing '.snd' or 'dns.' marker.") },
	ErrorStruct {	error: SFE_AU_EMBED_BAD_LEN	    , r#str: c_str!("Embedded AU file with unknown length.") },

	ErrorStruct {	error: SFE_RAW_READ_BAD_SPEC	, r#str: c_str!("Error while opening RAW file for read. Must specify format and channels.\nPossibly trying to open unsupported format.") },
	ErrorStruct {	error: SFE_RAW_BAD_BITWIDTH	    , r#str: c_str!("Error. RAW file bitwidth must be a multiple of 8.") },
	ErrorStruct {	error: SFE_RAW_BAD_FORMAT		, r#str: c_str!("Error. Bad format field in SF_INFO struct when opening a RAW file for read.") },

	ErrorStruct {	error: SFE_PAF_NO_MARKER		, r#str: c_str!("Error in PAF file, no marker.") },
	ErrorStruct {	error: SFE_PAF_VERSION			, r#str: c_str!("Error in PAF file, bad version.") },
	ErrorStruct {	error: SFE_PAF_UNKNOWN_FORMAT	, r#str: c_str!("Error in PAF file, unknown format.") },
	ErrorStruct {	error: SFE_PAF_SHORT_HEADER	    , r#str: c_str!("Error in PAF file. File shorter than minimal header.") },
	ErrorStruct {	error: SFE_PAF_BAD_CHANNELS	    , r#str: c_str!("Error in PAF file. Bad channel count.") },

	ErrorStruct {	error: SFE_SVX_NO_FORM			, r#str: c_str!("Error in 8SVX / 16SV file, no 'FORM' marker.") },
	ErrorStruct {	error: SFE_SVX_NO_BODY			, r#str: c_str!("Error in 8SVX / 16SV file, no 'BODY' marker.") },
	ErrorStruct {	error: SFE_SVX_NO_DATA			, r#str: c_str!("Error in 8SVX / 16SV file, no sound data.") },
	ErrorStruct {	error: SFE_SVX_BAD_COMP		    , r#str: c_str!("Error in 8SVX / 16SV file, unsupported compression format.") },
	ErrorStruct {	error: SFE_SVX_BAD_NAME_LENGTH	, r#str: c_str!("Error in 8SVX / 16SV file, NAME chunk too long.") },

	ErrorStruct {	error: SFE_NIST_BAD_HEADER		, r#str: c_str!("Error in NIST file, bad header.") },
	ErrorStruct {	error: SFE_NIST_CRLF_CONVERISON , r#str: c_str!("Error : NIST file damaged by Windows CR -> CRLF conversion process.")	},
	ErrorStruct {	error: SFE_NIST_BAD_ENCODING	, r#str: c_str!("Error in NIST file, unsupported compression format.") },

	ErrorStruct {	error: SFE_VOC_NO_CREATIVE		, r#str: c_str!("Error in VOC file, no 'Creative Voice File' marker.") },
	ErrorStruct {	error: SFE_VOC_BAD_FORMAT		, r#str: c_str!("Error in VOC file, bad format.") },
	ErrorStruct {	error: SFE_VOC_BAD_VERSION		, r#str: c_str!("Error in VOC file, bad version number.") },
	ErrorStruct {	error: SFE_VOC_BAD_MARKER		, r#str: c_str!("Error in VOC file, bad marker in file.") },
	ErrorStruct {	error: SFE_VOC_BAD_SECTIONS	    , r#str: c_str!("Error in VOC file, incompatible VOC sections.") },
	ErrorStruct {	error: SFE_VOC_MULTI_SAMPLERATE , r#str: c_str!("Error in VOC file, more than one sample rate defined.") },
	ErrorStruct {	error: SFE_VOC_MULTI_SECTION	, r#str: c_str!("Unimplemented VOC file feature, file contains multiple sound sections.") },
	ErrorStruct {	error: SFE_VOC_MULTI_PARAM		, r#str: c_str!("Error in VOC file, file contains multiple bit or channel widths.") },
	ErrorStruct {	error: SFE_VOC_SECTION_COUNT	, r#str: c_str!("Error in VOC file, too many sections.") },
	ErrorStruct {	error: SFE_VOC_NO_PIPE			, r#str: c_str!("Error : not able to operate on VOC files over a pipe.") },

	ErrorStruct {	error: SFE_IRCAM_NO_MARKER		, r#str: c_str!("Error in IRCAM file, bad IRCAM marker.") },
	ErrorStruct {	error: SFE_IRCAM_BAD_CHANNELS	, r#str: c_str!("Error in IRCAM file, bad channel count.") },
	ErrorStruct {	error: SFE_IRCAM_UNKNOWN_FORMAT , r#str: c_str!("Error in IRCAM file, unknown encoding format.") },

	ErrorStruct {	error: SFE_W64_64_BIT			, r#str: c_str!("Error in W64 file, file contains 64 bit offset.") },
	ErrorStruct {	error: SFE_W64_NO_RIFF			, r#str: c_str!("Error in W64 file. No 'riff' chunk marker.") },
	ErrorStruct {	error: SFE_W64_NO_WAVE			, r#str: c_str!("Error in W64 file. No 'wave' chunk marker.") },
	ErrorStruct {	error: SFE_W64_NO_DATA			, r#str: c_str!("Error in W64 file. No 'data' chunk marker.") },
	ErrorStruct {	error: SFE_W64_ADPCM_NOT4BIT	, r#str: c_str!("Error in ADPCM W64 file. Invalid bit width.") },
	ErrorStruct {	error: SFE_W64_ADPCM_CHANNELS	, r#str: c_str!("Error in ADPCM W64 file. Invalid number of channels.") },
	ErrorStruct {	error: SFE_W64_GSM610_FORMAT	, r#str: c_str!("Error in GSM610 W64 file. Invalid format chunk.") },

	ErrorStruct {	error: SFE_MAT4_BAD_NAME		, r#str: c_str!("Error in MAT4 file. No variable name.") },
	ErrorStruct {	error: SFE_MAT4_NO_SAMPLERATE	, r#str: c_str!("Error in MAT4 file. No sample rate.") },

	ErrorStruct {	error: SFE_MAT5_BAD_ENDIAN		, r#str: c_str!("Error in MAT5 file. Not able to determine endian-ness.") },
	ErrorStruct {	error: SFE_MAT5_NO_BLOCK		, r#str: c_str!("Error in MAT5 file. Bad block structure.") },
	ErrorStruct {	error: SFE_MAT5_SAMPLE_RATE	    , r#str: c_str!("Error in MAT5 file. Not able to determine sample rate.") },

	ErrorStruct {	error: SFE_PVF_NO_PVF1			, r#str: c_str!("Error in PVF file. No PVF1 marker.") },
	ErrorStruct {	error: SFE_PVF_BAD_HEADER		, r#str: c_str!("Error in PVF file. Bad header.") },
	ErrorStruct {	error: SFE_PVF_BAD_BITWIDTH	    , r#str: c_str!("Error in PVF file. Bad bit width.") },

	ErrorStruct {	error: SFE_XI_BAD_HEADER		, r#str: c_str!("Error in XI file. Bad header.") },
	ErrorStruct {	error: SFE_XI_EXCESS_SAMPLES	, r#str: c_str!("Error in XI file. Excess samples in file.") },
	ErrorStruct {	error: SFE_XI_NO_PIPE			, r#str: c_str!("Error : not able to operate on XI files over a pipe.") },

	ErrorStruct {	error: SFE_HTK_NO_PIPE			, r#str: c_str!("Error : not able to operate on HTK files over a pipe.") },

	ErrorStruct {	error: SFE_SDS_NOT_SDS			, r#str: c_str!("Error : not an SDS file.") },
	ErrorStruct {	error: SFE_SDS_BAD_BIT_WIDTH	, r#str: c_str!("Error : bad bit width for SDS file.") },

	ErrorStruct {	error: SFE_SD2_FD_DISALLOWED	, r#str: c_str!("Error : cannot open SD2 file without a file name.") },
	ErrorStruct {	error: SFE_SD2_BAD_DATA_OFFSET	, r#str: c_str!("Error : bad data offset.") },
	ErrorStruct {	error: SFE_SD2_BAD_MAP_OFFSET	, r#str: c_str!("Error : bad map offset.") },
	ErrorStruct {	error: SFE_SD2_BAD_DATA_LENGTH	, r#str: c_str!("Error : bad data length.") },
	ErrorStruct {	error: SFE_SD2_BAD_MAP_LENGTH	, r#str: c_str!("Error : bad map length.") },
	ErrorStruct {	error: SFE_SD2_BAD_RSRC		    , r#str: c_str!("Error : bad resource fork.") },
	ErrorStruct {	error: SFE_SD2_BAD_SAMPLE_SIZE	, r#str: c_str!("Error : bad sample size.") },

	ErrorStruct {	error: SFE_FLAC_BAD_HEADER		, r#str: c_str!("Error : bad flac header.") },
	ErrorStruct {	error: SFE_FLAC_NEW_DECODER	    , r#str: c_str!("Error : problem while creating flac decoder.") },
	ErrorStruct {	error: SFE_FLAC_INIT_DECODER	, r#str: c_str!("Error : problem with initialization of the flac decoder.") },
	ErrorStruct {	error: SFE_FLAC_LOST_SYNC		, r#str: c_str!("Error : flac decoder lost sync.") },
	ErrorStruct {	error: SFE_FLAC_BAD_SAMPLE_RATE , r#str: c_str!("Error : flac does not support this sample rate.") },
	ErrorStruct {	error: SFE_FLAC_CHANNEL_COUNT_CHANGED, r#str: c_str!("Error : flac channel changed mid stream.") },
	ErrorStruct {	error: SFE_FLAC_UNKOWN_ERROR	, r#str: c_str!("Error : unknown error in flac decoder.") },

	ErrorStruct {	error: SFE_WVE_NOT_WVE			, r#str: c_str!("Error : not a WVE file.") },
	ErrorStruct {	error: SFE_WVE_NO_PIPE			, r#str: c_str!("Error : not able to operate on WVE files over a pipe.") },

	ErrorStruct {	error: SFE_DWVW_BAD_BITWIDTH	, r#str: c_str!("Error : Bad bit width for DWVW encoding. Must be 12, 16 or 24.") },
	ErrorStruct {	error: SFE_G72X_NOT_MONO		, r#str: c_str!("Error : G72x encoding does not support more than 1 channel.") },
	ErrorStruct {	error: SFE_NMS_ADPCM_NOT_MONO	, r#str: c_str!("Error : NMS ADPCM encoding does not support more than 1 channel.") },

	ErrorStruct {	error: SFE_VORBIS_ENCODER_BUG	, r#str: c_str!("Error : Sample rate chosen is known to trigger a Vorbis encoder bug on this CPU.") },

	ErrorStruct {	error: SFE_RF64_NOT_RF64		, r#str: c_str!("Error : Not an RF64 file.") },
	ErrorStruct {	error: SFE_RF64_PEAK_B4_FMT	    , r#str: c_str!("Error in RF64 file. 'PEAK' chunk found before 'fmt ' chunk.") },
	ErrorStruct {	error: SFE_RF64_NO_DATA		    , r#str: c_str!("Error in RF64 file. No 'data' chunk marker.") },

	ErrorStruct {	error: SFE_ALAC_FAIL_TMPFILE	, r#str: c_str!("Error : Failed to open tmp file for ALAC encoding.") },

	ErrorStruct {	error: SFE_BAD_CHUNK_PTR		, r#str: c_str!("Error : Bad SF_CHUNK_INFO pointer.") },
	ErrorStruct {	error: SFE_UNKNOWN_CHUNK		, r#str: c_str!("Error : Unknown chunk marker.") },
	ErrorStruct {	error: SFE_BAD_CHUNK_FORMAT	    , r#str: c_str!("Error : Reading/writing chunks from this file format is not supported.") },
	ErrorStruct {	error: SFE_BAD_CHUNK_MARKER	    , r#str: c_str!("Error : Bad chunk marker.") },
	ErrorStruct {	error: SFE_BAD_CHUNK_DATA_PTR	, r#str: c_str!("Error : Bad data pointer in SF_CHUNK_INFO struct.") },
	ErrorStruct {	error: SFE_FILENAME_TOO_LONG	, r#str: c_str!("Error : Supplied filename too long.") },
	ErrorStruct {	error: SFE_NEGATIVE_RW_LEN		, r#str: c_str!("Error : Length parameter passed to read/write is negative.") },

	ErrorStruct {	error: SFE_OPUS_BAD_SAMPLERATE	, r#str: c_str!("Error : Opus only supports sample rates of 8000, 12000, 16000, 24000 and 48000.") },

	ErrorStruct {	error: SFE_MAX_ERROR			, r#str: c_str!("Maximum error number.") },
	ErrorStruct {	error: SFE_MAX_ERROR + 1		, r#str: c_str!("") }
];

macro_rules! VALIDATE_SNDFILE_AND_ASSIGN_PSF {
    ($a:expr, $b:expr, $c:expr) => {
        if $a.is_null() {
            psf_set_sf_errno(SFE_BAD_SNDFILE_PTR);
            return 0;
        }
        $b = $a as *mut SF_PRIVATE;
        if (*$b).virtual_io == SF_FALSE && psf_file_valid($b) == 0 {
            (*$b).error = SFE_BAD_FILE_PTR;
            return 0;
        }
        if (*$b).Magick != SNDFILE_MAGICK {
            (*$b).error = SFE_BAD_SNDFILE_PTR;
            return 0;
        }
        if $c != SF_FALSE {
            (*$b).error = 0;
        }
    };
}

#[no_mangle]
pub unsafe fn sf_error_number(errnum: c_int) -> *const c_char {
    let bad_errnum = c_str!("No error defined for this error number. This is a bug in libsndfile.");

    if errnum == SFE_MAX_ERROR {
        return SndfileErrors[0].r#str.as_ptr();
    }

    if errnum < 0 || errnum > SFE_MAX_ERROR {
        /* This really shouldn't happen in release versions. */
        // printf ("Not a valid error number (%d).\n", errnum) ;
        return bad_errnum.as_ptr();
    }

    for errstr in SndfileErrors.iter() {
        if errnum == errstr.error {
            return errstr.r#str.as_ptr();
        }
    }

    return bad_errnum.as_ptr();
}

#[no_mangle]
pub unsafe fn sf_strerror(sndfile: *mut SNDFILE) -> *const c_char {
    // SF_PRIVATE 	*psf = NULL ;
    // int errnum ;

    let mut errnum = SFE_NO_ERROR;
    if sndfile.is_null() {
        errnum = psf_get_sf_errno();
        if errnum == SFE_SYSTEM && !psf_get_sf_syserr().is_null() {
            return psf_get_sf_syserr();
        }
    } else {
        let psf = &mut *(sndfile as *mut SF_PRIVATE);

        if psf.Magick != SNDFILE_MAGICK {
            return c_str!("sf_strerror : Bad magic number.").as_ptr();
        }

        errnum = psf.error;

        if errnum == SFE_SYSTEM && psf.syserr[0] != 0 {
            return psf.syserr.as_ptr() as *const c_char;
        }
    }

    return sf_error_number(errnum);
}

#[no_mangle]
pub unsafe fn sf_error(sndfile: *mut SNDFILE) -> c_int {
    if sndfile.is_null() {
        return psf_get_sf_errno();
    }

    let psf: *mut SF_PRIVATE;
    VALIDATE_SNDFILE_AND_ASSIGN_PSF!(sndfile, psf, 0);
    let psf = &mut *psf;

    if psf.error != SFE_NO_ERROR {
        return psf.error;
    }

    SFE_NO_ERROR
}

#[no_mangle]
pub unsafe fn sf_perror(sndfile: *mut SNDFILE) -> c_int {
    let errnum = if sndfile.is_null() {
        psf_get_sf_errno()
    } else {
        let psf: *mut SF_PRIVATE;
        VALIDATE_SNDFILE_AND_ASSIGN_PSF!(sndfile, psf, 0);
        (*psf).error
    };

    let errstr = CStr::from_ptr(sf_error_number(errnum));
    eprintln!("{}", errstr.to_string_lossy());

    SFE_NO_ERROR
}

#[no_mangle]
pub unsafe fn sf_error_str(sndfile: *mut SNDFILE, r#str: *mut c_char, maxlen: size_t) -> c_int {
    if str.is_null() {
        return SFE_INTERNAL;
    }

    let errnum = if sndfile.is_null() {
        psf_get_sf_errno()
    } else {
        let psf: *mut SF_PRIVATE;
        VALIDATE_SNDFILE_AND_ASSIGN_PSF!(sndfile, psf, 0);
        (*psf).error
    };

    snprintf(
        r#str,
        maxlen,
        c_str!("%s").as_ptr(),
        sf_error_number(errnum),
    );

    return SFE_NO_ERROR;
}

#[no_mangle]
pub unsafe fn sf_format_check(info: *const SF_INFO) -> c_int {
    assert!(!info.is_null());
    let info = &*info;

    let subformat = SF_CODEC!(info.format);
    let endian = SF_ENDIAN!(info.format);

    // This is the place where each file format can check if the suppiled
    // SF_INFO struct is valid.
    // Return 0 on failure, 1 ons success.

    if info.channels < 1 || info.channels > SF_MAX_CHANNELS {
        return 0;
    }

    if info.samplerate < 0 {
        return 0;
    }

    match SF_CONTAINER!(info.format) {
        SF_FORMAT_WAV => {
            /* WAV now allows both endian, RIFF or RIFX (little or big respectively) */
            if subformat == SF_FORMAT_PCM_U8 || subformat == SF_FORMAT_PCM_16 {
                return 1;
            }
            if subformat == SF_FORMAT_PCM_24 || subformat == SF_FORMAT_PCM_32 {
                return 1;
            }
            if (subformat == SF_FORMAT_IMA_ADPCM || subformat == SF_FORMAT_MS_ADPCM)
                && info.channels <= 2
            {
                return 1;
            }
            if subformat == SF_FORMAT_GSM610 && info.channels == 1 {
                return 1;
            }
            if subformat == SF_FORMAT_G721_32 && info.channels == 1 {
                return 1;
            }
            if subformat == SF_FORMAT_ULAW || subformat == SF_FORMAT_ALAW {
                return 1;
            }
            if subformat == SF_FORMAT_FLOAT || subformat == SF_FORMAT_DOUBLE {
                return 1;
            }
            if (subformat == SF_FORMAT_NMS_ADPCM_16
                || subformat == SF_FORMAT_NMS_ADPCM_24
                || subformat == SF_FORMAT_NMS_ADPCM_32)
                && info.channels == 1
            {
                return 1;
            }
        }

        SF_FORMAT_WAVEX => {
            if endian == SF_ENDIAN_BIG || endian == SF_ENDIAN_CPU {
                return 0;
            }
            if subformat == SF_FORMAT_PCM_U8 || subformat == SF_FORMAT_PCM_16 {
                return 1;
            }
            if subformat == SF_FORMAT_PCM_24 || subformat == SF_FORMAT_PCM_32 {
                return 1;
            }
            if subformat == SF_FORMAT_ULAW || subformat == SF_FORMAT_ALAW {
                return 1;
            }
            if subformat == SF_FORMAT_FLOAT || subformat == SF_FORMAT_DOUBLE {
                return 1;
            }
        }

        SF_FORMAT_AIFF => {
            /* AIFF does allow both endian-nesses for PCM data.*/
            if subformat == SF_FORMAT_PCM_16
                || subformat == SF_FORMAT_PCM_24
                || subformat == SF_FORMAT_PCM_32
            {
                return 1;
            }
            /* For other encodings reject any endian-ness setting. */
            if endian != 0 {
                return 0;
            }
            if subformat == SF_FORMAT_PCM_U8 || subformat == SF_FORMAT_PCM_S8 {
                return 1;
            }
            if subformat == SF_FORMAT_FLOAT || subformat == SF_FORMAT_DOUBLE {
                return 1;
            }
            if subformat == SF_FORMAT_ULAW || subformat == SF_FORMAT_ALAW {
                return 1;
            }
            if (subformat == SF_FORMAT_DWVW_12
                || subformat == SF_FORMAT_DWVW_16
                || subformat == SF_FORMAT_DWVW_24)
                && info.channels == 1
            {
                return 1;
            }
            if subformat == SF_FORMAT_GSM610 && info.channels == 1 {
                return 1;
            }
            if subformat == SF_FORMAT_IMA_ADPCM && (info.channels == 1 || info.channels == 2) {
                return 1;
            }
        }

        SF_FORMAT_AU => {
            if subformat == SF_FORMAT_PCM_S8 || subformat == SF_FORMAT_PCM_16 {
                return 1;
            }
            if subformat == SF_FORMAT_PCM_24 || subformat == SF_FORMAT_PCM_32 {
                return 1;
            }
            if subformat == SF_FORMAT_ULAW || subformat == SF_FORMAT_ALAW {
                return 1;
            }
            if subformat == SF_FORMAT_FLOAT || subformat == SF_FORMAT_DOUBLE {
                return 1;
            }
            if subformat == SF_FORMAT_G721_32 && info.channels == 1 {
                return 1;
            }
            if subformat == SF_FORMAT_G723_24 && info.channels == 1 {
                return 1;
            }
            if subformat == SF_FORMAT_G723_40 && info.channels == 1 {
                return 1;
            }
        }

        SF_FORMAT_CAF => {
            if subformat == SF_FORMAT_PCM_S8 || subformat == SF_FORMAT_PCM_16 {
                return 1;
            }
            if subformat == SF_FORMAT_PCM_24 || subformat == SF_FORMAT_PCM_32 {
                return 1;
            }
            if subformat == SF_FORMAT_ULAW || subformat == SF_FORMAT_ALAW {
                return 1;
            }
            if subformat == SF_FORMAT_ALAC_16 || subformat == SF_FORMAT_ALAC_20 {
                return 1;
            }
            if subformat == SF_FORMAT_ALAC_24 || subformat == SF_FORMAT_ALAC_32 {
                return 1;
            }
            if subformat == SF_FORMAT_FLOAT || subformat == SF_FORMAT_DOUBLE {
                return 1;
            }
        }

        SF_FORMAT_RAW => {
            if subformat == SF_FORMAT_PCM_U8
                || subformat == SF_FORMAT_PCM_S8
                || subformat == SF_FORMAT_PCM_16
            {
                return 1;
            }
            if subformat == SF_FORMAT_PCM_24 || subformat == SF_FORMAT_PCM_32 {
                return 1;
            }
            if subformat == SF_FORMAT_FLOAT || subformat == SF_FORMAT_DOUBLE {
                return 1;
            }
            if subformat == SF_FORMAT_ALAW || subformat == SF_FORMAT_ULAW {
                return 1;
            }
            if (subformat == SF_FORMAT_DWVW_12
                || subformat == SF_FORMAT_DWVW_16
                || subformat == SF_FORMAT_DWVW_24)
                && info.channels == 1
            {
                return 1;
            }
            if subformat == SF_FORMAT_GSM610 && info.channels == 1 {
                return 1;
            }
            if subformat == SF_FORMAT_VOX_ADPCM && info.channels == 1 {
                return 1;
            }
            if (subformat == SF_FORMAT_NMS_ADPCM_16
                || subformat == SF_FORMAT_NMS_ADPCM_24
                || subformat == SF_FORMAT_NMS_ADPCM_32)
                && info.channels == 1
            {
                return 1;
            }
        }

        SF_FORMAT_PAF => {
            if subformat == SF_FORMAT_PCM_S8
                || subformat == SF_FORMAT_PCM_16
                || subformat == SF_FORMAT_PCM_24
            {
                return 1;
            }
        }

        SF_FORMAT_SVX => {
            // SVX only supports writing mono SVX files.
            if info.channels > 1 {
                return 0;
            }
            // Always big endian.
            if endian == SF_ENDIAN_LITTLE || endian == SF_ENDIAN_CPU {
                return 0;
            }

            if subformat == SF_FORMAT_PCM_S8 || subformat == SF_FORMAT_PCM_16 {
                return 1;
            }
        }

        SF_FORMAT_NIST => {
            if subformat == SF_FORMAT_PCM_S8 || subformat == SF_FORMAT_PCM_16 {
                return 1;
            }
            if subformat == SF_FORMAT_PCM_24 || subformat == SF_FORMAT_PCM_32 {
                return 1;
            }
            if subformat == SF_FORMAT_ULAW || subformat == SF_FORMAT_ALAW {
                return 1;
            }
        }

        SF_FORMAT_IRCAM => {
            if info.channels > 256 {
                return 0;
            }
            if subformat == SF_FORMAT_PCM_16 || subformat == SF_FORMAT_PCM_32 {
                return 1;
            }
            if subformat == SF_FORMAT_ULAW
                || subformat == SF_FORMAT_ALAW
                || subformat == SF_FORMAT_FLOAT
            {
                return 1;
            }
        }

        SF_FORMAT_VOC => {
            if info.channels > 2 {
                return 0;
            }
            // VOC is strictly little endian.
            if endian == SF_ENDIAN_BIG || endian == SF_ENDIAN_CPU {
                return 0;
            }
            if subformat == SF_FORMAT_PCM_U8 || subformat == SF_FORMAT_PCM_16 {
                return 1;
            }
            if subformat == SF_FORMAT_ULAW || subformat == SF_FORMAT_ALAW {
                return 1;
            }
        }

        SF_FORMAT_W64 => {
            // W64 is strictly little endian.
            if endian == SF_ENDIAN_BIG || endian == SF_ENDIAN_CPU {
                return 0;
            }
            if subformat == SF_FORMAT_PCM_U8 || subformat == SF_FORMAT_PCM_16 {
                return 1;
            }
            if subformat == SF_FORMAT_PCM_24 || subformat == SF_FORMAT_PCM_32 {
                return 1;
            }
            if (subformat == SF_FORMAT_IMA_ADPCM || subformat == SF_FORMAT_MS_ADPCM)
                && info.channels <= 2
            {
                return 1;
            }
            if subformat == SF_FORMAT_GSM610 && info.channels == 1 {
                return 1;
            }
            if subformat == SF_FORMAT_ULAW || subformat == SF_FORMAT_ALAW {
                return 1;
            }
            if subformat == SF_FORMAT_FLOAT || subformat == SF_FORMAT_DOUBLE {
                return 1;
            }
        }

        SF_FORMAT_MAT4 => {
            if subformat == SF_FORMAT_PCM_16 || subformat == SF_FORMAT_PCM_32 {
                return 1;
            }
            if subformat == SF_FORMAT_FLOAT || subformat == SF_FORMAT_DOUBLE {
                return 1;
            }
        }

        SF_FORMAT_MAT5 => {
            if subformat == SF_FORMAT_PCM_U8
                || subformat == SF_FORMAT_PCM_16
                || subformat == SF_FORMAT_PCM_32
            {
                return 1;
            }
            if subformat == SF_FORMAT_FLOAT || subformat == SF_FORMAT_DOUBLE {
                return 1;
            }
        }

        SF_FORMAT_PVF => {
            if subformat == SF_FORMAT_PCM_S8
                || subformat == SF_FORMAT_PCM_16
                || subformat == SF_FORMAT_PCM_32
            {
                return 1;
            }
        }

        SF_FORMAT_XI => {
            if info.channels != 1 {
                return 0;
            }
            if subformat == SF_FORMAT_DPCM_8 || subformat == SF_FORMAT_DPCM_16 {
                return 1;
            }
        }

        SF_FORMAT_HTK => {
            if info.channels != 1 {
                return 0;
            }
            // HTK is strictly big endian.
            if endian == SF_ENDIAN_LITTLE || endian == SF_ENDIAN_CPU {
                return 0;
            }
            if subformat == SF_FORMAT_PCM_16 {
                return 1;
            }
        }

        SF_FORMAT_SDS => {
            if info.channels != 1 {
                return 0;
            }
            /* SDS is strictly big endian. */
            if endian == SF_ENDIAN_LITTLE || endian == SF_ENDIAN_CPU {
                return 0;
            }
            if subformat == SF_FORMAT_PCM_S8
                || subformat == SF_FORMAT_PCM_16
                || subformat == SF_FORMAT_PCM_24
            {
                return 1;
            }
        }

        SF_FORMAT_AVR => {
            if info.channels > 2 {
                return 0;
            }
            // SDS is strictly big endian.
            if endian == SF_ENDIAN_LITTLE || endian == SF_ENDIAN_CPU {
                return 0;
            }
            if subformat == SF_FORMAT_PCM_U8
                || subformat == SF_FORMAT_PCM_S8
                || subformat == SF_FORMAT_PCM_16
            {
                return 1;
            }
        }

        SF_FORMAT_FLAC => {
            // FLAC can't do more than 8 channels.
            if info.channels > 8 {
                return 0;
            }
            if endian != SF_ENDIAN_FILE {
                return 0;
            }
            if subformat == SF_FORMAT_PCM_S8
                || subformat == SF_FORMAT_PCM_16
                || subformat == SF_FORMAT_PCM_24
            {
                return 1;
            }
        }

        SF_FORMAT_SD2 => {
            // SD2 is strictly big endian.
            if endian == SF_ENDIAN_LITTLE || endian == SF_ENDIAN_CPU {
                return 0;
            }
            if subformat == SF_FORMAT_PCM_S8
                || subformat == SF_FORMAT_PCM_16
                || subformat == SF_FORMAT_PCM_24
                || subformat == SF_FORMAT_PCM_32
            {
                return 1;
            }
        }

        SF_FORMAT_WVE => {
            if info.channels > 1 {
                return 0;
            }
            // WVE is strictly big endian.
            if endian == SF_ENDIAN_BIG || endian == SF_ENDIAN_CPU {
                return 0;
            }
            if subformat == SF_FORMAT_ALAW {
                return 1;
            }
        }

        SF_FORMAT_OGG => {
            if endian != SF_ENDIAN_FILE {
                return 0;
            }
            if subformat == SF_FORMAT_VORBIS {
                return 1;
            }
            if subformat == SF_FORMAT_OPUS {
                return 1;
            }
        }

        SF_FORMAT_MPC2K => {
            if info.channels > 2 {
                return 0;
            }
            // MPC2000 is strictly little endian.
            if endian == SF_ENDIAN_BIG || endian == SF_ENDIAN_CPU {
                return 0;
            }
            if subformat == SF_FORMAT_PCM_16 {
                return 1;
            }
        }

        SF_FORMAT_RF64 => {
            if endian == SF_ENDIAN_BIG || endian == SF_ENDIAN_CPU {
                return 0;
            }
            if subformat == SF_FORMAT_PCM_U8 || subformat == SF_FORMAT_PCM_16 {
                return 1;
            }
            if subformat == SF_FORMAT_PCM_24 || subformat == SF_FORMAT_PCM_32 {
                return 1;
            }
            if subformat == SF_FORMAT_ULAW || subformat == SF_FORMAT_ALAW {
                return 1;
            }
            if subformat == SF_FORMAT_FLOAT || subformat == SF_FORMAT_DOUBLE {
                return 1;
            }
        }
        _ => {}
    }

    0
}

extern "C" {
    fn psf_get_sf_errno() -> c_int;
    fn psf_set_sf_errno(errnum: c_int);
    fn psf_get_sf_syserr() -> *const c_char;
    fn psf_binheader_readf(psf: *mut SF_PRIVATE, format: *const c_char, ...) -> c_int;
    fn psf_binheader_writef(psf: *mut SF_PRIVATE, format: *const c_char, ...) -> c_int;

    fn pcm_init(psf: *mut SF_PRIVATE) -> c_int;

    #[cfg(windows)]
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
}
