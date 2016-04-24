use eina_ffi::*;

use libc::*;
use std::ptr;

trait EIterator {
	fn get_container(&mut self) -> Option<&mut c_void>;
	fn next(&mut self) -> Option<*mut c_void>;
	fn foreach<T>(&mut self, callback: EinaEachCb, fdata: &T);
	fn lock(&mut self) -> bool;
	fn unlock(&mut self) -> bool;
}
	
impl EIterator for EinaIterator {
	pub fn get_container(&mut self) -> Option<&mut c_void> {
		unsafe {
			let container = eina_iterator_container_get(self as *mut EinaIterator);
			match container.is_null() {
				false => Some(&mut *container),
				true => None,
			}
		}
	}

	pub fn next(&mut self) -> Option<*mut c_void> {
		unsafe {
			let data = ptr::null_mut();
			let result = eina_iterator_next(self as *mut EinaIterator, &mut data as *mut _ as *mut c_void);
			match result {
				EINA_TRUE => Some(data),
				_ => None,
			}
		}
	}

	pub fn foreach<T>(&mut self, callback: EinaEachCb, fdata: &T) {
		unsafe {
			eina_iterator_foreach(self as *mut EinaIterator, callback, fdata as *const _ as *const c_void);
		}
	}

	pub fn lock(&mut self) -> bool {
		unsafe {
			let result = eina_iterator_lock(self as *mut EinaIterator);
			match result {
				EINA_TRUE => true,
				_ => false,
			}
		}
	}

	pub fn unlock(&mut self) -> bool {
		unsafe {
			let result = eina_iterator_unlock(self as *mut EinaIterator);
			match result {
				EINA_TRUE => true,
				_ => false,
			}
		}
	}	

}

impl Drop for EinaIterator {
	pub fn drop(&mut self) {
		unsafe {
			eina_iterator_free(self as *mut EinaIterator);
		}
	}
}
