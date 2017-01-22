//! Tool for playing audio files.

// The implementation is based on
// http://hzqtc.github.io/2012/05/play-mp3-with-libmpg123-and-libao.html

extern crate ao_sys as ao;
extern crate libc;
extern crate mpg123_sys as mpg123;

use libc::{c_int, c_long, uint32_t};

use std::ffi::CString;
use std::path::Path;
use std::ptr;

macro_rules! raise(
    ($message:expr) => (return Err($message));
);

/// Play an audio file.
pub fn play<T: AsRef<Path>>(path: T) -> Result<(), &'static str> {
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
        mpg123::mpg123_init();
        let mut error: c_int = 0;
        let handle = mpg123::mpg123_new(ptr::null(), &mut error);
        if error != mpg123::MPG123_OK as c_int {
            raise!("something went wrong");
        }
        error = mpg123::mpg123_open(handle, path.as_ptr());
        if error != mpg123::MPG123_OK as c_int {
            mpg123::mpg123_delete(handle);
            mpg123::mpg123_exit();
            raise!("failed to open the file");
        }
        let mut rate: c_long = 0;
        let mut channels: c_int = 0;
        let mut encoding: c_int = 0;
        error = mpg123::mpg123_getformat(handle, &mut rate, &mut channels, &mut encoding);
        if error != mpg123::MPG123_OK as c_int {
            mpg123::mpg123_close(handle);
            mpg123::mpg123_delete(handle);
            mpg123::mpg123_exit();
            raise!("failed to get the format");
        }

        ao::ao_initialize();
        let mut format = ao::ao_sample_format {
            bits: 8 * mpg123::mpg123_encsize(encoding),
            rate: rate as c_int,
            channels: channels,
            byte_format: ao::AO_FMT_NATIVE,
            matrix: ptr::null_mut(),
        };
        let driver = ao::ao_default_driver_id();
        let device = ao::ao_open_live(driver, &mut format, ptr::null_mut());

        let buffer_size = mpg123::mpg123_outblock(handle);
        let buffer = libc::malloc(8 * buffer_size) as *mut _;
        let mut done = 0;
        loop {
            error = mpg123::mpg123_read(handle, buffer, buffer_size, &mut done);
            if error != mpg123::MPG123_OK as c_int {
                break;
            }
            ao::ao_play(device, buffer as *mut _, done as uint32_t);
        }
        libc::free(buffer as *mut _);

        ao::ao_close(device);
        ao::ao_shutdown();

        mpg123::mpg123_close(handle);
        mpg123::mpg123_delete(handle);
        mpg123::mpg123_exit();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn play() {
        ::play("tests/fixtures/sound.mp3").unwrap();
    }
}
