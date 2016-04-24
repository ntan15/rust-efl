use EinaAccessor;
use eina_ffi;
//use eina_ffi::*;

use libc::*;
use std::ptr;

trait EAccessor {
	fn get_data(&mut self, position: c_uint) -> Option<*mut c_void>;
	fn get_container(&mut self) -> Option<&mut c_void>;
	fn exec_over<T>(&mut self, cb: eina_ffi::EinaEachCb, start: c_uint, end: c_uint, fdata: &T);
	fn lock(&mut self) -> bool; 
	fn unlock(&mut self) -> bool;
}

impl EAccessor for EinaAccessor {
	pub fn get_data(&mut self, position: c_uint) -> Option<*mut c_void> {
		unsafe {
			let data: *mut c_void = ptr::null_mut();
			let result = eina_ffi::eina_accessor_data_get(self as *mut EinaAccessor, position, &mut data as *mut _ as *mut c_void);
			match result {
				EINA_TRUE => Some(data),
				_ => None,
			}
		}
	}

	pub fn get_container(&mut self) -> Option<&mut c_void> {
		unsafe {
			let container = eina_ffi::eina_accessor_container_get(self as *mut EinaAccessor);
			match container.is_null() {
				false => Some(&mut *container),
				true => None,
			}
		}
	}

	pub fn exec_over<T>(&mut self, cb: eina_ffi::EinaEachCb, start: c_uint, end: c_uint, fdata: &T) {
		unsafe {
			eina_ffi::eina_accessor_over(self as *mut EinaAccessor, cb, start, end, fdata as *const _ as *const c_void);
		}
	}

	pub fn lock(&mut self) -> bool {
		unsafe {
			let result = eina_ffi::eina_accessor_lock(self as *mut EinaAccessor);
			match result {
				EINA_TRUE => true,
				EINA_FALSE => false,
			}
		}
	}
	
	pub fn unlock(&mut self) -> bool {
		unsafe {
			let result = eina_ffi::eina_accessor_unlock(self as *mut EinaAccessor);
			match result {
				EINA_TRUE => true,
				EINA_FALSE => false,
			}

		}

	}
}

impl Drop for EinaAccessor {
	fn drop(&mut self) {
		unsafe {
			eina_ffi::eina_accessor_free(self);
		}
	}
}

impl Clone for EinaAccessor {
	pub fn clone(&mut self) -> Option<EinaAccessor> {
		unsafe {
			let new_clone = eina_ffi::eina_accessor_clone(self as *mut EinaAccessor);
			match new_clone.is_null() {
				true => None,
				false => Some(*new_clone),
			}
		}
	}
}
