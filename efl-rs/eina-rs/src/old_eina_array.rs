use eina_ffi::*;

use libc::*;
use std::ptr;

trait EArray {
	fn new(step: c_uint) -> Option<EinaArray>;
	fn set_step(&mut self, sizeof_eina_array: c_uint, step: c_uint);
	fn clean(&mut self);
	fn flush(&mut self);
	fn remove<T>(&mut self, keep: extern "C" fn(data: *mut c_void, data: *mut c_void) -> EinaBool, gdata: &mut T) -> bool;
	fn push<T>(&mut self, data: &mut T) -> bool;
	fn pop(&mut self) -> Option<*mut c_void>;
	fn get_data(&self, idx: c_uint) -> Option<*mut c_void>;
	fn set_data<T>(&mut self, idx: c_uint, data: &mut T);
	fn get_count(&self) -> c_uint;
	fn count(&self) -> c_uint;
	fn new_iterator(&self) -> Option<EinaIterator>;
	fn new_accessor(&self) -> Option<EinaAccessor>;
}

impl EArray for EinaArray {
	/// Create a new array
	fn new(step: c_uint) -> Option<EinaArray> {
		unsafe {
			let array = eina_array_new(step);
			match array.is_null() {
				false => Some(*array),
				true => None,
			}
		}
	}
	/// Set the step of an array
	fn set_step(&mut self, sizeof_eina_array: c_uint, step: c_uint) {
		unsafe {
			eina_array_step_set(self as *mut EinaArray, sizeof_eina_array, step);
		}
				
	}

	/// Clean an array
	///
	/// Does not free space only sets cound to 0
	fn clean(&mut self) {
		unsafe {
			self.count = 0;
		}
	}

	/// Flush an array
	///
	/// Sets count and total members of array to 0, sets data member to NULL
	fn flush(&mut self) {
		unsafe {
			eina_array_flush(self as *mut EinaArray);
		}
	}

	/// Rebuild an array by specifying the data to keep
	fn remove<T>(&mut self, keep:extern "C" fn(data: *mut c_void, data: *mut c_void) -> EinaBool, gdata: &mut T) -> bool {
		unsafe {
			let result = eina_array_remove(self as *mut EinaArray, keep, gdata as *mut _ as *mut c_void);
			match result {
				EINA_TRUE => true,
				_ => false,
			}
		}
	}

	/// Append data to an array
	fn push<T>(&mut self, data: &mut T) -> bool {
		unsafe {
			// if(data.is_null())
			// 	return false
			if self.count +1 > self.total {
				if eina_array_grow(self as *mut EinaArray) == EINA_FALSE { 
					return false
				}
			}
			*self.data.offset(((self.count)+1) as isize) = data as *mut _ as *mut c_void;
			true
		}
	}

	/// Remove teh last data of an array
	fn pop(&mut self) -> Option<*mut c_void> {
		unsafe {
			let mut ret = ptr::null_mut();
			if self.count <= 0  {
				return None
			}

			ret = *self.data.offset((--(self.count)) as isize);
			match ret.is_null() {
				false => Some(ret),
				true => None,
			}
		}
	}
			
	/// Return the data at a given position in an array
	fn get_data(&self, idx: c_uint) -> Option<*mut c_void> {
		unsafe {
			let stuff = *self.data.offset(idx as isize);
			match stuff.is_null() {
				true => None,
				false => Some(stuff),
			}
		}
	}

	/// Set the data at a given position in an array
	fn set_data<T>(&mut self, idx: c_uint, data: &mut T) {
		unsafe {
			*self.data.offset(idx as isize) = data as *mut _ as *mut c_void;
		}
	}

	/// Return the number of elements in an array
	fn get_count(&self) -> c_uint {
		unsafe {
			self.count
		}
	}

	/// Return the number of elements in an array (again)
	fn count(&self) -> c_uint {
		unsafe {
			self.count
		}
	}

	/// Get a new iterator associated to an array.
	fn new_iterator(&self) -> Option<EinaIterator> {
		unsafe {
			let iter = eina_array_iterator_new(self as *const EinaArray);
			match iter.is_null() {
				true => None,
				false => Some(*iter),
			}
		}
	}

	/// Get a new accessor associated to an array
	fn new_accessor(&self) -> Option<EinaAccessor> {
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

