use crate::*;

pub use file_io::*;

use std::fmt;
use std::mem;
use std::ptr;
use std::slice;

use byte_strings::c_str;

pub(crate) static PACKAGE_NAME: &CStr = c_str!("libsndfile");
pub(crate) static PACKAGE_VERSION: &CStr = c_str!("1.0.29pre2");

#[cfg(not(ENABLE_EXPERIMENTAL_CODE))]
pub(crate) static PACKAGE_VERSION_STRING: &CStr = c_str!("libsndfile", "-", "1.0.29pre2");
#[cfg(ENABLE_EXPERIMENTAL_CODE)]
pub(crate) static PACKAGE_VERSION_STRING: &CStr = c_str!("libsndfile", "-", "1.0.29pre2", "-exp");

pub const SF_BUFFER_LEN: usize = 8192;
pub const SF_FILENAME_LEN: usize = 1024;
pub const SF_SYSERR_LEN: usize = 256;
pub const SF_MAX_STRINGS: usize = 32;
pub const SF_PARSELOG_LEN: usize = 2048;

pub const PSF_SEEK_ERROR: sf_count_t = -1;

pub const SF_MAX_CHANNELS: c_int = 1024;

#[repr(C)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum SF_BOOL {
    FALSE,
    TRUE,
}

impl Default for SF_BOOL {
    fn default() -> Self {
        SF_BOOL::FALSE
    }
}

impl From<c_int> for SF_BOOL {
    fn from(value: c_int) -> Self {
        if value != SF_FALSE { SF_BOOL::TRUE } else { SF_BOOL::FALSE }
    }
}

impl From<bool> for SF_BOOL {
    fn from(value: bool) -> Self {
        if value { SF_BOOL::TRUE } else { SF_BOOL::FALSE }
    }
}

#[repr(C)]
pub enum SF_SEEK_MODE {
    SET =  0,
    CUR =  1,
    END =  2,
}

macro_rules! SF_CONTAINER {
    ($x:expr) => {
        $x & SF_FORMAT_TYPEMASK
    };
}
macro_rules! SF_CODEC {
    ($x:expr) => {
        $x & SF_FORMAT_SUBMASK
    };
}
macro_rules! SF_ENDIAN {
    ($x:expr) => {
        $x & SF_FORMAT_ENDMASK
    };
}

macro_rules! BHW4 {
    ($x:expr) => {
        $x as u32
    };
}

#[repr(C)]
pub enum SF_PEAK_LOCATION {
    SF_PEAK_START = 42,
    SF_PEAK_END = 43,
}

pub(crate) const SF_STR_ALLOW_START: u32 = 0x0100;
pub(crate) const SF_STR_ALLOW_END: u32 = 0x0200;

pub(crate) const SF_STR_LOCATE_START: u32 = 0x0400;
pub(crate) const SF_STR_LOCATE_END: u32 = 0x0800;

pub type sfwchar_t = u16;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct PEAK_POS {
    pub value: f64,           /* signed value of peak */
    pub position: sf_count_t, /* the sample frame for the peak */
}

#[repr(C)]
pub struct PEAK_INFO {
    /* libsndfile internal : write a PEAK chunk at the start or end of the file? */
    pub peak_loc: SF_PEAK_LOCATION,

    /* WAV/AIFF */
    pub version: c_uint,   /* version of the PEAK chunk */
    pub timestamp: c_uint, /* secs since 1/1/1970  */

    /* CAF */
    pub edit_number: c_uint,

