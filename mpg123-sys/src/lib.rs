#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_char, c_double, c_int, c_long, c_uchar, c_ulong, c_void, off_t, size_t, ssize_t};

#[derive(Clone, Copy)]
#[repr(C)]
pub enum mpg123_channelcount {
    MPG123_MONO = 1,
    MPG123_STEREO = 2,
}
pub use mpg123_channelcount::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum mpg123_channels {
    MPG123_LEFT = 0x1,
    MPG123_RIGHT = 0x2,
    MPG123_LR = 0x3,
}
pub use mpg123_channels::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum mpg123_errors {
    MPG123_DONE = -12,
    MPG123_NEW_FORMAT = -11,
    MPG123_NEED_MORE = -10,
    MPG123_ERR = -1,
    MPG123_OK = 0,
    MPG123_BAD_OUTFORMAT,
    MPG123_BAD_CHANNEL,
    MPG123_BAD_RATE,
    MPG123_ERR_16TO8TABLE,
    MPG123_BAD_PARAM,
    MPG123_BAD_BUFFER,
    MPG123_OUT_OF_MEM,
    MPG123_NOT_INITIALIZED,
    MPG123_BAD_DECODER,
    MPG123_BAD_HANDLE,
    MPG123_NO_BUFFERS,
    MPG123_BAD_RVA,
    MPG123_NO_GAPLESS,
    MPG123_NO_SPACE,
    MPG123_BAD_TYPES,
    MPG123_BAD_BAND,
    MPG123_ERR_NULL,
    MPG123_ERR_READER,
    MPG123_NO_SEEK_FROM_END,
    MPG123_BAD_WHENCE,
    MPG123_NO_TIMEOUT,
    MPG123_BAD_FILE,
    MPG123_NO_SEEK,
    MPG123_NO_READER,
    MPG123_BAD_PARS,
    MPG123_BAD_INDEX_PAR,
    MPG123_OUT_OF_SYNC,
    MPG123_RESYNC_FAIL,
    MPG123_NO_8BIT,
    MPG123_BAD_ALIGN,
    MPG123_NULL_BUFFER,
    MPG123_NO_RELSEEK,
    MPG123_NULL_POINTER,
    MPG123_BAD_KEY,
    MPG123_NO_INDEX,
    MPG123_INDEX_FAIL,
    MPG123_BAD_DECODER_SETUP,
    MPG123_MISSING_FEATURE,
    MPG123_BAD_VALUE,
    MPG123_LSEEK_FAILED,
    MPG123_BAD_CUSTOM_IO,
    MPG123_LFS_OVERFLOW,
    MPG123_INT_OVERFLOW,
}
pub use mpg123_errors::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum mpg123_feature_set {
    MPG123_FEATURE_ABI_UTF8OPEN = 0,
    MPG123_FEATURE_OUTPUT_8BIT,
    MPG123_FEATURE_OUTPUT_16BIT,
    MPG123_FEATURE_OUTPUT_32BIT,
    MPG123_FEATURE_INDEX,
    MPG123_FEATURE_PARSE_ID3V2,
    MPG123_FEATURE_DECODE_LAYER1,
    MPG123_FEATURE_DECODE_LAYER2,
    MPG123_FEATURE_DECODE_LAYER3,
    MPG123_FEATURE_DECODE_ACCURATE,
    MPG123_FEATURE_DECODE_DOWNSAMPLE,
    MPG123_FEATURE_DECODE_NTOM,
    MPG123_FEATURE_PARSE_ICY,
    MPG123_FEATURE_TIMEOUT_READ,
    MPG123_FEATURE_EQUALIZER,
}
pub use mpg123_feature_set::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum mpg123_flags {
    MPG123_CRC = 0x1,
    MPG123_COPYRIGHT = 0x2,
    MPG123_PRIVATE = 0x4,
    MPG123_ORIGINAL = 0x8,
}
pub use mpg123_flags::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct mpg123_frameinfo {
    pub version: mpg123_version,
    pub layer: c_int,
    pub rate: c_long,
    pub mode: mpg123_mode,
    pub mode_ext: c_int,
    pub framesize: c_int,
    pub flags: mpg123_flags,
    pub emphasis: c_int,
    pub bitrate: c_int,
    pub abr_rate: c_int,
    pub vbr: mpg123_vbr,
}

