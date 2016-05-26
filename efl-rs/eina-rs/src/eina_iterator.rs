#![allow(dead_code)]
use eina_ffi::*;

use libc::*;
use std::ptr;

pub struct EinaIterator {
    pub ptr: *mut Eina_Iterator,
}

impl EinaIterator {
    /// Return the container of an iterator
    pub fn get_container(&mut self) -> Option<&mut c_void> {
        unsafe {
            let container = eina_iterator_container_get(self.ptr);
            match container.is_null() {
                false => Some(&mut*container),
                true => None,
            }

        }
    }

    /// Return the value of the current element and go to the next one
    pub fn next(&mut self) -> Option<&mut c_void> {
        unsafe {
            let mut data: *mut c_void = ptr::null_mut();
            match eina_iterator_next(self.ptr, &mut data as *mut *mut _ as *mut *mut c_void) {
                EINA_TRUE => Some(&mut *data),
                _ => None,
            }
        }
    }

    /// Iterate over the container and execute a callback on each element
    pub fn foreach<T>(&mut self, callback: EinaEachCb, fdata: &T) {
        unsafe {
            eina_iterator_foreach(self.ptr, callback, fdata as *const _ as *const c_void);
        }
    }
                
    /// Lock the container of the iterator
    ///
    /// Warning: None of the existing eina data structures are lockable
    pub fn lock(&mut self) -> bool {
        unsafe {
           match eina_iterator_lock(self.ptr) {
                EINA_TRUE => true,
                _ => false,
            }
        }
    }

    /// Unlock the container of the iterator
    ///
    /// Warning: None of the existing eina data structures are lockable
    pub fn unlock(&mut self) -> bool {
        unsafe {
           match eina_iterator_unlock(self.ptr) {
                EINA_TRUE => true,
                _ => false,
            }
        }
    }
}

impl Drop for EinaIterator {
    fn drop(&mut self) {
        unsafe {
            eina_iterator_free(self.ptr)
        }
    }
}
