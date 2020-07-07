#[cfg(windows)]
pub use windows::*;

#[cfg(not(windows))]
pub use unix::*;

#[cfg(not(windows))]
pub mod unix {
    use libc::{c_int, c_void};

    use crate::{SF_BOOL, sf_count_t};
    use crate::common::{SF_PRIVATE, SF_SEEK_MODE};

    extern {
        fn psf_close_rsrc(psf: *mut SF_PRIVATE) -> c_int;
        fn psf_set_stdio(psf: *mut SF_PRIVATE) -> c_int;
        fn psf_init_files(psf: *mut SF_PRIVATE);
        fn psf_set_file(psf: *mut SF_PRIVATE, fd: c_int);
        fn psf_file_valid(psf: *mut SF_PRIVATE) -> c_int;
        fn psf_fopen(psf: *mut SF_PRIVATE) -> c_int;
        fn psf_fclose(psf: *mut SF_PRIVATE) -> c_int;
        fn psf_open_rsrc(psf: *mut SF_PRIVATE) -> c_int;
        fn psf_get_filelen(psf: *mut SF_PRIVATE) -> sf_count_t;
        fn psf_use_rsrc(psf: *mut SF_PRIVATE, on_off: SF_BOOL);
        fn psf_fseek(
            psf: *mut SF_PRIVATE,
            offset: sf_count_t,
            whence: SF_SEEK_MODE,
        ) -> sf_count_t;
        fn psf_fread(
            ptr: *mut c_void,
            bytes: sf_count_t,
            items: sf_count_t,
            psf: *mut SF_PRIVATE,
        ) -> sf_count_t;
        fn psf_fwrite(
            ptr: *const c_void,
            bytes: sf_count_t,
            items: sf_count_t,
            psf: *mut SF_PRIVATE,
        ) -> sf_count_t;
        fn psf_ftell(psf: *mut SF_PRIVATE) -> sf_count_t;
        fn psf_is_pipe(psf: *mut SF_PRIVATE) -> c_int;
        fn psf_fsync(psf: *mut SF_PRIVATE);
        fn psf_ftruncate(psf: *mut SF_PRIVATE, len: sf_count_t) -> c_int;
    }
}

#[cfg(windows)]
pub mod windows {
    use crate::common::*;
    use crate::{sf_count_t, SFM_OPEN_MODE, SF_FALSE, SF_TRUE};

    use std::mem::zeroed;
    use std::ptr;

    use byte_strings::c_str;
    use errno;
    use libc::*;
    use winapi::shared::minwindef::*;
    use winapi::shared::winerror::*;
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::um::fileapi::*;
    use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
    use winapi::um::processenv::GetStdHandle;
    use winapi::um::winbase::*;
    use winapi::um::winnt::*;

    const SENSIBLE_SIZE: sf_count_t = 0x40000000;

