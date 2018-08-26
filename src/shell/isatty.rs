//! Minified version of <https://github.com/dtolnay/isatty/blob/master/src/lib.rs>
//!
//! Written by David Tolnay. Licensed under the Apache License Version 2.0
//!
//! This was in turn inspired by:
//!
//!  - https://github.com/rust-lang/cargo/blob/099ad28104fe319f493dc42e0c694d468c65767d/src/cargo/lib.rs#L154-L178
//!  - https://github.com/BurntSushi/ripgrep/issues/94#issuecomment-261761687

use super::Stream;

#[cfg(not(any(windows, unix, target_os = "redox")))]
compile_error!("no support for your OS, sorry!");

#[cfg(unix)]
pub(crate) fn isatty(stream: Stream) -> bool {
    extern crate libc;

    let fd = match stream {
        Stream::Stdout => libc::STDOUT_FILENO,
        Stream::Stderr => libc::STDERR_FILENO,
    };

    #[allow(unsafe_code)]
    unsafe {
        libc::isatty(fd) != 0
    }
}

#[cfg(windows)]
pub(crate) use self::windows::isatty;

#[cfg(windows)]
mod windows {
    extern crate winapi;

    use super::Stream;

    pub(crate) fn isatty(stream: Stream) -> bool {
        let handle = match stream {
            Stream::Stdout => winapi::um::winbase::STD_OUTPUT_HANDLE,
            Stream::Stderr => winapi::um::winbase::STD_ERROR_HANDLE,
        };

        #[allow(unsafe_code)]
        unsafe {
            let handle = winapi::um::processenv::GetStdHandle(handle);

            // check for msys/cygwin
            if is_cygwin_pty(handle) {
                return true;
            }

            let mut out = 0;
            winapi::um::consoleapi::GetConsoleMode(handle, &mut out) != 0
        }
    }

    /// Returns true if there is an MSYS/cygwin tty on the given handle.
    fn is_cygwin_pty(handle: winapi::um::winnt::HANDLE) -> bool {
        use std::ffi::OsString;
        use std::mem;
        use std::os::windows::ffi::OsStringExt;
        use std::slice;

        use self::winapi::shared::minwindef::LPVOID;
        use self::winapi::shared::minwindef::MAX_PATH;
        use self::winapi::um::fileapi::FILE_NAME_INFO;
        use self::winapi::um::minwinbase::FileNameInfo;
        use self::winapi::um::winbase::GetFileInformationByHandleEx;

        #[allow(unsafe_code)]
        unsafe {
            let size = mem::size_of::<FILE_NAME_INFO>();
            let mut name_info_bytes = vec![0u8; size + MAX_PATH];
            let res = GetFileInformationByHandleEx(
                handle,
                FileNameInfo,
                &mut *name_info_bytes as *mut _ as LPVOID,
                name_info_bytes.len() as u32,
            );
            if res == 0 {
                return true;
            }
            let name_info: FILE_NAME_INFO =
                *(name_info_bytes[0..size].as_ptr() as *const FILE_NAME_INFO);
            let name_bytes = &name_info_bytes[size..size + name_info.FileNameLength as usize];
            let name_u16 =
                slice::from_raw_parts(name_bytes.as_ptr() as *const u16, name_bytes.len() / 2);
            let name = OsString::from_wide(name_u16)
                .as_os_str()
                .to_string_lossy()
                .into_owned();
            name.contains("msys-") || name.contains("-pty")
        }
    }
}

#[cfg(target_os = "redox")]
pub(crate) fn isatty(stream: Stream) -> bool {
    extern crate syscall;
    use std::io;
    use std::os::unix::io::AsRawFd;

    let raw_fd = match stream {
        Stream::Stdout => io::stdout().as_raw_fd(),
        Stream::Stderr => io::stderr().as_raw_fd(),
    };

    if let Ok(fd) = syscall::dup(raw_fd, b"termios") {
        let _ = syscall::close(fd);
        true
    } else {
        false
    }
}
