//! Means of playing audio files.
//!
//! # Example
//!
//! ```
//! play::play("tests/fixtures/sound.mp3").unwrap();
//! ```

extern crate libc;
extern crate mpg123_sys as mpg123;
extern crate out123_sys as out123;

use libc::c_int;

use std::ffi::CString;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::ptr;

macro_rules! raise(($message:expr) => (return Err(Error::new(ErrorKind::Other, $message))));

/// Play an audio file.
pub fn play<T: AsRef<Path>>(path: T) -> Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        raise!("the file does not exist");
    }
    let path = match path.to_str() {
        Some(path) => match CString::new(path) {
            Ok(path) => path,
            _ => raise!("the path is malformed"),
        },
        _ => raise!("the path is malformed"),
    };
    unsafe {
        let mut error = mpg123::mpg123_init();
        if error != mpg123::MPG123_OK as c_int {
            raise!("failed to initialize mpg123");
        }
        let mpg123_handle;
        let mut out123_handle = ptr::null_mut();
        let mut buffer = ptr::null_mut();
        macro_rules! cleanup(
            () => ({
                if !buffer.is_null() {
                    libc::free(buffer as *mut _);
                }
                if !out123_handle.is_null() {
                    out123::out123_del(out123_handle);
                }
                if !mpg123_handle.is_null() {
                    mpg123::mpg123_close(mpg123_handle);
                    mpg123::mpg123_delete(mpg123_handle);
                }
                mpg123::mpg123_exit();
            });
        );
        macro_rules! cleanup_and_raise(
            ($message:expr) => ({
                cleanup!();
                raise!($message);
            });
        );
        mpg123_handle = mpg123::mpg123_new(ptr::null(), &mut error);
        if mpg123_handle.is_null() || error != mpg123::MPG123_OK as c_int {
            cleanup_and_raise!("failed to instantiate mpg123");
        }
        error = mpg123::mpg123_open(mpg123_handle, path.as_ptr());
        if error != mpg123::MPG123_OK as c_int {
            cleanup_and_raise!("failed to open the input");
        }
        let mut rate = 0;
        let mut channels = 0;
        let mut encoding = 0;
        error = mpg123::mpg123_getformat(mpg123_handle, &mut rate, &mut channels, &mut encoding);
        if error != mpg123::MPG123_OK as c_int {
            cleanup_and_raise!("failed to get the format");
        }
        out123_handle = out123::out123_new();
        if out123_handle.is_null() {
            cleanup_and_raise!("failed to instantiate out123");
        }
        error = out123::out123_open(out123_handle, ptr::null(), ptr::null());
        if error != out123::OUT123_OK as c_int {
            cleanup_and_raise!("failed to open the output");
        }
        error = out123::out123_start(out123_handle, rate, channels, encoding);
        if error != out123::OUT123_OK as c_int {
            cleanup_and_raise!("failed to start the output");
        }
        let buffer_size = mpg123::mpg123_outblock(mpg123_handle);
        buffer = libc::malloc(buffer_size) as *mut _;
        loop {
            let mut read = 0;
            error = mpg123::mpg123_read(mpg123_handle, buffer, buffer_size, &mut read);
            if error != mpg123::MPG123_OK as c_int && error != mpg123::MPG123_DONE as c_int {
                cleanup_and_raise!("failed to read the input");
            }
            if out123::out123_play(out123_handle, buffer as *mut _, read) != read {
                cleanup_and_raise!("failed to play the output");
            }
            if error == mpg123::MPG123_DONE as c_int {
                break;
            }
        }
        cleanup!();
    }
    Ok(())
}