pub enum mpg123_handle {}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum mpg123_id3_enc {
    mpg123_id3_latin1 = 0,
    mpg123_id3_utf16bom = 1,
    mpg123_id3_utf16be = 2,
    mpg123_id3_utf8 = 3,
}
pub use mpg123_id3_enc::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum mpg123_id3_pic_type {
    mpg123_id3_pic_other = 0,
    mpg123_id3_pic_icon = 1,
    mpg123_id3_pic_other_icon = 2,
    mpg123_id3_pic_front_cover = 3,
    mpg123_id3_pic_back_cover = 4,
    mpg123_id3_pic_leaflet = 5,
    mpg123_id3_pic_media = 6,
    mpg123_id3_pic_lead = 7,
    mpg123_id3_pic_artist = 8,
    mpg123_id3_pic_conductor = 9,
    mpg123_id3_pic_orchestra = 10,
    mpg123_id3_pic_composer = 11,
    mpg123_id3_pic_lyricist = 12,
    mpg123_id3_pic_location = 13,
    mpg123_id3_pic_recording = 14,
    mpg123_id3_pic_performance = 15,
    mpg123_id3_pic_video = 16,
    mpg123_id3_pic_fish = 17,
    mpg123_id3_pic_illustration = 18,
    mpg123_id3_pic_artist_logo = 19,
    mpg123_id3_pic_publisher_logo = 20,
}
pub use mpg123_id3_pic_type::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct mpg123_id3v1 {
    pub tag: [c_char; 3],
    pub title: [c_char; 30],
    pub artist: [c_char; 30],
    pub album: [c_char; 30],
    pub year: [c_char; 4],
    pub comment: [c_char; 30],
    pub genre: c_uchar,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct mpg123_id3v2 {
    pub version: c_uchar,
    pub title: *mut mpg123_string,
    pub artist: *mut mpg123_string,
    pub album: *mut mpg123_string,
    pub year: *mut mpg123_string,
    pub genre: *mut mpg123_string,
    pub comment: *mut mpg123_string,
    pub comment_list: *mut mpg123_text,
    pub comments: size_t,
    pub text: *mut mpg123_text,
    pub texts: size_t,
    pub extra: *mut mpg123_text,
    pub extras: size_t,
    pub picture: *mut mpg123_picture,
    pub pictures: size_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum mpg123_mode {
    MPG123_M_STEREO = 0,
    MPG123_M_JOINT,
    MPG123_M_DUAL,
    MPG123_M_MONO,
}
pub use mpg123_mode::*;

pub enum mpg123_pars {}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum mpg123_parms {
    MPG123_VERBOSE = 0,
    MPG123_FLAGS,
    MPG123_ADD_FLAGS,
    MPG123_FORCE_RATE,
    MPG123_DOWN_SAMPLE,
    MPG123_RVA,
    MPG123_DOWNSPEED,
    MPG123_UPSPEED,
    MPG123_START_FRAME,
    MPG123_DECODE_FRAMES,
    MPG123_ICY_INTERVAL,
    MPG123_OUTSCALE,
    MPG123_TIMEOUT,
    MPG123_REMOVE_FLAGS,
    MPG123_RESYNC_LIMIT,
    MPG123_INDEX_SIZE,
    MPG123_PREFRAMES,
    MPG123_FEEDPOOL,
    MPG123_FEEDBUFFER,
}
pub use mpg123_parms::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum mpg123_param_flags {
    MPG123_FORCE_MONO = 0x7,
    MPG123_MONO_LEFT = 0x1,
    MPG123_MONO_RIGHT = 0x2,
    MPG123_MONO_MIX = 0x4,
    MPG123_FORCE_STEREO = 0x8,
    MPG123_FORCE_8BIT = 0x10,
    MPG123_QUIET = 0x20,
    MPG123_GAPLESS = 0x40,
    MPG123_NO_RESYNC = 0x80,
    MPG123_SEEKBUFFER = 0x100,
    MPG123_FUZZY = 0x200,
    MPG123_FORCE_FLOAT  = 0x400,
    MPG123_PLAIN_ID3TEXT = 0x800,
    MPG123_IGNORE_STREAMLENGTH = 0x1000,
    MPG123_SKIP_ID3V2 = 0x2000,
    MPG123_IGNORE_INFOFRAME = 0x4000,
    MPG123_AUTO_RESAMPLE = 0x8000,
    MPG123_PICTURE = 0x10000,
}
pub use mpg123_param_flags::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum mpg123_param_rva {
    MPG123_RVA_OFF = 0,
    MPG123_RVA_MIX = 1,
    MPG123_RVA_ALBUM = 2,
}
pub use mpg123_param_rva::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct mpg123_picture {
    pub _type: c_char,
    pub description: mpg123_string,
    pub mime_type: mpg123_string,
    pub size: size_t,
    pub data: *mut c_uchar,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum mpg123_state {
    MPG123_ACCURATE = 1,
    MPG123_BUFFERFILL,
    MPG123_FRANKENSTEIN,
    MPG123_FRESH_DECODER,
}
pub use mpg123_state::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct mpg123_string {
    pub p: *mut c_char,
    pub size: size_t,
    pub fill: size_t,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct mpg123_text {
    pub lang: [c_char; 3],
    pub id: [c_char; 4],
    pub description: mpg123_string,
    pub text: mpg123_string,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum mpg123_text_encoding {
    mpg123_text_unknown = 0,
    mpg123_text_utf8 = 1,
    mpg123_text_latin1 = 2,
    mpg123_text_icy = 3,
    mpg123_text_cp1252 = 4,
    mpg123_text_utf16 = 5,
    mpg123_text_utf16bom = 6,
    mpg123_text_utf16be = 7,
}
pub use mpg123_text_encoding::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum mpg123_vbr {
    MPG123_CBR = 0,
    MPG123_VBR,
    MPG123_ABR,
}
pub use mpg123_vbr::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub enum mpg123_version {
    MPG123_1_0 = 0,
    MPG123_2_0,
    MPG123_2_5,
}
pub use mpg123_version::*;

extern {
    pub fn mpg123_init() -> c_int;
    pub fn mpg123_exit();
    pub fn mpg123_new(decoder: *const c_char, error: *mut c_int) -> *mut mpg123_handle;
    pub fn mpg123_delete(mh: *mut mpg123_handle);
    pub fn mpg123_param(mh: *mut mpg123_handle, _type: mpg123_parms, value: c_long, fvalue: c_double) -> c_int;
    pub fn mpg123_getparam(mh: *mut mpg123_handle, _type: mpg123_parms, value: *mut c_long, fvalue: *mut c_double) -> c_int;
    pub fn mpg123_feature(key: mpg123_feature_set) -> c_int;
    pub fn mpg123_plain_strerror(errcode: c_int) -> *const c_char;
    pub fn mpg123_strerror(mh: *mut mpg123_handle) -> *const c_char;
    pub fn mpg123_errcode(mh: *mut mpg123_handle) -> c_int;
    pub fn mpg123_decoders() -> *mut *const c_char;
    pub fn mpg123_supported_decoders() -> *mut *const c_char;
    pub fn mpg123_decoder(mh: *mut mpg123_handle, decoder_name: *const c_char) -> c_int;
    pub fn mpg123_current_decoder(mh: *mut mpg123_handle) -> *const c_char;
    pub fn mpg123_rates(list: *mut *const c_long, number: *mut size_t);
    pub fn mpg123_encodings(list: *mut *const c_int, number: *mut size_t);
    pub fn mpg123_encsize(encoding: c_int) -> c_int;
    pub fn mpg123_format_none(mh: *mut mpg123_handle) -> c_int;
    pub fn mpg123_format_all(mh: *mut mpg123_handle) -> c_int;
    pub fn mpg123_format(mh: *mut mpg123_handle, rate: c_long, channels: c_int, encodings: c_int) -> c_int;
    pub fn mpg123_format_support(mh: *mut mpg123_handle, rate: c_long, encoding: c_int) -> c_int;
    pub fn mpg123_getformat(mh: *mut mpg123_handle, rate: *mut c_long, channels: *mut c_int, encoding: *mut c_int) -> c_int;
    pub fn mpg123_open(mh: *mut mpg123_handle, path: *const c_char) -> c_int;
    pub fn mpg123_open_fd(mh: *mut mpg123_handle, fd: c_int) -> c_int;
    pub fn mpg123_open_handle(mh: *mut mpg123_handle, iohandle: *mut c_void) -> c_int;
    pub fn mpg123_open_feed(mh: *mut mpg123_handle) -> c_int;
    pub fn mpg123_close(mh: *mut mpg123_handle) -> c_int;
    pub fn mpg123_read(mh: *mut mpg123_handle, outmemory: *mut c_uchar, outmemsize: size_t, done: *mut size_t) -> c_int;
    pub fn mpg123_feed(mh: *mut mpg123_handle, _in: *const c_uchar, size: size_t) -> c_int;
    pub fn mpg123_decode(mh: *mut mpg123_handle, inmemory: *const c_uchar, inmemsize: size_t, outmemory: *mut c_uchar, outmemsize: size_t, done: *mut size_t) -> c_int;
    pub fn mpg123_decode_frame(mh: *mut mpg123_handle, num: *mut off_t, audio: *mut *mut c_uchar, bytes: *mut size_t) -> c_int;
    pub fn mpg123_framebyframe_decode(mh: *mut mpg123_handle, num: *mut off_t, audio: *mut *mut c_uchar, bytes: *mut size_t) -> c_int;
    pub fn mpg123_framebyframe_next(mh: *mut mpg123_handle) -> c_int;
    pub fn mpg123_framedata(mh: *mut mpg123_handle, header: *mut c_ulong, bodydata: *mut *mut c_uchar, bodybytes: *mut size_t) -> c_int;
    pub fn mpg123_framepos(mh: *mut mpg123_handle) -> off_t;
    pub fn mpg123_tell(mh: *mut mpg123_handle) -> off_t;
    pub fn mpg123_tellframe(mh: *mut mpg123_handle) -> off_t;
    pub fn mpg123_tell_stream(mh: *mut mpg123_handle) -> off_t;
    pub fn mpg123_seek(mh: *mut mpg123_handle, sampleoff: off_t, whence: c_int) -> off_t;
    pub fn mpg123_feedseek(mh: *mut mpg123_handle, sampleoff: off_t, whence: c_int, input_offset: *mut off_t) -> off_t;
    pub fn mpg123_seek_frame(mh: *mut mpg123_handle, frameoff: off_t, whence: c_int) -> off_t;
    pub fn mpg123_timeframe(mh: *mut mpg123_handle, sec: c_double) -> off_t;
    pub fn mpg123_index(mh: *mut mpg123_handle, offsets: *mut *mut off_t, step: *mut off_t, fill: *mut size_t) -> c_int;
    pub fn mpg123_set_index(mh: *mut mpg123_handle, offsets: *mut off_t, step: off_t, fill: size_t) -> c_int;
    pub fn mpg123_position(mh: *mut mpg123_handle, frame_offset: off_t, buffered_bytes: off_t, current_frame: *mut off_t, frames_left: *mut off_t, current_seconds: *mut c_double, seconds_left: *mut c_double) -> c_int;
    pub fn mpg123_eq(mh: *mut mpg123_handle, channel: mpg123_channels, band: c_int, val: c_double) -> c_int;
    pub fn mpg123_geteq(mh: *mut mpg123_handle, channel: mpg123_channels, band: c_int) -> c_double;
    pub fn mpg123_reset_eq(mh: *mut mpg123_handle) -> c_int;
    pub fn mpg123_volume(mh: *mut mpg123_handle, vol: c_double) -> c_int;
    pub fn mpg123_volume_change(mh: *mut mpg123_handle, change: c_double) -> c_int;
    pub fn mpg123_getvolume(mh: *mut mpg123_handle, base: *mut c_double, really: *mut c_double, rva_db: *mut c_double) -> c_int;
    pub fn mpg123_info(mh: *mut mpg123_handle, mi: *mut mpg123_frameinfo) -> c_int;
    pub fn mpg123_safe_buffer() -> size_t;
    pub fn mpg123_scan(mh: *mut mpg123_handle) -> c_int;
    pub fn mpg123_framelength(mh: *mut mpg123_handle) -> off_t;
    pub fn mpg123_length(mh: *mut mpg123_handle) -> off_t;
    pub fn mpg123_set_filesize(mh: *mut mpg123_handle, size: off_t) -> c_int;
    pub fn mpg123_tpf(mh: *mut mpg123_handle) -> c_double;
    pub fn mpg123_spf(mh: *mut mpg123_handle) -> c_int;
    pub fn mpg123_clip(mh: *mut mpg123_handle) -> c_long;
    pub fn mpg123_getstate(mh: *mut mpg123_handle, key: mpg123_state, val: *mut c_long, fval: *mut c_double) -> c_int;
    pub fn mpg123_init_string(sb: *mut mpg123_string);
    pub fn mpg123_free_string(sb: *mut mpg123_string);
    pub fn mpg123_resize_string(sb: *mut mpg123_string, news: size_t) -> c_int;
    pub fn mpg123_grow_string(sb: *mut mpg123_string, news: size_t) -> c_int;
    pub fn mpg123_copy_string(from: *mut mpg123_string, to: *mut mpg123_string) -> c_int;
    pub fn mpg123_add_string(sb: *mut mpg123_string, stuff: *const c_char) -> c_int;
    pub fn mpg123_add_substring(sb: *mut mpg123_string, stuff: *const c_char, from: size_t, count: size_t) -> c_int;
    pub fn mpg123_set_string(sb: *mut mpg123_string, stuff: *const c_char) -> c_int;
    pub fn mpg123_set_substring(sb: *mut mpg123_string, stuff: *const c_char, from: size_t, count: size_t) -> c_int;
    pub fn mpg123_strlen(sb: *mut mpg123_string, utf8: c_int) -> size_t;
    pub fn mpg123_chomp_string(sb: *mut mpg123_string) -> c_int;
    pub fn mpg123_enc_from_id3(id3_enc_byte: c_uchar) -> mpg123_text_encoding;
    pub fn mpg123_store_utf8(sb: *mut mpg123_string, enc: mpg123_text_encoding, source: *const c_uchar, source_size: size_t) -> c_int;
    pub fn mpg123_meta_check(mh: *mut mpg123_handle) -> c_int;
    pub fn mpg123_meta_free(mh: *mut mpg123_handle);
    pub fn mpg123_id3(mh: *mut mpg123_handle, v1: *mut *mut mpg123_id3v1, v2: *mut *mut mpg123_id3v2) -> c_int;
    pub fn mpg123_icy(mh: *mut mpg123_handle, icy_meta: *mut *mut c_char) -> c_int;
    pub fn mpg123_icy2utf8(icy_text: *const c_char) -> *mut c_char;
    pub fn mpg123_parnew(mp: *mut mpg123_pars, decoder: *const c_char, error: *mut c_int) -> *mut mpg123_handle;
    pub fn mpg123_new_pars(error: *mut c_int) -> *mut mpg123_pars;
    pub fn mpg123_delete_pars(mp: *mut mpg123_pars);
    pub fn mpg123_fmt_none(mp: *mut mpg123_pars) -> c_int;
    pub fn mpg123_fmt_all(mp: *mut mpg123_pars) -> c_int;
    pub fn mpg123_fmt(mp: *mut mpg123_pars, rate: c_long, channels: c_int, encodings: c_int) -> c_int;
    pub fn mpg123_fmt_support(mp: *mut mpg123_pars, rate: c_long, encoding: c_int) -> c_int;
    pub fn mpg123_par(mp: *mut mpg123_pars, _type: mpg123_parms, value: c_long, fvalue: c_double) -> c_int;
    pub fn mpg123_getpar(mp: *mut mpg123_pars, _type: mpg123_parms, value: *mut c_long, fvalue: *mut c_double) -> c_int;
    pub fn mpg123_replace_buffer(mh: *mut mpg123_handle, data: *mut c_uchar, size: size_t) -> c_int;
    pub fn mpg123_outblock(mh: *mut mpg123_handle) -> size_t;

    pub fn mpg123_replace_reader(mh: *mut mpg123_handle,
                                 r_read: unsafe extern fn(c_int, *mut c_void, size_t) -> ssize_t,
                                 r_lseek: unsafe extern fn(c_int, off_t, c_int) -> off_t)
                                 -> c_int;

    pub fn mpg123_replace_reader_handle(mh: *mut mpg123_handle,
                                        r_read: unsafe extern fn(*mut c_void, *mut c_void, size_t)
                                                                 -> ssize_t,
                                        r_lseek: unsafe extern fn(*mut c_void, off_t, c_int)
                                                                  -> off_t,
                                        cleanup: unsafe extern fn(*mut c_void))
                                        -> c_int;
}
