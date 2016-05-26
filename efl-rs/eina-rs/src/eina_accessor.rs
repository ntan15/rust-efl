#![allow(dead_code)]
use eina_ffi::*;

use libc::*;
use std::ptr;

pub struct EinaAccessor {
    pub ptr: *mut Eina_Accessor,
}

impl EinaAccessor {
    /// Gets the data of an accessor at the given position.
    pub fn get_data(&mut self, position: c_uint) -> Option<&mut c_void> {
        unsafe {
            let mut data: *mut c_void = ptr::null_mut();
            match eina_accessor_data_get(self.ptr, position, &mut data as *mut *mut _ as *mut *mut c_void) {
             EINA_TRUE => Some(&mut*data),
                _ => None,
            }
        }
    }

    /// Gets the container of an accessor
    pub fn get_container(&mut self) -> Option<&mut c_void> {
        unsafe {
            let container = eina_accessor_container_get(self.ptr);
            match container.is_null() {
                false => Some(&mut*container),
                true => None,
            }

        }
    }

    /// Iterates over container and executes callback on chosen elements
    pub fn exec_over<T>(&mut self, cb: EinaEachCb, start: c_uint, end: c_uint, fdata: &T) {
        unsafe {
            eina_accessor_over(self.ptr, cb, start, end, fdata as *const _ as *const c_void);
        }
    }

    /// Locks the container of the accessor
    pub fn lock(&mut self) -> bool {
        unsafe {
            match eina_accessor_lock(self.ptr) {
                EINA_TRUE => true,
                _ => false,
            }
        }
    }

    /// Unlock the container of the accessor
    ///
    /// None of the existing eina data structures are lockable
    pub fn unlock(&mut self) -> bool {
        unsafe {
            match eina_accessor_unlock(self.ptr) {
                EINA_TRUE => true,
                _ => false,
            }
        }
    }
}

impl Drop for EinaAccessor {
    fn drop(&mut self) {
        unsafe {
            eina_accessor_free(self.ptr);
        }
    }
}

impl Clone for EinaAccessor {
    fn clone(&self) -> Self {
        unsafe {
            EinaAccessor{ptr: eina_accessor_clone(self.ptr)}
        }
    }
}
