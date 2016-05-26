use eina_ffi::*;

use libc::*;
use std::ptr;

trait EAccessor {
	fn get_data(&mut self, position: c_uint) -> Option<*mut c_void>;
	fn get_container(&mut self) -> Option<&mut c_void>;
	fn exec_over<T>(&mut self, cb: EinaEachCb, start: c_uint, end: c_uint, fdata: &T);
	fn lock(&mut self) -> bool; 
	fn unlock(&mut self) -> bool;
}

impl EAccessor for EinaAccessor {
	fn get_data(&mut self, position: c_uint) -> Option<*mut c_void> {
		unsafe {
			let mut data: *mut c_void = ptr::null_mut();
			let result = eina_accessor_data_get(self as *mut EinaAccessor, position, &mut data as *mut *mut _ as *mut *mut c_void);
			match result {
				EINA_TRUE => Some(data),
				_ => None,
			}
		}
	}

	fn get_container(&mut self) -> Option<&mut c_void> {
		unsafe {
			let container = eina_accessor_container_get(self as *mut EinaAccessor);
			match container.is_null() {
				false => Some(&mut *container),
				true => None,
			}
		}
	}

	fn exec_over<T>(&mut self, cb: EinaEachCb, start: c_uint, end: c_uint, fdata: &T) {
		unsafe {
			eina_accessor_over(self as *mut EinaAccessor, cb, start, end, fdata as *const _ as *const c_void);
		}
	}

	fn lock(&mut self) -> bool {
		unsafe {
			let result = eina_accessor_lock(self as *mut EinaAccessor);
			match result {
				EINA_TRUE => true,
				EINA_FALSE => false,
				_ => false,
			}
		}
	}
	
	fn unlock(&mut self) -> bool {
		unsafe {
			let result = eina_accessor_unlock(self as *mut EinaAccessor);
			match result {
				EINA_TRUE => true,
				EINA_FALSE => false,
				_ => false,
			}

		}

	}
}


