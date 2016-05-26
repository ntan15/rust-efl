#![allow(dead_code)]
use eina_ffi::*;

use libc::*;
use super::eina_iterator::*;
use super::eina_accessor::*;

pub struct EinaArray {
    pub ptr: *mut Eina_Array,
}

impl EinaArray {
    /// Create a new Array.
    ///
    /// step:   Count of pointers to add when increasing array size
    /// This function creates a new array. When adding an element, the array allocates step elements. When that buffer is full, then adding another element will increase the buffer by step elements again.
    pub fn new(step: c_uint) -> Option<EinaArray> {
        unsafe {
            let arr = eina_array_new(step);
            match arr.is_null() {
                false => Some(EinaArray{ptr: arr}),
                true => None,
            }
        }
    }

    /// Set the step of an array
    ///
    /// sizeof_eina_array:  Should be value returned by sizeof(Eina_Array)
    /// step:               Count of pointers to add when increasing array size
    /// Warning:    Can only be called on uninitialized arrays
    pub fn set_step(&self, sizeof_eina_array: c_uint, step: c_uint) {
        unsafe {
            eina_array_step_set(self.ptr, sizeof_eina_array, step);
        }
    }
    
    /// Clean an array
    ///
    /// This function sets the count member of array to 0, however it doesn't free any space. This is particularly useful if you need to empty the array and add lots of elements quickly. For performance reasons, there is no check of array. If it is NULL or invalid, the program may crash. 
    pub fn clean(&self) {
        unsafe {
            (*self.ptr).count = 0;
        }
    }
    
    /// Flush an array
    ///
    /// This function sets the count and total members of array to 0, frees and set to NULL its data member. For performance reasons, there is no check of array. If it is NULL or invalid, the program may crash.
    pub fn flush(&self) {
        unsafe {
            eina_array_flush(self.ptr);
        }
    }

    /// Rebuild an array by specifying the data to keep.
    ///
    /// keep:   Functions which selects datat to keep
    /// gdata:  Data to pass to keep function
    /// This function rebuilds array be specifying the elements to keep with the function keep. No empty/invalid fields are left in the array. gdata is an additional data to pass to keep. For performance reasons, there is no check of array. If it is NULL or invalid, the program may crash.
    pub fn remove<T>(&self, keep: extern "C" fn(data: *mut c_void, gdata: *mut c_void) -> EinaBool, gdata: &mut T) -> bool {
        unsafe {
            match eina_array_remove(self.ptr, keep, gdata as *mut _ as *mut c_void) {
                EINA_TRUE => true,
                _ => false,
            }
        }
    }

    /// Append a data to an array
    pub fn push<T>(&self, data: &mut T) -> bool {
        unsafe {
			// if(data.is_null())
			// 	return false
            if (*self.ptr).count + 1 > (*self.ptr).total {
                if eina_array_grow(self.ptr) == EINA_FALSE {
                    return false
                }
            }
			*(*self.ptr).data.offset((((*self.ptr).count)+1) as isize) = data as *mut _ as *mut c_void;
            (*self.ptr).count += 1;
			true
        }
    }

    /// Remove the last data of an array
    pub fn pop<T>(&self) -> Option<&mut T> {
        unsafe {
			if (*self.ptr).count <= 0  {
				return None
			}

            let index = (*self.ptr).count - 1;
			let ret = *(*self.ptr).data.offset((index+1) as isize);
            (*self.ptr).count -= 1;
			match ret.is_null() {
				false => Some(&mut*(ret as *mut T)),
				true => None,
			}
            
        }
    }

    /// Return the data at a given position in an array
    pub fn get_data<T>(&self, idx: c_uint) -> Option<&mut T> {
        unsafe {
			let stuff = *(*self.ptr).data.offset((idx+1) as isize);
			match stuff.is_null() {
				true => None,
				false => Some(&mut*(stuff as *mut T)),
			}

        }
    }

    /// Set the data at a given position in an array
    pub fn set_data<T>(&self, idx: c_uint, data: &mut T) {
        unsafe {
			*(*self.ptr).data.offset((idx+1) as isize) = data as *mut _ as *mut c_void;
        }
    }

    /// Return the number of elemeents in an array.
    pub fn get_count(&self) -> c_uint {
        unsafe {
            (*self.ptr).count
        }
    }

    /// Return the number of elemeents in an array.
    pub fn count(&self) -> c_uint {
        unsafe {
            (*self.ptr).count
        }
    }

    /// Get a new iterator associated to an array
    pub fn new_iterator(&self) -> Option<EinaIterator> {
        unsafe {
            let iter = eina_array_iterator_new(self.ptr);
            match iter.is_null() {
                false => Some(EinaIterator{ptr: iter}),
                true => None,
            }
        }
    }

    /// Get a new accessor associated to an array
    pub fn new_accessor(&self) -> Option<EinaAccessor> {
        unsafe {
            let acces = eina_array_accessor_new(self.ptr);
            match acces.is_null() {
                false => Some(EinaAccessor{ptr: acces}),
                true => None,
            }
        }
    }
}

impl Drop for EinaArray {
    fn drop(&mut self) {
        unsafe {
            eina_array_free(self.ptr);
        }
    }
}
