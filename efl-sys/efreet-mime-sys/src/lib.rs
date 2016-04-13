extern crate libc;

use libc::*;
/* automatically generated by rust-bindgen */

#[link(name = "efreet_mime")]
extern "C" {
    pub fn efreet_mime_init() -> c_int;
    pub fn efreet_mime_shutdown() -> c_int;
    pub fn efreet_mime_type_get(file: *const c_char)
     -> *const c_char;
    pub fn efreet_mime_magic_type_get(file: *const c_char)
     -> *const c_char;
    pub fn efreet_mime_globs_type_get(file: *const c_char)
     -> *const c_char;
    pub fn efreet_mime_special_type_get(file: *const c_char)
     -> *const c_char;
    pub fn efreet_mime_fallback_type_get(file: *const c_char)
     -> *const c_char;
    pub fn efreet_mime_type_icon_get(mime: *const c_char,
                                     theme: *const c_char,
                                     size: c_uint)
     -> *const c_char;
    pub fn efreet_mime_type_cache_clear();
    pub fn efreet_mime_type_cache_flush();
}