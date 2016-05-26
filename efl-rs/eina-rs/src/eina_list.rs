#![allow(dead_code)]
use eina_ffi::*;
use super::eina_accessor::*;
use super::eina_iterator::*;
use libc::*;
use std::ptr;

pub struct EinaList {
    pub ptr: *mut Eina_List,
}

impl EinaList {
    /// Create new list
    pub fn new<T>(data: &T) -> Option<EinaList> {
        unsafe {
            let list = eina_list_append(ptr::null_mut(), data as *const _ as *const c_void);
            match list.is_null() {
                false => Some(EinaList{ptr: list}),
                true => None,
            }
        }
    }

    /// Append given data to list
    pub fn append<T>(&mut self, data: &T) {
        unsafe {
            self.ptr = eina_list_append(self.ptr, data as *const _ as *const c_void);
        }
    }

    /// Prepend given data to list
    pub fn prepend<T>(&mut self, data: &T) {
        unsafe {
            self.ptr = eina_list_prepend(self.ptr, data as *const _ as *const c_void);
        }
    }

    /// Insert the given data into the given linked list after the specified data.
    pub fn append_relative<T, D>(&mut self, data: &T, relative: &D) {
        unsafe {
            self.ptr = eina_list_append_relative(self.ptr, data as *const _ as *const c_void, relative as *const _ as *const c_void);   
        }
    }

    /// Append a list node to a linked list after the specified member.
    pub fn append_relative_list<T>(&mut self, data: &T, relative: *mut Eina_List) {
        unsafe {
            self.ptr = eina_list_append_relative_list(self.ptr, data as *const _ as *const c_void, relative);
        }
    }

    /// Prepend a data pointer to a linke list before the specified member
    pub fn prepend_relative<T, D>(&mut self, data: &T, relative: &D) {
        unsafe {
            self.ptr = eina_list_prepend_relative(self.ptr, data as *const _ as *const c_void, relative as *const _ as *const c_void);   
        }
    }

    /// Prepend a list node to a linked list before the specified member
    pub fn prepend_relative_list<T>(&mut self, data: &T, relative: *mut Eina_List) {
        unsafe {
            self.ptr = eina_list_prepend_relative_list(self.ptr, data as *const _ as *const c_void, relative);
        }
    }

    /// Insert a new node into a sorted list
    ///
    /// Assumes list is sorted 
    pub fn sorted_insert<T>(&mut self, func: EinaCompareCb, data: &T) {
        unsafe {
            self.ptr = eina_list_sorted_insert(self.ptr, func, data as *const _ as *const c_void);
        }
    }

    /// Remove first instance of specified data from list
    pub fn remove<T>(&mut self, data: &T) {
        unsafe {
            self.ptr = eina_list_remove(self.ptr, data as *const _ as *const c_void);
        }
    }

    /// Remove specified list node
    pub fn remove_list(&mut self, remove_list: *mut Eina_List) {
        unsafe {
            self.ptr = eina_list_remove_list(self.ptr, remove_list);
        }
    }

    /// Move specified data to head of list
    pub fn promote_list(&mut self, move_list: *mut Eina_List) {
        unsafe {
            self.ptr = eina_list_promote_list(self.ptr, move_list);
        }
    }

    /// Move specified data to tail of list
    pub fn demote_list(&mut self, move_list: *mut Eina_List) {
        unsafe {
            self.ptr = eina_list_demote_list(self.ptr, move_list);
        }
    }
    
    /// Find a member of a list and return the member
    pub fn find_data<T>(&mut self, data: &T) -> Option<&mut T> {
        unsafe {
            let d = eina_list_data_find(self.ptr, data as *const _ as *const c_void);
            match d.is_null() {
                false => Some(&mut *(d as *mut _ as *mut T)),
                true => None,
            }
        }
    }

    /// Find a member of a list and return the list node contianing that member
    pub fn find_list<T>(&mut self, data: &T) -> Option<EinaList> {
        unsafe {
            let lst = eina_list_data_find_list(self.ptr, data as *const _ as *const c_void);
            match lst.is_null() {
                false => Some(EinaList{ptr: lst}),
                true => None,
            }
        }
    }

    /// Move a data pointer from one list to another
    pub fn move_data_to<T>(&mut self, dst: &mut EinaList, data: &mut T) -> bool {
        unsafe {
            let result = eina_list_move(&mut dst.ptr as *mut *mut Eina_List, &mut self.ptr as *mut *mut Eina_List, data as *mut _ as *mut c_void);
            match result {
                EINA_TRUE => true,
                _ => false,
            }
        }
    }

    /// Move a list node from one list to another
    pub fn move_list_to(&mut self, dst: &mut EinaList, data: &mut EinaList) -> bool {
        unsafe {
            let result = eina_list_move_list(&mut dst.ptr as *mut *mut Eina_List, &mut self.ptr as *mut *mut Eina_List, data.ptr);
            match result {
                EINA_TRUE => true,
                _ => false,
            }
        }
    }