    #[no_mangle]
    unsafe fn psf_log_syserr(psf: *mut SF_PRIVATE, error: c_int) {
        assert!(!psf.is_null());
        let psf = &mut *psf;

        let lpMsgBuf: LPSTR = ptr::null_mut();

        /* Only log an error if no error has been set yet. */
        if psf.error == SFE::NO_ERROR {
            psf.error = SFE::SYSTEM;

            FormatMessageA(
                FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM,
                ptr::null_mut(),
                error as DWORD,
                MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT).into(),
                lpMsgBuf,
                0,
                ptr::null_mut(),
            );

            snprintf(
                psf.syserr.as_mut_ptr() as _,
                SF_SYSERR_LEN as size_t,
                c_str!("System error : %s").as_ptr(),
                lpMsgBuf as *mut c_char,
            );
            LocalFree(lpMsgBuf as _);
        }
    }

    #[no_mangle]
    pub(crate) unsafe fn psf_close_rsrc(psf: *mut SF_PRIVATE) -> c_int {
        assert!(!psf.is_null());
        let psf = &mut *psf;

        psf_close_handle(psf.rsrc.handle);
        psf.rsrc.handle = ptr::null_mut();
        return 0;
    }

    #[no_mangle]
    pub(crate) unsafe fn psf_set_stdio(psf: *mut SF_PRIVATE) -> SFE {
        assert!(!psf.is_null());
        let psf = &mut *psf;

        let mut handle: HANDLE = ptr::null_mut();
        let mut error = SFE::NO_ERROR;

        match psf.file.mode {
            SFM_OPEN_MODE::RDWR => error = SFE::OPEN_PIPE_RDWR,

            SFM_OPEN_MODE::READ => {
                handle = GetStdHandle(STD_INPUT_HANDLE);
                psf.file.do_not_close_descriptor = 1;
            }

            SFM_OPEN_MODE::WRITE => {
                handle = GetStdHandle(STD_OUTPUT_HANDLE);
                psf.file.do_not_close_descriptor = 1;
            }
        }

        psf.file.handle = handle;
        psf.filelength = 0;

        error
    }

    #[no_mangle]
    pub(crate) unsafe fn psf_init_files(psf: *mut SF_PRIVATE) {
        assert!(!psf.is_null());
        let psf = &mut *psf;

        psf.file.handle = ptr::null_mut();
        psf.rsrc.handle = ptr::null_mut();
        psf.file.hsaved = ptr::null_mut();
    }

    #[no_mangle]
    pub(crate) unsafe fn psf_set_file(psf: *mut SF_PRIVATE, fd: c_int) {
        assert!(!psf.is_null());
        let psf = &mut *psf;

        let osfhandle = get_osfhandle(fd);
        let handle = osfhandle as HANDLE;

        psf.file.handle = handle;
    } /* psf_set_file */

    #[no_mangle]
    pub(crate) unsafe fn psf_file_valid(psf: *mut SF_PRIVATE) -> c_int {
        assert!(!psf.is_null());
        let psf = &mut *psf;

        if psf.file.handle == ptr::null_mut() {
            return SF_FALSE;
        }
        if psf.file.handle == INVALID_HANDLE_VALUE {
            return SF_FALSE;
        }
        return SF_TRUE;
    }

    #[no_mangle]
    pub(crate) unsafe fn psf_fopen(psf: *mut SF_PRIVATE) -> SFE {
        assert!(!psf.is_null());
        let psf = &mut *psf;

        psf.error = SFE::NO_ERROR;
        psf.file.handle = psf_open_handle(&mut psf.file);

        if psf.file.handle.is_null() {
            psf_log_syserr(psf, GetLastError() as c_int);
        }

        return psf.error;
    }

    #[no_mangle]
    pub(crate) unsafe fn psf_fclose(psf: *mut SF_PRIVATE) -> c_int {
        assert!(!psf.is_null());
        let psf = &mut *psf;

        if psf.virtual_io != SF_FALSE {
            return 0;
        }

        if psf.file.do_not_close_descriptor != SF_FALSE {
            psf.file.handle = ptr::null_mut();
            return 0;
        };

        let retval = psf_close_handle(psf.file.handle);
        if retval == -1 {
            psf_log_syserr(psf, GetLastError() as c_int);
        }

        psf.file.handle = ptr::null_mut();

        retval
    }

    #[no_mangle]
    pub(crate) unsafe fn psf_open_rsrc(psf: *mut SF_PRIVATE) -> SFE {
        assert!(!psf.is_null());
        let psf = &mut *psf;

        if !psf.rsrc.handle.is_null() {
            return SFE::NO_ERROR;
        }

        /* Test for MacOSX style resource fork on HPFS or HPFS+ filesystems. */
        snprintf(
            psf.rsrc.path.c.as_mut_ptr(),
            SF_FILENAME_LEN as usize,
            c_str!("%s/rsrc").as_ptr(),
            psf.file.path.c.as_ptr(),
        );
        psf.error = SFE::NO_ERROR;
        psf.rsrc.handle = psf_open_handle(&mut psf.rsrc);
        if !psf.rsrc.handle.is_null() {
            psf.rsrclength = psf_get_filelen_handle(psf.rsrc.handle);
            return SFE::NO_ERROR;
        };

        /*
         ** Now try for a resource fork stored as a separate file in the same
         ** directory, but preceded with a dot underscore.
         */
        snprintf(
            psf.rsrc.path.c.as_mut_ptr(),
            SF_FILENAME_LEN as usize,
            c_str!("%s._%s").as_ptr(),
            psf.file.dir.c,
            psf.file.name.c.as_ptr(),
        );
        psf.error = SFE::NO_ERROR;
        psf.rsrc.handle = psf_open_handle(&mut psf.rsrc);
        if !psf.rsrc.handle.is_null() {
            psf.rsrclength = psf_get_filelen_handle(psf.rsrc.handle);
            return SFE::NO_ERROR;
        };

        /*
         ** Now try for a resource fork stored in a separate file in the
         ** .AppleDouble/ directory.
         */
        snprintf(
            psf.rsrc.path.c.as_mut_ptr(),
            SF_FILENAME_LEN as usize,
            c_str!("%s.AppleDouble/%s").as_ptr(),
            psf.file.dir.c.as_ptr(),
            psf.file.name.c.as_ptr(),
        );
        psf.error = SFE::NO_ERROR;
        psf.rsrc.handle = psf_open_handle(&mut psf.rsrc);
        if !psf.rsrc.handle.is_null() {
            psf.rsrclength = psf_get_filelen_handle(psf.rsrc.handle);
            return SFE::NO_ERROR;
        };

        /* No resource file found. */
        if psf.rsrc.handle.is_null() {
            psf_log_syserr(psf, GetLastError() as c_int);
        }

        psf.rsrc.handle = ptr::null_mut();

        return psf.error;
    }

    #[no_mangle]
    pub(crate) unsafe fn psf_get_filelen(psf: *mut SF_PRIVATE) -> sf_count_t {
        assert!(!psf.is_null());
        let psf = &mut *psf;

        if psf.virtual_io != SF_FALSE {
            if let Some(vio_get_filelen) = psf.vio.get_filelen {
                return vio_get_filelen(psf.vio_user_data);
            } else {
                psf.error = SFE::BAD_VIRTUAL_IO;
                return -1;
            }
        }

        let mut filelen = psf_get_filelen_handle(psf.file.handle);

        if filelen == -1 {
            psf_log_syserr(psf, errno::errno().into());
            return -1;
        };

        if filelen == -SFE_BAD_STAT_SIZE as sf_count_t {
            psf.error = SFE::BAD_STAT_SIZE;
            return -1;
        };

        match psf.file.mode {
            SFM_OPEN_MODE::WRITE => filelen = filelen - psf.fileoffset,

            SFM_OPEN_MODE::READ => {
                if psf.fileoffset > 0 && psf.filelength > 0 {
                    filelen = psf.filelength;
                }
            }

            SFM_OPEN_MODE::RDWR => {},
        }

        filelen
    }

    #[no_mangle]
    pub(crate) unsafe fn psf_use_rsrc(psf: *mut SF_PRIVATE, on_off: SF_BOOL) {
        assert!(!psf.is_null());
        let psf = &mut *psf;

        if on_off != SF_BOOL::FALSE {
            if psf.file.handle != psf.rsrc.handle {
                psf.file.hsaved = psf.file.handle;
                psf.file.handle = psf.rsrc.handle;
            }
        } else if psf.file.handle == psf.rsrc.handle {
            psf.file.handle = psf.file.hsaved;
        }
    }

    #[no_mangle]
    unsafe fn psf_open_handle(pfile: *mut PSF_FILE) -> HANDLE {
        assert!(!pfile.is_null());
        let pfile = &mut *pfile;

        let (dwDesiredAccess, dwShareMode, dwCreationDistribution) = match pfile.mode {
            SFM_OPEN_MODE::READ => (
                GENERIC_READ,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                OPEN_EXISTING,
            ),

            SFM_OPEN_MODE::WRITE => (
                GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                CREATE_ALWAYS,
            ),

            SFM_OPEN_MODE::RDWR => (
                GENERIC_READ | GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                OPEN_ALWAYS,
            ),
        };

        let handle = if pfile.use_wchar != SF_FALSE {
            CreateFileW(
                pfile.path.wc.as_ptr(),
                dwDesiredAccess,
                dwShareMode,
                ptr::null_mut(),
                dwCreationDistribution,
                FILE_ATTRIBUTE_NORMAL,
                ptr::null_mut(),
            )
        } else {
            CreateFileA(
                pfile.path.c.as_ptr(),
                dwDesiredAccess,
                dwShareMode,
                ptr::null_mut(),
                dwCreationDistribution,
                FILE_ATTRIBUTE_NORMAL,
                ptr::null_mut(),
            )
        };

        if handle == INVALID_HANDLE_VALUE {
            return ptr::null_mut();
        }

        handle
    }

    #[no_mangle]
    pub(crate) unsafe fn psf_fseek(
        psf: *mut SF_PRIVATE,
        offset: sf_count_t,
        whence: SF_SEEK_MODE,
    ) -> sf_count_t {
        assert!(!psf.is_null());
        let psf = &mut *psf;

        if psf.virtual_io != SF_FALSE {
            if let Some(vio_seek) = psf.vio.seek {
                return vio_seek(offset, whence, psf.vio_user_data);
            } else {
                psf.error = SFE::BAD_VIRTUAL_IO;
                return -1;
            }
        }

        let mut offset = offset;
        let dwMoveMethod = match whence {
            SF_SEEK_MODE::SET => {
                offset += psf.fileoffset;
                FILE_BEGIN
            }

            SF_SEEK_MODE::END => FILE_END,

            _ => FILE_CURRENT,
        };

        let mut liDistanceToMove = zeroed::<LARGE_INTEGER>();
        let mut liNewFilePointer = zeroed::<LARGE_INTEGER>();
        *liDistanceToMove.QuadPart_mut() = offset;

        let fResult = SetFilePointerEx(
            psf.file.handle,
            liDistanceToMove,
            &mut liNewFilePointer,
            dwMoveMethod,
        );

        let dwError = if fResult == FALSE {
            GetLastError()
        } else {
            NO_ERROR
        };

        if dwError != NO_ERROR {
            psf_log_syserr(psf, dwError as c_int);
            return -1;
        }

        *liNewFilePointer.QuadPart_mut() - psf.fileoffset
    }

    #[no_mangle]
    pub(crate) unsafe fn psf_fread(
        ptr: *mut c_void,
        bytes: sf_count_t,
        items: sf_count_t,
        psf: *mut SF_PRIVATE,
    ) -> sf_count_t {
        let mut total: sf_count_t = 0;

        assert!(!psf.is_null());
        let psf = &mut *psf;

        if psf.virtual_io != SF_FALSE {
            if let Some(vio_read) = psf.vio.read {
                return vio_read(ptr, bytes * items, psf.vio_user_data) / bytes;
            }
        }

        let mut items = items * bytes;

        /* Do this check after the multiplication above. */
        if items <= 0 {
            return 0;
        }

        while items > 0 {
            /* Break the writes down to a sensible size. */
            let mut count = if items > SENSIBLE_SIZE {
                SENSIBLE_SIZE
            } else {
                items
            };
            let mut dwNumberOfBytesRead: DWORD = 0;

            if ReadFile(
                psf.file.handle,
                ptr.offset(total as isize) as _,
                count as DWORD,
                &mut dwNumberOfBytesRead,
                ptr::null_mut(),
            ) == 0
            {
                psf_log_syserr(psf, GetLastError() as c_int);
                break;
            } else {
                count = dwNumberOfBytesRead as sf_count_t;
            }

            if count == 0 {
                break;
            }

            total += count;
            items -= count;
        }

        if psf.is_pipe != SF_FALSE {
            psf.pipeoffset += total;
        }

        return total / bytes;
    }

    #[no_mangle]
    pub(crate) unsafe fn psf_fwrite(
        ptr: *const c_void,
        bytes: sf_count_t,
        items: sf_count_t,
        psf: *mut SF_PRIVATE,
    ) -> sf_count_t {
        assert!(!psf.is_null());
        let psf = &mut *psf;

        if psf.virtual_io != SF_FALSE {
            if let Some(vio_write) = psf.vio.write {
                return vio_write(ptr, bytes * items, psf.vio_user_data) / bytes;
            };
        }

        let mut items = items * bytes;

        /* Do this check after the multiplication above. */
        if items <= 0 {
            return 0;
        }

        let mut total = 0;
        while items > 0 {
            let mut dwNumberOfBytesWritten: DWORD = 0;

            /* Break the writes down to a sensible size. */
            let mut count = if items > SENSIBLE_SIZE {
                SENSIBLE_SIZE
            } else {
                items
            };

            if WriteFile(
                psf.file.handle,
                ptr.offset(total as isize) as _,
                count as DWORD,
                &mut dwNumberOfBytesWritten,
                ptr::null_mut(),
            ) == 0
            {
                psf_log_syserr(psf, GetLastError() as c_int);
                break;
            } else {
                count = dwNumberOfBytesWritten as sf_count_t;
            }

            if count == 0 {
                break;
            }

            total += count;
            items -= count;
        }

        if psf.is_pipe != SF_FALSE {
            psf.pipeoffset += total;
        }

        return total / bytes;
    }

    #[no_mangle]
    pub(crate) unsafe fn psf_ftell(psf: *mut SF_PRIVATE) -> sf_count_t {
        assert!(!psf.is_null());
        let psf = &mut *psf;

        if psf.virtual_io != SF_FALSE {
            if let Some(vio_tell) = psf.vio.tell {
                return vio_tell(psf.vio_user_data);
            } else {
                psf.error = SFE::BAD_VIRTUAL_IO;
                return -1;
            }
        }

        if psf.is_pipe != SF_FALSE {
            return psf.pipeoffset;
        }

        let liDistanceToMove = zeroed::<LARGE_INTEGER>();
        let mut liNewFilePointer = zeroed::<LARGE_INTEGER>();

        let fResult = SetFilePointerEx(
            psf.file.handle,
            liDistanceToMove,
            &mut liNewFilePointer,
            FILE_CURRENT,
        );

        let dwError = if fResult == FALSE {
            GetLastError()
        } else {
            NO_ERROR
        };

        if dwError != NO_ERROR {
            psf_log_syserr(psf, dwError as c_int);
            return -1;
        }

        let pos = *liNewFilePointer.QuadPart();

        return pos - psf.fileoffset;
    }

    #[no_mangle]
    unsafe fn psf_close_handle(handle: HANDLE) -> c_int {
        if handle.is_null() {
            return 0;
        }

        if CloseHandle(handle) == 0 {
            return -1;
        }

        return 0;
    }

    #[no_mangle]
    pub(crate) unsafe fn psf_is_pipe(psf: *mut SF_PRIVATE) -> c_int {
        assert!(!psf.is_null());
        let psf = &mut *psf;

        if psf.virtual_io != SF_FALSE {
            return SF_FALSE;
        }

        if GetFileType(psf.file.handle) == FILE_TYPE_DISK {
            return SF_FALSE;
        }

        /* Default to maximum safety. */
        return SF_TRUE;
    }

    #[no_mangle]
    unsafe fn psf_get_filelen_handle(handle: HANDLE) -> sf_count_t {
        let mut dwError = NO_ERROR;
        let mut liFileSize = zeroed::<LARGE_INTEGER>();
        let fResult = GetFileSizeEx(handle, &mut liFileSize);

        if fResult == FALSE {
            dwError = GetLastError();
        }

        if dwError != NO_ERROR {
            return -1;
        }

        let filelen = liFileSize.QuadPart();

        return *filelen;
    }

    #[no_mangle]
    pub(crate) unsafe fn psf_fsync(psf: *mut SF_PRIVATE) {
        assert!(!psf.is_null());
        let psf = &mut *psf;

        FlushFileBuffers(psf.file.handle);
    }

    #[no_mangle]
    pub(crate) unsafe fn psf_ftruncate(psf: *mut SF_PRIVATE, len: sf_count_t) -> c_int {
        assert!(!psf.is_null());
        let psf = &mut *psf;

        /* This implementation trashes the current file position.
         ** should it save and restore it? what if the current position is past
         ** the new end of file?
         */

        /* Returns 0 on success, non-zero on failure. */
        if len < 0 {
            return 1;
        }

        let mut liDistanceToMove = zeroed::<LARGE_INTEGER>();
        *liDistanceToMove.QuadPart_mut() = len.into();

        let fResult = SetFilePointerEx(
            psf.file.handle,
            liDistanceToMove,
            ptr::null_mut(),
            FILE_BEGIN,
        );

        let mut dwError = 0;
        if fResult == FALSE {
            dwError = GetLastError();
        }

        let mut retval = 0;
        if dwError != NO_ERROR {
            retval = -1;
            psf_log_syserr(psf, dwError as c_int);
        } else {
            /* Note: when SetEndOfFile is used to extend a file, the contents of the
             ** new portion of the file is undefined. This is unlike chsize(),
             ** which guarantees that the new portion of the file will be zeroed.
             ** Not sure if this is important or not.
             */
            if SetEndOfFile(psf.file.handle) == 0 {
                retval = -1;
                psf_log_syserr(psf, GetLastError() as c_int);
            }
        }

        return retval;
    } /* psf_ftruncate */

    extern "C" {
        pub fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, _: ...) -> c_int;
    }
}