    /* the per channel peak info */
    pub peaks: [PEAK_POS; 1],
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct STR_DATA {
    pub r#type: c_int,
    pub flags: c_int,
    pub offset: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct READ_CHUNK {
    pub hash: u64,
    pub id: [c_char; 64],
    pub id_size: c_uint,
    pub mark32: u32,
    pub offset: sf_count_t,
    pub len: u32,
}

#[repr(C)]
pub struct WRITE_CHUNK {
    pub hash: u64,
    pub mark32: u32,
    pub len: u32,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct READ_CHUNKS {
    pub count: u32,
    pub used: u32,
    pub chunks: *mut READ_CHUNK,
}

impl Default for READ_CHUNKS {
    fn default() -> Self {
        READ_CHUNKS {
            count: 0,
            used: 0,
            chunks: ptr::null_mut(),
        }
    }
}

#[repr(C)]
pub struct WRITE_CHUNKS {
    pub count: u32,
    pub used: u32,
    pub chunks: *mut WRITE_CHUNK,
}

impl Default for WRITE_CHUNKS {
    fn default() -> Self {
        WRITE_CHUNKS {
            count: 0,
            used: 0,
            chunks: ptr::null_mut(),
        }
    }
}

#[repr(C)]
pub struct SF_CHUNK_ITERATOR {
    pub current: u32,
    pub hash: i64,
    pub id: [c_char; 64],
    pub id_size: c_uint,
    pub sndfile: *mut SF_PRIVATE,
}

SF_BROADCAST_INFO_VAR!(16 * 1024, SF_BROADCAST_INFO_16K);
SF_CART_INFO_VAR!(16 * 1024, SF_CART_INFO_16K);

#[repr(C)]
pub union PSF_FILE_PATH {
    pub c: [c_char; SF_FILENAME_LEN],
    pub wc: [sfwchar_t; SF_FILENAME_LEN],
}

impl Default for PSF_FILE_PATH {
    fn default() -> Self {
        PSF_FILE_PATH {
            wc: [0; SF_FILENAME_LEN],
        }
    }
}

#[repr(C)]
pub union PSF_FILE_DIR {
    pub c: [c_char; SF_FILENAME_LEN],
    pub wc: [sfwchar_t; SF_FILENAME_LEN],
}

impl Default for PSF_FILE_DIR {
    fn default() -> Self {
        PSF_FILE_DIR {
            wc: [0; SF_FILENAME_LEN],
        }
    }
}

#[repr(C)]
pub union PSF_FILE_NAME {
    pub c: [c_char; SF_FILENAME_LEN / 4],
    pub wc: [sfwchar_t; SF_FILENAME_LEN / 4],
}

impl Default for PSF_FILE_NAME {
    fn default() -> Self {
        PSF_FILE_NAME {
            wc: [0; SF_FILENAME_LEN / 4],
        }
    }
}

#[repr(C)]
pub struct PSF_FILE {
    pub path: PSF_FILE_PATH,
    pub dir: PSF_FILE_DIR,
    pub name: PSF_FILE_NAME,

    #[cfg(windows)]
    pub handle: winapi::um::winnt::HANDLE,
    #[cfg(windows)]
    pub hsaved: winapi::um::winnt::HANDLE,
    #[cfg(windows)]
    pub use_wchar: c_int,

    #[cfg(not(windows))]
    pub filedes: c_int,
    #[cfg(not(windows))]
    pub savedes: c_int,

    pub do_not_close_descriptor: c_int,
    pub mode: SFM_OPEN_MODE,
}

impl fmt::Debug for PSF_FILE {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("PSF_FILE").finish()
    }
}

impl Default for PSF_FILE {
    fn default() -> Self {
        PSF_FILE {
            path: PSF_FILE_PATH::default(),
            dir: PSF_FILE_DIR::default(),
            name: PSF_FILE_NAME::default(),
            #[cfg(windows)]
            handle: ptr::null_mut(),
            #[cfg(windows)]
            hsaved: ptr::null_mut(),
            #[cfg(windows)]
            use_wchar: 0,
            #[cfg(not(windows))]
            filedes: 0,
            #[cfg(not(windows))]
            savedes: 0,
            do_not_close_descriptor: 0,
            mode: SFM_OPEN_MODE::READ,
        }
    }
}

#[repr(C)]
pub union BUF_UNION {
    pub dbuf: [f64; SF_BUFFER_LEN / mem::size_of::<f64>()],
    pub lbuf: [i64; SF_BUFFER_LEN / mem::size_of::<i64>()],
    pub fbuf: [f32; SF_BUFFER_LEN / mem::size_of::<f32>()],
    pub ibuf: [i32; SF_BUFFER_LEN / mem::size_of::<i32>()],
    pub sbuf: [i16; SF_BUFFER_LEN / mem::size_of::<i16>()],
    pub cbuf: [c_char; SF_BUFFER_LEN / mem::size_of::<c_char>()],
    pub scbuf: [c_char; SF_BUFFER_LEN / mem::size_of::<c_char>()],
    pub ucbuf: [c_uchar; SF_BUFFER_LEN / mem::size_of::<c_uchar>()],
}

impl Default for BUF_UNION {
    fn default() -> Self {
        BUF_UNION {
            ucbuf: [0; SF_BUFFER_LEN / mem::size_of::<c_uchar>()],
        }
    }
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct SF_PRIVATE_CANARY {
    pub d: [f64; 2],
    //  pub c: [u8; 16],
}

#[repr(C)]
pub struct SF_PRIVATE_PARSELOG {
    pub buf: [c_char; SF_PARSELOG_LEN],
    pub indx: c_int,
}

impl Default for SF_PRIVATE_PARSELOG {
    fn default() -> Self {
        SF_PRIVATE_PARSELOG {
            buf: [0; SF_PARSELOG_LEN],
            indx: 0,
        }
    }
}

#[repr(C)]
pub struct SF_PRIVATE_HEADER {
    pub ptr: *mut u8,
    pub indx: sf_count_t,
    pub end: sf_count_t,
    pub len: sf_count_t,
}

impl Default for SF_PRIVATE_HEADER {
    fn default() -> Self {
        SF_PRIVATE_HEADER {
            ptr: ptr::null_mut(),
            indx: 0,
            end: 0,
            len: 0,
        }
    }
}

#[repr(C)]
pub struct SF_PRIVATE_STRINGS {
    pub data: [STR_DATA; SF_MAX_STRINGS],
    pub storage: *mut c_char,
    pub storage_len: usize,
    pub storage_used: usize,
    pub flags: u32,
}

impl Default for SF_PRIVATE_STRINGS {
    fn default() -> Self {
        SF_PRIVATE_STRINGS {
            data: [STR_DATA::default(); SF_MAX_STRINGS],
            storage: ptr::null_mut(),
            storage_len: 0,
            storage_used: 0,
            flags: 0,
        }
    }
}

#[repr(C)]
pub struct sf_private_tag {
    /* Canary in a coal mine. */
    pub canary: SF_PRIVATE_CANARY,

    pub file: PSF_FILE,
    pub rsrc: PSF_FILE,

    pub syserr: [u8; SF_SYSERR_LEN],

    /* parselog and indx should only be changed within the logging functions
    	** of common.c
    	*/
    pub parselog: SF_PRIVATE_PARSELOG,

    pub header: SF_PRIVATE_HEADER,

    pub rwf_endian: c_int, /* Header endian-ness flag. */

    /* Storage and housekeeping data for adding/reading strings from
    	** sound files.
    	*/
    pub strings: SF_PRIVATE_STRINGS,

    /* Guard value. If this changes the buffers above have overflowed. */
    pub Magick: c_int,

    pub unique_id: c_uint,

    pub error: c_int,

    pub endian: c_int, /* File endianness : SF_ENDIAN_LITTLE or SF_ENDIAN_BIG. */
    pub data_endswap: c_int, /* Need to endswap data? */

    /*
     * Maximum float value for calculating the multiplier for
     * float/double to short/int conversions.
     */
    pub float_int_mult: c_int,
    pub float_max: f32,

    pub scale_int_float: c_int,

    /* Vairables for handling pipes. */
    pub is_pipe: c_int,         /* True if file is a pipe. */
    pub pipeoffset: sf_count_t, /* Number of bytes read from a pipe. */

    /* True if clipping must be performed on float->int conversions. */
    pub add_clipping: c_int,

    pub sf: SF_INFO,

    pub have_written: c_int, /* Has a single write been done to the file? */
    pub peak_info: *mut PEAK_INFO,

    /* Cue Marker Info */
    pub cues: *mut SF_CUES,

    /* Loop Info */
    pub loop_info: *mut SF_LOOP_INFO,
    pub instrument: *mut SF_INSTRUMENT,

    /* Broadcast (EBU) Info */
    pub broadcast_16k: *mut SF_BROADCAST_INFO_16K,

    /* Cart (AES46) Info */
    pub cart_16k: *mut SF_CART_INFO_16K,

    /* Channel map data (if present) : an array of ints. */
    pub channel_map: *mut c_int,

    pub filelength: sf_count_t, /* Overall length of (embedded) file. */
    pub fileoffset: sf_count_t, /* Offset in number of bytes from beginning of file. */

    pub rsrclength: sf_count_t, /* Length of the resource fork (if it exists). */

    pub dataoffset: sf_count_t, /* Offset in number of bytes from beginning of file. */
    pub datalength: sf_count_t, /* Length in bytes of the audio data. */
    pub dataend: sf_count_t,    /* Offset to file tailer. */

    pub blockwidth: c_int, /* Size in bytes of one set of interleaved samples. */
    pub bytewidth: c_int,  /* Size in bytes of one sample (one channel). */

    pub dither: *mut c_void,
    pub interleave: *mut c_void,

    pub last_op: c_int, /* Last operation; either SFM_READ or SFM_WRITE */
    pub read_current: sf_count_t,
    pub write_current: sf_count_t,

    pub container_data: *mut c_void, /* This is a pointer to dynamically allocated file
                                      * container format specific data.
                                      */

    pub codec_data: *mut c_void, /* This is a pointer to dynamically allocated file
                                  * codec format specific data.
                                  */

    pub write_dither: SF_DITHER_INFO,
    pub read_dither: SF_DITHER_INFO,

    pub norm_double: c_int,
    pub norm_float: c_int,

    pub auto_header: c_int,

    pub ieee_replace: c_int,

    /* A set of file specific function pointers */
    pub read_short: Option<
        unsafe extern "C" fn(
            psf: *mut sf_private_tag,
            ptr: *mut c_short,
            len: sf_count_t,
        ) -> sf_count_t,
    >,
    pub read_int: Option<
        unsafe extern "C" fn(
            psf: *mut sf_private_tag,
            ptr: *mut c_int,
            len: sf_count_t,
        ) -> sf_count_t,
    >,
    pub read_float: Option<
        unsafe extern "C" fn(
            psf: *mut sf_private_tag,
            ptr: *mut c_float,
            len: sf_count_t,
        ) -> sf_count_t,
    >,
    pub read_double: Option<
        unsafe extern "C" fn(
            psf: *mut sf_private_tag,
            ptr: *mut c_double,
            len: sf_count_t,
        ) -> sf_count_t,
    >,

    pub write_short: Option<
        unsafe extern "C" fn(
            psf: *mut sf_private_tag,
            ptr: *const c_short,
            len: sf_count_t,
        ) -> sf_count_t,
    >,
    pub write_int: Option<
        unsafe extern "C" fn(
            psf: *mut sf_private_tag,
            ptr: *const c_int,
            len: sf_count_t,
        ) -> sf_count_t,
    >,
    pub write_float: Option<
        unsafe extern "C" fn(
            psf: *mut sf_private_tag,
            ptr: *const c_float,
            len: sf_count_t,
        ) -> sf_count_t,
    >,
    pub write_double: Option<
        unsafe extern "C" fn(
            psf: *mut sf_private_tag,
            ptr: *const c_double,
            len: sf_count_t,
        ) -> sf_count_t,
    >,

    pub seek: Option<
        unsafe extern "C" fn(
            psf: *mut sf_private_tag,
            mode: c_int,
            samples_from_start: sf_count_t,
        ) -> sf_count_t,
    >,
    pub write_header:
        Option<unsafe extern "C" fn(psf: *mut sf_private_tag, calc_length: c_int) -> c_int>,
    pub command: Option<
        unsafe extern "C" fn(
            psf: &mut sf_private_tag,
            command: c_int,
            data: *mut c_void,
            datasize: c_int,
        ) -> c_int,
    >,
    pub byterate: Option<unsafe extern "C" fn(psf: *mut sf_private_tag) -> c_int>,

    /*
     * Separate close functions for the codec and the container.
     * The codec close function is always called first.
     */
    pub codec_close: Option<unsafe extern "C" fn(psf: *mut sf_private_tag) -> c_int>,
    pub container_close: Option<unsafe extern "C" fn(psf: *mut sf_private_tag) -> c_int>,

    pub format_desc: *mut c_char,

    /* Virtual I/O functions. */
    pub virtual_io: c_int,
    pub vio: SF_VIRTUAL_IO,
    pub vio_user_data: *mut c_void,

    /* Chunk get/set. */
    pub iterator: *mut SF_CHUNK_ITERATOR,

    pub rchunks: READ_CHUNKS,
    pub wchunks: WRITE_CHUNKS,

    pub set_chunk: Option<
        unsafe extern "C" fn(psf: *mut sf_private_tag, chunk_info: *const SF_CHUNK_INFO) -> c_int,
    >,
    pub next_chunk_iterator: Option<
        unsafe extern "C" fn(
            psf: *mut sf_private_tag,
            iterator: *mut SF_CHUNK_ITERATOR,
        ) -> *mut SF_CHUNK_ITERATOR,
    >,

    pub get_chunk_size: Option<
        unsafe extern "C" fn(
            psf: *mut sf_private_tag,
            iterator: *const SF_CHUNK_ITERATOR,
            chunk_info: *mut SF_CHUNK_INFO,
        ) -> c_int,
    >,

    pub get_chunk_data: Option<
        unsafe extern "C" fn(
            psf: *mut sf_private_tag,
            iterator: *const SF_CHUNK_ITERATOR,
            chunk_info: *mut SF_CHUNK_INFO,
        ) -> c_int,
    >,
}

impl Drop for SF_PRIVATE {
    fn drop(&mut self) {
        unsafe {
            match self.codec_close {
                Some(codec_close) => {
                    codec_close(self);
                }
                _ => {}
            };
            /* To prevent it being called in self->container_close(). */
            self.codec_close = None;
            match self.container_close {
                Some(container_close) => {
                    container_close(self);
                }
                _ => {}
            };
            psf_fclose(self);
            psf_close_rsrc(self);
            /* For an ISO C compliant implementation it is ok to free a NULL pointer. */
            free(self.header.ptr as *mut c_void);
            free(self.container_data as *mut c_void);
            free(self.codec_data as *mut c_void);
            free(self.interleave as *mut c_void);
            free(self.dither as *mut c_void);
            free(self.peak_info as *mut c_void);
            free(self.broadcast_16k as *mut c_void);
            free(self.loop_info as *mut c_void);
            free(self.instrument as *mut c_void);
            free(self.cues as *mut c_void);
            free(self.channel_map as *mut c_void);
            free(self.format_desc as *mut c_void);
            free(self.strings.storage as *mut c_void);
            if !self.wchunks.chunks.is_null() {
                let wchunks =
                    slice::from_raw_parts_mut(self.wchunks.chunks, self.wchunks.used as usize);
                for wchunk in wchunks.iter() {
                    free(wchunk.data as *mut c_void);
                }
            }
            free(self.rchunks.chunks as *mut c_void);
            free(self.wchunks.chunks as *mut c_void);
            free(self.iterator as *mut c_void);
            free(self.cart_16k as *mut c_void);
        };
    }
}

impl Default for SF_PRIVATE {
    fn default() -> Self {
        SF_PRIVATE {
            canary: SF_PRIVATE_CANARY::default(),
            file: PSF_FILE::default(),
            rsrc: PSF_FILE::default(),
            syserr: [0; SF_SYSERR_LEN],
            parselog: SF_PRIVATE_PARSELOG::default(),
            header: SF_PRIVATE_HEADER::default(),
            rwf_endian: 0,
            strings: SF_PRIVATE_STRINGS::default(),
            Magick: 0,
            unique_id: 0,
            error: 0,
            endian: 0,
            data_endswap: 0,
            float_int_mult: 0,
            float_max: 0.0,
            scale_int_float: 0,
            is_pipe: 0,
            pipeoffset: 0,
            add_clipping: 0,
            sf: SF_INFO::default(),
            have_written: 0,
            peak_info: ptr::null_mut(),
            cues: ptr::null_mut(),
            loop_info: ptr::null_mut(),
            instrument: ptr::null_mut(),
            broadcast_16k: ptr::null_mut(),
            cart_16k: ptr::null_mut(),
            channel_map: ptr::null_mut(),
            filelength: 0,
            fileoffset: 0,
            rsrclength: 0,
            dataoffset: 0,
            datalength: 0,
            dataend: 0,
            blockwidth: 0,
            bytewidth: 0,
            dither: ptr::null_mut(),
            interleave: ptr::null_mut(),
            last_op: 0,
            read_current: 0,
            write_current: 0,
            container_data: ptr::null_mut(),
            codec_data: ptr::null_mut(),
            write_dither: SF_DITHER_INFO::default(),
            read_dither: SF_DITHER_INFO::default(),
            norm_double: 0,
            norm_float: 0,
            auto_header: 0,
            ieee_replace: 0,
            read_short: None,
            read_int: None,
            read_float: None,
            read_double: None,
            write_short: None,
            write_int: None,
            write_float: None,
            write_double: None,
            seek: None,
            write_header: None,
            command: None,
            byterate: None,
            codec_close: None,
            container_close: None,
            format_desc: ptr::null_mut(),
            virtual_io: 0,
            vio: SF_VIRTUAL_IO::default(),
            vio_user_data: ptr::null_mut(),
            iterator: ptr::null_mut(),
            rchunks: READ_CHUNKS::default(),
            wchunks: WRITE_CHUNKS::default(),
            set_chunk: None,
            next_chunk_iterator: None,
            get_chunk_size: None,
            get_chunk_data: None,
        }
    }
}

pub type SF_PRIVATE = sf_private_tag;

pub const SFE_NO_ERROR: c_int = SF_ERR_NO_ERROR;
pub const SFE_BAD_OPEN_FORMAT: c_int = SF_ERR_UNRECOGNISED_FORMAT;
pub const SFE_SYSTEM: c_int = SF_ERR_SYSTEM;
pub const SFE_MALFORMED_FILE: c_int = SF_ERR_MALFORMED_FILE;
pub const SFE_UNSUPPORTED_ENCODING: c_int = SF_ERR_UNSUPPORTED_ENCODING;

pub const SFE_ZERO_MAJOR_FORMAT: c_int = 5;
pub const SFE_ZERO_MINOR_FORMAT: c_int = 6;
pub const SFE_BAD_FILE: c_int = 7;
pub const SFE_BAD_FILE_READ: c_int = 8;
pub const SFE_OPEN_FAILED: c_int = 9;
pub const SFE_BAD_SNDFILE_PTR: c_int = 10;
pub const SFE_BAD_SF_INFO_PTR: c_int = 11;
pub const SFE_BAD_SF_INCOMPLETE: c_int = 12;
pub const SFE_BAD_FILE_PTR: c_int = 13;
pub const SFE_BAD_INT_PTR: c_int = 14;
pub const SFE_BAD_STAT_SIZE: c_int = 15;
pub const SFE_NO_TEMP_DIR: c_int = 16;
pub const SFE_MALLOC_FAILED: c_int = 17;
pub const SFE_UNIMPLEMENTED: c_int = 18;
pub const SFE_BAD_READ_ALIGN: c_int = 19;
pub const SFE_BAD_WRITE_ALIGN: c_int = 20;
pub const SFE_NOT_READMODE: c_int = 21;
pub const SFE_NOT_WRITEMODE: c_int = 22;
pub const SFE_BAD_MODE_RW: c_int = 23;
pub const SFE_BAD_SF_INFO: c_int = 24;
pub const SFE_BAD_OFFSET: c_int = 25;
pub const SFE_NO_EMBED_SUPPORT: c_int = 26;
pub const SFE_NO_EMBEDDED_RDWR: c_int = 27;
pub const SFE_NO_PIPE_WRITE: c_int = 28;

pub const SFE_INTERNAL: c_int = 29;
pub const SFE_BAD_COMMAND_PARAM: c_int = 30;
pub const SFE_BAD_ENDIAN: c_int = 31;
pub const SFE_CHANNEL_COUNT_ZERO: c_int = 32;
pub const SFE_CHANNEL_COUNT: c_int = 33;
pub const SFE_CHANNEL_COUNT_BAD: c_int = 34;

pub const SFE_BAD_VIRTUAL_IO: c_int = 35;

pub const SFE_INTERLEAVE_MODE: c_int = 36;
pub const SFE_INTERLEAVE_SEEK: c_int = 37;
pub const SFE_INTERLEAVE_READ: c_int = 38;

pub const SFE_BAD_SEEK: c_int = 39;
pub const SFE_NOT_SEEKABLE: c_int = 40;
pub const SFE_AMBIGUOUS_SEEK: c_int = 41;
pub const SFE_WRONG_SEEK: c_int = 42;
pub const SFE_SEEK_FAILED: c_int = 43;

pub const SFE_BAD_OPEN_MODE: c_int = 44;
pub const SFE_OPEN_PIPE_RDWR: c_int = 45;
pub const SFE_RDWR_POSITION: c_int = 46;
pub const SFE_RDWR_BAD_HEADER: c_int = 47;
pub const SFE_CMD_HAS_DATA: c_int = 48;
pub const SFE_BAD_BROADCAST_INFO_SIZE: c_int = 49;
pub const SFE_BAD_BROADCAST_INFO_TOO_BIG: c_int = 50;
pub const SFE_BAD_CART_INFO_SIZE: c_int = 51;
pub const SFE_BAD_CART_INFO_TOO_BIG: c_int = 52;

pub const SFE_STR_NO_SUPPORT: c_int = 53;
pub const SFE_STR_NOT_WRITE: c_int = 54;
pub const SFE_STR_MAX_DATA: c_int = 55;
pub const SFE_STR_MAX_COUNT: c_int = 56;
pub const SFE_STR_BAD_TYPE: c_int = 57;
pub const SFE_STR_NO_ADD_END: c_int = 58;
pub const SFE_STR_BAD_STRING: c_int = 59;
pub const SFE_STR_WEIRD: c_int = 60;

pub const SFE_WAV_NO_RIFF: c_int = 61;
pub const SFE_WAV_NO_WAVE: c_int = 62;
pub const SFE_WAV_NO_FMT: c_int = 63;
pub const SFE_WAV_BAD_FMT: c_int = 64;
pub const SFE_WAV_FMT_SHORT: c_int = 65;
pub const SFE_WAV_BAD_FACT: c_int = 66;
pub const SFE_WAV_BAD_PEAK: c_int = 67;
pub const SFE_WAV_PEAK_B4_FMT: c_int = 68;
pub const SFE_WAV_BAD_FORMAT: c_int = 69;
pub const SFE_WAV_BAD_BLOCKALIGN: c_int = 70;
pub const SFE_WAV_NO_DATA: c_int = 71;
pub const SFE_WAV_BAD_LIST: c_int = 72;
pub const SFE_WAV_ADPCM_NOT4BIT: c_int = 73;
pub const SFE_WAV_ADPCM_CHANNELS: c_int = 74;
pub const SFE_WAV_ADPCM_SAMPLES: c_int = 75;
pub const SFE_WAV_GSM610_FORMAT: c_int = 76;
pub const SFE_WAV_UNKNOWN_CHUNK: c_int = 77;
pub const SFE_WAV_WVPK_DATA: c_int = 78;
pub const SFE_WAV_NMS_FORMAT: c_int = 79;

pub const SFE_AIFF_NO_FORM: c_int = 80;
pub const SFE_AIFF_AIFF_NO_FORM: c_int = 81;
pub const SFE_AIFF_COMM_NO_FORM: c_int = 82;
pub const SFE_AIFF_SSND_NO_COMM: c_int = 83;
pub const SFE_AIFF_UNKNOWN_CHUNK: c_int = 84;
pub const SFE_AIFF_COMM_CHUNK_SIZE: c_int = 85;
pub const SFE_AIFF_BAD_COMM_CHUNK: c_int = 86;
pub const SFE_AIFF_PEAK_B4_COMM: c_int = 87;
pub const SFE_AIFF_BAD_PEAK: c_int = 88;
pub const SFE_AIFF_NO_SSND: c_int = 89;
pub const SFE_AIFF_NO_DATA: c_int = 90;
pub const SFE_AIFF_RW_SSND_NOT_LAST: c_int = 91;

pub const SFE_AU_UNKNOWN_FORMAT: c_int = 92;
pub const SFE_AU_NO_DOTSND: c_int = 93;
pub const SFE_AU_EMBED_BAD_LEN: c_int = 94;

pub const SFE_RAW_READ_BAD_SPEC: c_int = 95;
pub const SFE_RAW_BAD_BITWIDTH: c_int = 96;
pub const SFE_RAW_BAD_FORMAT: c_int = 97;

pub const SFE_PAF_NO_MARKER: c_int = 98;
pub const SFE_PAF_VERSION: c_int = 99;
pub const SFE_PAF_UNKNOWN_FORMAT: c_int = 100;
pub const SFE_PAF_SHORT_HEADER: c_int = 101;
pub const SFE_PAF_BAD_CHANNELS: c_int = 102;

pub const SFE_SVX_NO_FORM: c_int = 103;
pub const SFE_SVX_NO_BODY: c_int = 104;
pub const SFE_SVX_NO_DATA: c_int = 105;
pub const SFE_SVX_BAD_COMP: c_int = 106;
pub const SFE_SVX_BAD_NAME_LENGTH: c_int = 107;

pub const SFE_NIST_BAD_HEADER: c_int = 108;
pub const SFE_NIST_CRLF_CONVERISON: c_int = 109;
pub const SFE_NIST_BAD_ENCODING: c_int = 110;

pub const SFE_VOC_NO_CREATIVE: c_int = 111;
pub const SFE_VOC_BAD_FORMAT: c_int = 112;
pub const SFE_VOC_BAD_VERSION: c_int = 113;
pub const SFE_VOC_BAD_MARKER: c_int = 114;
pub const SFE_VOC_BAD_SECTIONS: c_int = 115;
pub const SFE_VOC_MULTI_SAMPLERATE: c_int = 116;
pub const SFE_VOC_MULTI_SECTION: c_int = 117;
pub const SFE_VOC_MULTI_PARAM: c_int = 118;
pub const SFE_VOC_SECTION_COUNT: c_int = 119;
pub const SFE_VOC_NO_PIPE: c_int = 120;

pub const SFE_IRCAM_NO_MARKER: c_int = 121;
pub const SFE_IRCAM_BAD_CHANNELS: c_int = 122;
pub const SFE_IRCAM_UNKNOWN_FORMAT: c_int = 123;

pub const SFE_W64_64_BIT: c_int = 124;
pub const SFE_W64_NO_RIFF: c_int = 125;
pub const SFE_W64_NO_WAVE: c_int = 126;
pub const SFE_W64_NO_DATA: c_int = 127;
pub const SFE_W64_ADPCM_NOT4BIT: c_int = 128;
pub const SFE_W64_ADPCM_CHANNELS: c_int = 129;
pub const SFE_W64_GSM610_FORMAT: c_int = 130;

pub const SFE_MAT4_BAD_NAME: c_int = 131;
pub const SFE_MAT4_NO_SAMPLERATE: c_int = 132;

pub const SFE_MAT5_BAD_ENDIAN: c_int = 133;
pub const SFE_MAT5_NO_BLOCK: c_int = 134;
pub const SFE_MAT5_SAMPLE_RATE: c_int = 135;

pub const SFE_PVF_NO_PVF1: c_int = 136;
pub const SFE_PVF_BAD_HEADER: c_int = 137;
pub const SFE_PVF_BAD_BITWIDTH: c_int = 138;

pub const SFE_DWVW_BAD_BITWIDTH: c_int = 139;
pub const SFE_G72X_NOT_MONO: c_int = 140;
pub const SFE_NMS_ADPCM_NOT_MONO: c_int = 141;

pub const SFE_XI_BAD_HEADER: c_int = 142;
pub const SFE_XI_EXCESS_SAMPLES: c_int = 143;
pub const SFE_XI_NO_PIPE: c_int = 144;

pub const SFE_HTK_NO_PIPE: c_int = 145;

pub const SFE_SDS_NOT_SDS: c_int = 146;
pub const SFE_SDS_BAD_BIT_WIDTH: c_int = 147;

pub const SFE_SD2_FD_DISALLOWED: c_int = 148;
pub const SFE_SD2_BAD_DATA_OFFSET: c_int = 149;
pub const SFE_SD2_BAD_MAP_OFFSET: c_int = 150;
pub const SFE_SD2_BAD_DATA_LENGTH: c_int = 151;
pub const SFE_SD2_BAD_MAP_LENGTH: c_int = 152;
pub const SFE_SD2_BAD_RSRC: c_int = 153;
pub const SFE_SD2_BAD_SAMPLE_SIZE: c_int = 154;

pub const SFE_FLAC_BAD_HEADER: c_int = 155;
pub const SFE_FLAC_NEW_DECODER: c_int = 156;
pub const SFE_FLAC_INIT_DECODER: c_int = 157;
pub const SFE_FLAC_LOST_SYNC: c_int = 158;
pub const SFE_FLAC_BAD_SAMPLE_RATE: c_int = 159;
pub const SFE_FLAC_CHANNEL_COUNT_CHANGED: c_int = 160;
pub const SFE_FLAC_UNKOWN_ERROR: c_int = 161;

pub const SFE_WVE_NOT_WVE: c_int = 162;
pub const SFE_WVE_NO_PIPE: c_int = 163;

pub const SFE_VORBIS_ENCODER_BUG: c_int = 164;

pub const SFE_RF64_NOT_RF64: c_int = 165;
pub const SFE_RF64_PEAK_B4_FMT: c_int = 166;
pub const SFE_RF64_NO_DATA: c_int = 167;

pub const SFE_BAD_CHUNK_PTR: c_int = 168;
pub const SFE_UNKNOWN_CHUNK: c_int = 169;
pub const SFE_BAD_CHUNK_FORMAT: c_int = 170;
pub const SFE_BAD_CHUNK_MARKER: c_int = 171;
pub const SFE_BAD_CHUNK_DATA_PTR: c_int = 172;
pub const SFE_ALAC_FAIL_TMPFILE: c_int = 173;
pub const SFE_FILENAME_TOO_LONG: c_int = 174;
pub const SFE_NEGATIVE_RW_LEN: c_int = 175;

pub const SFE_OPUS_BAD_SAMPLERATE: c_int = 176;

pub const SFE_MAX_ERROR: c_int = 177; /* This must be last in list. */

extern "C" {
    pub(crate) fn psf_log_printf(psf: *mut SF_PRIVATE, format: *const c_char, ...);
}
