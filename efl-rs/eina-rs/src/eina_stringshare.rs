#![allow(dead_code)]
use libc::*;
use eina_ffi::*;
use std::ffi::{CStr, CString};

pub struct EinaStringshare {
    pub ptr: *mut Eina_Stringshare,
}

pub struct EinaTmpstr {
    pub ptr: *mut Eina_Tmpstr,
}

impl EinaStringshare {
    /// Retrieve an instance of a string with a specific size for use in a program
    ///
    /// This function retrieves an instance of str. If str is NULL, then NULL is returned. If str is already stored, it is just returned and its reference counter is increased. Otherwise a duplicated string of str is returned.
    ///
    /// This function does not check string size, but uses the exact given size. This can be used to share_common part of a larger buffer or substring.
    pub fn add_length(string: &CStr, slen: c_uint) -> Option<EinaStringshare> {
        unsafe {
            let pointer = eina_stringshare_add_length(string.as_ptr(), slen);
            match pointer.is_null() {
                false => Some(EinaStringshare{ptr: pointer}),
                true => None,
            }
        }
    }

    /// Retrieve an instance of a string for use in a program
    /// This function retrieves an instance of str. If str is NULL, then NULL is returned. If str is already stored, it is just returned and its reference counter is increased. Otherwise a duplicated string of str is returned.
    /// 
    /// The string str must be NULL terminated ('\0') and its full length will be used. To use part of the string or non-null terminated, use eina_stringshare_add_length() instead.
    pub fn add(string: &CStr) -> Option<EinaStringshare> {
        unsafe {
            let pointer = eina_stringshare_add(string.as_ptr());
            match pointer.is_null() {
                false => Some(EinaStringshare{ptr: pointer}),
                true => None,
            }
        }
    }

    /// Increment references of the given shared string
    ///
    /// This is similar to eina_share_common_add(), but it's faster since it will avoid lookups if possible, but on the down side it requires the parameter to be shared string. In other words, it must be the return of a previous call to one of the stringshare functions.
    pub fn incr_ref(&mut self) -> Option<EinaStringshare>{
        unsafe {
            let string = eina_stringshare_ref(self.ptr);
            match string.is_null() {
                false => Some(EinaStringshare{ptr: string}),
                true => None,
            }
        }
    }

    /// Note that the given string must be shared
    ///
    /// If the given pointer is not shared, bad things will happen, likely a segmentation fault. If in doubt, try strlen(). 
    pub fn strlen(&mut self) -> c_int {
        unsafe {
            eina_stringshare_strlen(self.ptr)
        }
    }

}

impl Drop for EinaStringshare {
    fn drop(&mut self) {
        unsafe {
            eina_stringshare_del(self.ptr);
        }
    }
}

impl EinaTmpstr {
    /// Add a new temporary string based on the input string
    ///
    /// When you add a temporary string (tmpstr) it is expected to have a very short lifespan, and at any one time only a few of these are intended to exist. This is not intended for longer term storage of strings. The intended use is the ability to safely pass strings as return values from functions directly into parameters of new functions and then have the string be cleaned up automatically by the caller.

    /// If str is NULL, or no memory space exists to store the tmpstr, then NULL will be returned, otherwise a valid string pointer will be returned that you can treat as any other C string (eg strdup(tmpstr) or printf("%s\n", tmpstr) etc.). This string should be considered read-only and immutable, and when youa re done with the string yo should delete it with eina_tmpstr_del().
    pub fn add(string: &CStr) -> Option<EinaTmpstr> {
        unsafe {
            let pointer = eina_tmpstr_add(string.as_ptr());
            match pointer.is_null() {
                false => Some(EinaTmpstr{ptr: pointer}),
                true => None,
            }
        }
    }

    /// Add a new temporary strin gbased on the input string and length
    ///
    /// When you add a temporary string (tmpstr) it is expected to have a very short lifespan, and at any one time only a few of these are intended to exist. This is not intended for longer term storage of strings. The intended use is the ability to safely pass strings as return values from functions directly into parameters of new functions and then have the string be cleaned up automatically by the caller.
    ///
    /// If str is NULL, or no memory space exists to store the tmpstr, then NULL will be returned, otherwise a valid string pointer will be returned that you can treat as any other C string (eg strdup(tmpstr) or printf("%s\n", tmpstr) etc.). This string should be considered read-only and immutable, and when you are done with the string you should delete it with eina_tmpstr_del().
    pub fn add_length(string: &CStr, length: size_t) -> Option<EinaTmpstr> {
        unsafe {
            let pointer = eina_tmpstr_add_length(string.as_ptr(), length);
            match pointer.is_null() {
                false => Some(EinaTmpstr{ptr: pointer}),
                true => None,
            }
        }
    }

    /// Return length of temporary string
    pub fn len(&self) -> size_t {
        unsafe {
            eina_tmpstr_len(self.ptr)
        }
    }

/*
    /// Add a new temporary string using the passed string.
    ///
    /// The passed string is used directly as the buffer. the passed strin gmust be malloced
    pub fn new_manage(string: CString) -> Option<EinaTmpstr>{
        unsafe {
            let pointer = eina_tmpstr_manage_new(string.as_ptr());
            match pointer {
                false => Some(EinaTmpstr{ptr: pointer}),
                true => None,
            }
        }
    }
    
    /// Add a new temporary string using the passed string.
    ///
    /// The passed string is used directly as the buffer. the passed strin gmust be malloced
    pub fn new_manage_length(string: CString, length: size_t) -> Option<EinaTmpstr>{
        unsafe {
            let pointer = eina_tmpstr_manage_new(string.as_ptr(), length);
            match pointer {
                false => Some(EinaTmpstr{ptr: pointer}),
                true => None,
            }
        }
    }
*/
}

impl Drop for EinaTmpstr {
    fn drop(&mut self) {
        unsafe {
            eina_tmpstr_del(self.ptr);
        }
    }
}

