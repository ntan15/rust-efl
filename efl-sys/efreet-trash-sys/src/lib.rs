extern crate libc;
extern crate efreet_sys;
extern crate eina_sys;

use libc::*;
use efreet_sys::EfreetUri;
use eina_sys::EinaList;

/* automatically generated by rust-bindgen */

#[link(name = "efreet_trash")]
extern "C" {
    pub fn efreet_trash_init() -> c_int;
    pub fn efreet_trash_shutdown() -> c_int;
    pub fn efreet_trash_dir_get(for_file: *const c_char)
     -> *const c_char;
    pub fn efreet_trash_delete_uri(uri: *mut EfreetUri,
                                   force_delete: c_int)
     -> c_int;
    pub fn efreet_trash_ls() -> *mut EinaList;
    pub fn efreet_trash_is_empty() -> c_int;
    pub fn efreet_trash_empty_trash() -> c_int;
}