    /// Get the nth member's data pointer in a list
    pub fn nth_data<T>(&mut self, n: c_uint) -> Option<&mut T> {
        unsafe {
            let data = eina_list_nth(self.ptr, n);
            match data.is_null() {
                false => Some(&mut *(data as *mut _ as *mut T)),
                true => None,
            }
        }
    }

    /// Get the nth member's list node in a list
    pub fn nth_list(&mut self, n: c_uint) -> Option<EinaList> {
        unsafe {
            let lst = eina_list_nth_list(self.ptr, n);
            match lst.is_null() {
                false => Some(EinaList{ptr: lst}),
                true => None,
            }
        }
    }

    /// Reverse all the elements in the list
    pub fn reverse(&mut self) {
        unsafe {
            self.ptr = eina_list_reverse(self.ptr);
        }
    }

    /// Clone (copy) all the elements in the list in reverse order
    pub fn reverse_clone(&self) -> Option<EinaList> {
        unsafe {
            let lst = eina_list_reverse_clone(self.ptr);
            match lst.is_null() {
                false => Some(EinaList{ptr: lst}),
                true => None,
            }
        }
    }

    /// Clone (copy) all the elements in the list
    pub fn clone(&self) -> Option<EinaList> {
        unsafe {
            let lst = eina_list_clone(self.ptr);
            match lst.is_null() {
                false => Some(EinaList{ptr: lst}),
                true => None,
            }
        }
    }
    
    /// Sort a list according to the ordering func will return
    ///
    /// limit:  max number of list elements to sort
    pub fn sort(&mut self, limit: c_uint, func: EinaCompareCb) {
        unsafe {
            self.ptr = eina_list_sort(self.ptr, limit, func);
        }
    }

    /// Shuffle list
    pub fn shuffle(&mut self, func: EinaRandomCb) {
        unsafe {
            self.ptr = eina_list_shuffle(self.ptr, func);
        }
    }

    /// Merge two lists.
    ///
    /// This function puts right at the end of left and returns the head.
    /// 
    /// Both left and right do not exist anymore after the merge.
    pub fn merge(left: EinaList, right: EinaList) -> Option<EinaList>{
        unsafe {
            let merged = eina_list_merge(left.ptr, right.ptr);
            match merged.is_null() {
                false => Some(EinaList{ptr: merged}),
                true => None,
            }
        }
    }

    /// Merge two sorted lists according to the ordering func will return
    ///
    /// This function compares the head of left and right, and choose the smallest one to be head of the returned list. It will continue this process for all entry of both list.
    ///
    /// Both left and right lists are not valid anymore after the merge and should not be used. If func is NULL, it will return NULL.
    pub fn sorted_merge(left: EinaList, right: EinaList, func: EinaCompareCb) -> Option<EinaList> {
        unsafe {
            let sorted = eina_list_sorted_merge(left.ptr, right.ptr, func);
            match sorted.is_null() {
                false => Some(EinaList{ptr: sorted}),
                true => None,
            }
        }
    }

    /// Split a list into 2 lists
    ///
    /// This function splits list into two lists ( left and right ) after the node relative. Relative will become the last node of the left list. If list or right are NULL list is returns. If relative is NULL right is set to list and NULL is returns. If relative is the last node of list list is returns and right is set to NULL.
    /// 
    /// list does not exist anymore after the split.
    pub fn split(&mut self, relative: EinaList) -> [Option<EinaList>; 2] {
        unsafe {
            let right: *mut *mut Eina_List = ptr::null_mut();
            let left = eina_list_split_list(self.ptr, relative.ptr, right);
            let mut result = [None, None];
            result[0] = match left.is_null() {
                false => Some(EinaList{ptr: left}),
                true => None,
            };
            result[1] = match (*right).is_null() {
                false => Some(EinaList{ptr: *right}),
                true => None,
            };
            result
        }

    }

    /// Returns node nearest to data is in the sorted list.
    ///
    /// This function searches for a node containing data as its data in list, if such a node exists it will be returned and result_cmp will be 0. If the data of no node in list is equal to data, the node with the nearest value to that will be returned and result_cmp will be the return value of func with data and the returned node's data as arguments.

    /// This function is useful for inserting an element in the list only in case it isn't already present in the list, the naive way of doing this would be: 
    ///
    /// This function searches for a node containing data as its data in list, if such a node exists it will be returned and result_cmp will be 0. If the data of no node in list is equal to data, the node with the nearest value to that will be returned and result_cmp will be the return value of func with data and the returned node's data as arguments.
    pub fn near_sorted_list<T>(&mut self, func: EinaCompareCb, data: &T) -> (c_int, Option<EinaList>) {
        let mut cmp_result = 0;
        unsafe {
            let result = eina_list_search_sorted_near_list(self.ptr, func, data as *const _ as *const c_void, &mut cmp_result as *mut c_int);
            match result.is_null() {
                true => (cmp_result, None),
                false => (cmp_result, Some(EinaList{ptr: result})),
            }
        }
    
    }

