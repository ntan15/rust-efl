use eina_ffi::*;

use libc::*;
use std::ptr;

impl EinaArray {
	/// Create a new array
	pub fn new(step: c_uint) -> Option<EinaArray> {
		unsafe {
			let array = eina_array_new(step);
			match array.is_null() {
				false => Some(*array),
				true => None,
			}
		}
	}
	/// Set the step of an array
	pub fn set_step(&mut self, sizeof_eina_array: c_uint, step: c_uint) {
		unsafe {
			eina_array_step_set(self as *mut EinaArray, sizeof_eina_array, step);
		}
				
	}

	/// Clean an array
	///
	/// Does not free space only sets cound to 0
	pub fn clean(&mut self) {
		unsafe {
			*self.count = 0;
		}
	}

	/// Flush an array
	///
	/// Sets count and total members of array to 0, sets data member to NULL
	pub fn flush(&mut self) {
		unsafe {
			eina_array_flush(self as *mut EinaArray);
		}
	}

	/// Rebuild an array by specifying the data to keep
	pub fn remove<T>(&mut self, keep: extern "C" fn(data: *mut c_void, data: *mut c_void) -> EinaBool, gdata: &mut T) -> bool {
		unsafe {
			let result = eina_array_remove(self as *mut EinaArray, keep, gdata as *mut _ as *mut c_void);
			match result {
				EINA_TRUE => true,
				_ => false,
			}
		}
	}

	/// Append data to an array
	pub fn push<T>(&mut self, data: &T) -> bool {
		unsafe {
			// if(data.is_null())
			// 	return false
			if *self.count +1 > *self.total {
				if eina_array_grow(self as *mut EinaArray) == EINA_FALSE { 
					return false
				}
			}
			*self.data[*self.count+=1] = data as *const _ as *const c_void;
		}
	}

	/// Remove teh last data of an array
	pub fn pop(&mut self) -> Option<*mut c_void> {
		unsafe {
			let ret = ptr::null_mut();
			if *self.count <= 0  {
				return None
			}

			ret = *self.data[--(*self.count)];
			match ret.is_null() {
				false => Some(ret),
				true => None,
			}
		}
	}
			
	/// Return the data at a given position in an array
	pub fn get_data(&self, idx: c_uint) -> Option<*mut c_void> {
		unsafe {
			let stuff = *self.data[idx];
			match stuff.is_null() {
				true => None,
				false => Some(stuff),
			}
		}
	}

	/// Set the data at a given position in an array
	pub fn set_data<T>(&mut self, idx: c_uint, data: &mut T) {
		unsafe {
			*self.data[idx] = data as *mut _ as *mut c_void;
		}
	}

	/// Return the number of elements in an array
	pub fn get_count(&self) -> c_uint {
		unsafe {
			*self.count
		}
	}

	/// Return the number of elements in an array (again)
	pub fn count(&self) -> c_uint {
		unsafe {
			*self.count
		}
	}

	/// Get a new iterator associated to an array.
	pub fn new_iterator(&self) -> Option<EinaIterator> {
		unsafe {
			let iter = eina_array_iterator_new(self as *const EinaArray);
			match iter.is_null() {
				true => None,
				false => Some(*iter),
			}
		}
	}

	/// Get a new accessor associated to an array
	pub fn new_accessor(&self) -> Option<EinaIterator> {
		unsafe {
			let access = eina_array_accessor_new(self as *const EinaArray);
			match access.is_null() {
				true => None,
				false => Some(*access),
			}
		}
	}

//	/// Provide a safe way to iterate over an array


}

impl Drop for EinaArray {
	fn drop(&mut self) {
		unsafe {
			eina_array_free(self as *mut EinaArray);
		}
	}
}