    /// Returns node if data is in the sorted list
    ///
    /// This can be used to check if some value is inside the list and get the container node in this case. It should be used when list is known to be sorted as it will do binary search for results.
    pub fn search_sorted_list<T>(&self, func: EinaCompareCb, data: &T) -> Option<EinaList> {
        unsafe {
            let node = eina_list_search_sorted_list(self.ptr, func, data as *const _ as *const c_void);
            match node.is_null() {
                false => Some(EinaList{ptr: node}),
                true => None,
            }
        }
    }

    /// Returns node data if it is in the sorted list
    ///
    /// This can be used to check if some value is inside the list and get the existing instance in this case. It should be used when list is known to be sorted as it will do binary search for results.
    pub fn search_sorted<T>(&self, func: EinaCompareCb, data: &T) -> Option<&mut T> {
        unsafe {
            let stuff = eina_list_search_sorted(self.ptr, func, data as *const _ as *const c_void);
            match stuff.is_null() {
                false => Some(&mut *(stuff as *mut _ as *mut T)),
                true => None,
            }
        }
    }

    /// Returns node if data is in the unsorted list
    ///
    /// This can be used to check if some value is inside the list and get the container node in this case.
    pub fn search_unsorted_list<T>(&self, func: EinaCompareCb, data: &T) -> Option<EinaList> {
        unsafe {
            let node = eina_list_search_unsorted_list(self.ptr, func, data as *const _ as *const c_void);
            match node.is_null() {
                false => Some(EinaList{ptr: node}),
                true => None,
            }
        }
    
    }

    /// Returns node data if it is in the unsorted list
    pub fn search_unsorted<T>(&self, func: EinaCompareCb, data: &T) -> Option<&mut T> {
        unsafe {
            let stuff = eina_list_search_unsorted(self.ptr, func, data as *const _ as *const c_void);
            match stuff.is_null() {
                false => Some(&mut *(stuff as *mut _ as *mut T)),
                true => None,
            }
        }
            
    }

    /// Get the last list node in the list
    pub fn last(&self) -> Option<EinaList> {
        unsafe {
            match self.ptr.is_null() {
                true => None,
                false => Some(EinaList{ptr: (*(*self.ptr).accounting).last}),
            }
        }
    }

    /// Get the next listnode after the specified node
    pub fn next(&self) -> Option<EinaList> {
        unsafe {
            match self.ptr.is_null() {
                true => None,
                false => Some(EinaList{ptr: (*self.ptr).next}),
            }
        }

    }

    /// Get the previous list node before the specified list node
    pub fn prev(&self) -> Option<EinaList> {
        unsafe {
            match self.ptr.is_null() {
                true => None,
                false => Some(EinaList{ptr: (*self.ptr).prev}),
            }
        }

    }

    /// Get the list node data member
    pub fn get_data<T>(&self) -> Option<&mut T> {
        unsafe {
            match self.ptr.is_null() {
                true => None,
                false => Some(&mut *((*self.ptr).data as *mut T)),
            }
        }
    }

    /// Set the list node data member
    pub fn set_data<O, N>(&mut self, data: &mut N) -> Option<&mut O> {
        unsafe {
            if self.ptr.is_null() {
                return None;
            }
            let tmp = (*self.ptr).data;
            (*self.ptr).data = data as *mut _ as *mut c_void;
            return Some(&mut *(tmp as *mut O));
        }

    }

    /// Get the count of the number of items in a list
    pub fn count(&self) -> c_uint {
        unsafe {
            match self.ptr.is_null() {
                true => 0,
                false => (*(*self.ptr).accounting).count,
            }
        }
    }

    /// Return the last list node's data
    pub fn get_last_data<T>(&self) -> Option<&mut T> {
        unsafe {
            if self.ptr.is_null() {
                return None
            }
            let lst = (*(*self.ptr).accounting).last;
            if lst.is_null() {
                return None
            }
            let dat = (*lst).data as *mut _ as *mut T;
            if dat.is_null() {
                return None
            }
            Some(&mut *dat)   
        }
    }

    /// Return a new iterator associated to a list
    pub fn new_iterator(&self) -> Option<EinaIterator> {
        unsafe {
            let iter = eina_list_iterator_new(self.ptr);
            match iter.is_null() {
                false => Some(EinaIterator{ptr: iter}),
                true => None,
            }
        }
    }

    /// Return a new reversed iterator associated to a list
    pub fn new_reverse_iterator(&self) -> Option<EinaIterator> {
        unsafe {
            let iter = eina_list_iterator_reversed_new(self.ptr);
            match iter.is_null() {
                false => Some(EinaIterator{ptr: iter}),
                true => None,
            }
        }
    }

    /// Return a new accessor associated to a list
    pub fn new_accessor(&self) -> Option<EinaAccessor> {
        unsafe {
            let acces = eina_list_accessor_new(self.ptr);
            match acces.is_null() {
                false => Some(EinaAccessor{ptr: acces}),
                true => None,
            }
        }
    }

    /// Find the member of the list and return the index
    pub fn data_idx<T>(&self, data: &mut T) -> c_int {
        unsafe {
            eina_list_data_idx(self.ptr, data as *mut _ as *mut c_void)
        }
    }
}

impl Drop for EinaList {
    fn drop(&mut self) {
        unsafe {
            eina_list_free(self.ptr);
        }
    }
}

