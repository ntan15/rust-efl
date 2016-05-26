#![allow(dead_code)]
use eina_ffi::*;

use libc::*;
use super::eina_iterator::*;
use std::ffi::CStr;

pub struct EinaHash {
    pub ptr: *mut Eina_Hash,
}

impl EinaHash {
    /// Create a new hash table
    ///
    /// This function creates a new hash table using user-defined callbacks to manage the hash table. On failure, NULL is returned. If key_cmp_cb or key_hash_cb are NULL, NULL is returned. If buckets_power_size is smaller or equal than 2, or if it is greater or equal than 17, NULL is returned.

    /// The number of buckets created will be 2 ^ buckets_power_size. This means that if buckets_power_size is 5, there will be created 32 buckets. for a buckets_power_size of 8, there will be 256 buckets.
    pub fn new(key_length_cb: EinaKeyLength, key_cmp_cb: EinaKeyCmp, key_hash_cb: EinaKeyHash, data_free_cb: EinaFreeCb, buckets_power_size: c_int) -> Option<EinaHash> {
        unsafe {
            let hash = eina_hash_new(key_length_cb, key_cmp_cb, key_hash_cb, data_free_cb, buckets_power_size);
            match hash.is_null() {
                false => Some(EinaHash{ptr: hash}),
                true => None,
            }
        }
    }

    /// Redefine the callback that clean the data of a hash
    ///
    /// The argument received by data_free_cb will be that data of the item being removed.
    pub fn set_free_cb(&mut self, data_free_cb: EinaFreeCb) {
        unsafe {
            eina_hash_free_cb_set(self.ptr, data_free_cb);
        }
    }

    /// Create a new hash table using the djb2 algorithm
    ///
    /// This function creates a new hash table using the djb2 algorithm for table management and strcmp() to compare the keys. Values can then be looked up with pointers other than the original key pointer that was used to add values. On failure, this function returns NULL. 
    pub fn new_djb2(data_free_cb: EinaFreeCb) -> Option<EinaHash> {
        unsafe {
            let hash = eina_hash_string_djb2_new(data_free_cb);
            match hash.is_null() {
                false => Some(EinaHash{ptr: hash}),
                true => None,
            }
        }
    }

    /// Create a new hash table for use with strings
    ///
    /// This function creates a new hash table using the superfast algorithm for table management and strcmp() to compare the keys. Values can then be looked up with pointers other than the original key pointer that was used to add values. On failure, this function returns NULL.
    /// 
    /// NOTE: don't use this kind of hash when their is a possibility to remotely request and push data in it. This hash is subject to denial of service. 
    pub fn new_superfast(data_free_cb: EinaFreeCb) -> Option<EinaHash> {
        unsafe {
            let hash = eina_hash_string_superfast_new(data_free_cb);
            match hash.is_null() {
                false => Some(EinaHash{ptr: hash}),
                true => None,
            }
        }
    }

    /// Create a new hash table for use with strings with small bucket size
    ///
    /// This function creates a new hash table using the superfast algorithm for table management and strcmp() to compare the keys, but with a smaller bucket size (compared to eina_hash_string_superfast_new()) which will minimize the memory used by the returned hash table. Values can then be looked up with pointers other than the original key pointer that was used to add values. On failure, this function returns NULL. 
    pub fn new_small(data_free_cb: EinaFreeCb) -> Option<EinaHash> {
        unsafe {
            let hash = eina_hash_string_small_new(data_free_cb);
            match hash.is_null() {
                false => Some(EinaHash{ptr: hash}),
                true => None,
            }
        }
    }
    
    /// Create a new hash table for use with 32bit integers
    ///
    /// This function creates a new hash table where keys are 32bit integers. When adding or looking up in the hash table, pointers to 32bit integers must be passed. They can be addresses on the stack if you let the eina_hash copy the key. Values can then be looked up with pointers other than the original key pointer that was used to add values. This method is not suitable to match string keys as it would only match the first character. On failure, this function returns NULL. 
    pub fn new_int32(data_free_cb: EinaFreeCb) -> Option<EinaHash> {
        unsafe {
            let hash = eina_hash_int32_new(data_free_cb);
            match hash.is_null() {
                false => Some(EinaHash{ptr: hash}),
                true => None,
            }
        }
    }
    
    /// Create a new hash table for use with 64bit integers
    ///
    /// This function creates a new hash table where keys are 64bit integers. When adding or looking up in the hash table, pointers to 64bit integers must be passed. They can be addresses on the stack. Values can then be looked up with pointers other than the original key pointer that was used to add values. This method is not suitable to match string keys as it would only match the first character. On failure, this function returns NULL. 
    pub fn new_int64(data_free_cb: EinaFreeCb) -> Option<EinaHash> {
        unsafe {
            let hash = eina_hash_int64_new(data_free_cb);
            match hash.is_null() {
                false => Some(EinaHash{ptr: hash}),
                true => None,
            }
        }
    }
    
    /// Create a new hash table for use with pointers
    ///
    /// This function creates a new hash table using the int64/int32 algorithm for table management and dereferenced pointers to compare the keys. Values can then be looked up with pointers other than the original key pointer that was used to add values. This method may appear to be able to match string keys, actually it only matches the first character. On failure, this function returns NULL.
    pub fn new_pointer(data_free_cb: EinaFreeCb) -> Option<EinaHash> {
        unsafe {
            let hash = eina_hash_pointer_new(data_free_cb);
            match hash.is_null() {
                false => Some(EinaHash{ptr: hash}),
                true => None,
            }
        }
    }

    /// Create a nwe hash table optimized for stringshared values
    ///
    /// This function creates a new hash table optimized for stringshared values. Values CAN NOT be looked up with pointers not equal to the original key pointer that was used to add a value. On failure, this function returns NULL.
    pub fn new_stringshared(data_free_cb: EinaFreeCb) -> Option<EinaHash> {
        unsafe {
            let hash = eina_hash_stringshared_new(data_free_cb);
            match hash.is_null() {
                false => Some(EinaHash{ptr: hash}),
                true => None,
            }
        }
    }

    /// Add an entry to the given hash table
    ///
    /// This function adds key to hash. key is expected to be unique within the hash table. Key uniqueness varies depending on the type of hash: a stringshared Eina_Hash need to have unique pointers (which implies unique strings). All other string hash types require the strings themselves to be unique. Pointer, int32 and int64 hashes need to have these values as unique. Failure to use sufficient uniqueness will result in unexpected results when inserting data pointers accessed with eina_hash_find(), and removed with eina_hash_del(). Key strings are case sensitive. This function returns EINA_FALSE if an error occurred, EINA_TRUE otherwise. 
    pub fn add<K, D>(&mut self, key: &K, data: &D) -> bool {
        unsafe {
            match eina_hash_add(self.ptr, key as *const _ as *const c_void, data as *const _ as *const c_void) {
                EINA_TRUE => true,
                _ => false,
            }
        }
    }

    /// Add an entry to the given hash table without duplicating the string key
    ///
    /// This function adds key to hash. key is expected to be unique within the hash table. Key uniqueness varies depending on the type of hash: a stringshared Eina_Hash need have unique pointers (which implies unique strings). All other string hash types require the strings themselves to be unique. Pointer, int32 and int64 hashes need to have these values as unique. Failure to use sufficient uniqueness will result in unexpected results when inserting data pointers accessed with eina_hash_find(), and removed with eina_hash_del(). This function does not make a copy of key, so it must be a string constant or stored elsewhere ( in the object being added). Key strings are case sensitive. This function returns EINA_FALSE if an error occurred, EINA_TRUE otherwise. 
    pub fn direct_add<K, D>(&mut self, key: &K, data: &D) -> bool {
        unsafe {
            match eina_hash_direct_add(self.ptr, key as *const _ as *const c_void, data as *const _ as *const c_void) {
                EINA_TRUE => true,
                _ => false,
            }
        }
    }
    
    /// Remove the entry identified by a key or data from the given hash table
    ///
    /// This function removes the entry identified by key or data from hash. If a free function was given to the callback on creation, it will be called for the data being deleted. If hash is NULL, the functions returns immediately EINA_FALSE. If key is NULL, then data is used to find the a match to remove, otherwise key is used and data is not required and can be NULL. This function returns EINA_FALSE if an error occurred, EINA_TRUE otherwise.
    pub fn del<K, D>(&mut self, key: &K, data: &D) -> bool {
        unsafe {
            let success = eina_hash_del(self.ptr, key as *const _ as *const c_void, data as *const _ as *const c_void);
            match success {
                EINA_TRUE => true,
                _ => false,
            }
        }
    }
    
    /// Retrieve a specific entry in the given hash table
    ///
    /// This function retrieves the entry associated to key in hash. If hash is NULL, this function returns immediately NULL. This function returns the data pointer on success, NULL otherwise. 
    pub fn find<K, D>(&self, key: &K) -> Option<&mut D> {
        unsafe {
            let stuff = eina_hash_find(self.ptr, key as *const _ as *const c_void);
            match stuff.is_null() {
                false => Some(&mut *(stuff as *mut _ as *mut D)),
                true => None,
            }
        }
    }
    
    /// Modify the entry pointer at the specified key and return the old entry.
    ///
    /// This function modifies the data of key with data in hash. If no entry is found, nothing is added to hash. On success this function returns the old entry, otherwise it returns NULL. 
    pub fn modify<K, OD, ND>(&mut self, key: &K, data: &ND) -> Option<&mut OD> {
        unsafe {
            let old = eina_hash_modify(self.ptr, key as *const _ as *const c_void, data as *const _ as *const c_void);
            match old.is_null() {
                false => Some(&mut *(old as *mut _ as *mut OD)),
                true => None,
            }
        }
    }

    /// Modify the entry pointer at the specified key and return the old entry or add the entry if not found.
    ///
    /// This function modifies the data of key with data in hash. If no entry is found, data is added to hash with the key key. On success this function returns the old entry, otherwise it returns NULL. 
    pub fn set<K, OD, ND>(&mut self, key: &K, data: &ND) -> Option<&mut OD> {
        unsafe {
            let old = eina_hash_set(self.ptr, key as *const _ as *const c_void, data as *const _ as *const c_void);
            match old.is_null() {
                false => Some(&mut *(old as *mut _ as *mut OD)),
                true => None,
            }
        }
    }

    /// Change the key associated with a data without triggering the free callback
    ///
    /// This function allows for the move of data from one key to another, but does not call the Eina_Free_Cb associated with the hash table when destroying the old key.
    pub fn change_key<NK, OK>(&mut self, old_key: &OK, new_key: &NK) -> bool {
        unsafe {
            let success = eina_hash_move(self.ptr, old_key as *const _ as *const c_void, new_key as *const _ as *const c_void);
            match success {
                EINA_TRUE => true,
                _ => false,
            }
        }
    }

    /// Return the number of entries in the hash table
    pub fn population(&self) -> c_int {
        unsafe {
            eina_hash_population(self.ptr)
        }
    }

    /// Add an entry to the given hash table
    /// 
    /// This function adds key to hash. hash, key and data cannot be NULL, in that case EINA_FALSE is returned. key is expected to be a unique within the hash table. Otherwise, one cannot be sure which inserted data pointer will be accessed with eina_hash_find, and removed with eina_hash_del. Do not forget to count '\0' for string when setting the value of key_length. key_hash is expected to always match key. Otherwise, one cannot be sure to find it again with eina_hash_find_by_hash. Key strings are case sensitive. This function returns EINA_FALSE if an error occurred, EINA_TRUE otherwise.
    pub fn add_by_hash<K, D>(&mut self, key: &K, key_length: c_int, key_hash: c_int, data: &D) -> bool {
        unsafe {
            let result = eina_hash_add_by_hash(self.ptr, key as *const _ as *const c_void, key_length, key_hash, data as *const _ as *const c_void);
            match result {
                EINA_TRUE => true,
                _ => false,
            }
        }
    }

    /// Add an entry to the hash table and do not duplicate the string key
    ///
    /// This function adds key to hash. hash, key and data can be NULL, in that case EINA_FALSE is returned. key is expected to be unique within the hash table. Otherwise, one cannot be sure which inserted data pointer will be accessed with eina_hash_find, and removed with eina_hash_del. This function does not make a copy of key so it must be a string constant or stored elsewhere (in the object being added). Do not forget to count '\0' for string when setting the value of key_length. key_hash is expected to always match key. Otherwise, one cannot be sure to find it again with eina_hash_find_by_hash. Key strings are case sensitive. This function returns EINA_FALSE if an error occurred, EINA_TRUE otherwise.
    pub fn direct_add_by_hash<K, D>(&mut self, key: &K, key_length: c_int, key_hash: c_int, data: &D) -> bool {
        unsafe {
            let result = eina_hash_add_by_hash(self.ptr, key as *const _ as *const c_void, key_length, key_hash, data as *const _ as *const c_void);
            match result {
                EINA_TRUE => true,
                _ => false,
            }
        }
    }
    
    /// Remove the entry identified  by a key and a key hash from the table
    ///
    /// This function removes the entry identified by key and key_hash from hash. If a free function was given to the callback on creation, it will be called for the data being deleted. Do not forget to count '\0' for string when setting the value of key_length. If hash or key are NULL, the functions returns immediately EINA_FALSE. This function returns EINA_FALSE if an error occurred, EINA_TRUE otherwise.
    pub fn del_by_key_hash<K>(&mut self, key: &K, key_length: c_int, key_hash: c_int) -> bool {
        unsafe {
            let success = eina_hash_del_by_key_hash(self.ptr, key as *const _ as *const c_void, key_length, key_hash);
            match success {
                EINA_TRUE => true,
                _ => false,
            }
        }
    }

    /// Remove the entry identified by a key from the table
    /// This version will calculate key length and hash by functions provided at hash creation
    /// 
    /// This function removes the entry identified by key from hash. The key length and hash will be calculated automatically by using functiond provided to has creation function. If a free function was given to the callback on creation, it will be called for the data being deleted. If hash or key are NULL, the functions returns immediately EINA_FALSE. This function returns EINA_FALSE if an error occurred, EINA_TRUE otherwise.
    pub fn del_by_key<K>(&mut self, key: &K) -> bool {
        unsafe {
            let success = eina_hash_del_by_key(self.ptr, key as *const _ as *const c_void);
            match success {
                EINA_TRUE => true,
                _ => false,
            }
        }
    }

    /// Remove the entry identified by a data from the given hash table
    /// This version is slow since there is no quick access to nodes based on data
    ///
    /// This function removes the entry identified by data from hash. If a free function was given to the callback on creation, it will be called for the data being deleted. If hash or data are NULL, the functions returns immediately EINA_FALSE. This function returns EINA_FALSE if an error occurred, EINA_TRUE otherwise.
    pub fn del_by_data<D>(&mut self, key: &D) -> bool {
        unsafe {
            let success = eina_hash_del_by_data(self.ptr, key as *const _ as *const c_void);
            match success {
                EINA_TRUE => true,
                _ => false,
            }
        }
    }
    
    /// Remove the entry identified by a key and a key hash or a data from the table
    /// If key is NULL, then data is used to find a match to remove
    ///
    /// This function removes the entry identified by key and key_hash, or data, from hash. If a free function was given to the callback on creation, it will be called for the data being deleted. If hash is NULL, the functions returns immediately EINA_FALSE. If key is NULL, then key_length and key_hash are ignored and data is used to find a match to remove, otherwise key and key_hash are used and data is not required and can be NULL. Do not forget to count '\0' for string when setting the value of key_length. This function returns EINA_FALSE if an error occurred, EINA_TRUE otherwise.
    pub fn del_by_hash<K, D>(&mut self, key: &K, key_length: c_int, key_hash: c_int, data: &D) -> bool {
        unsafe {
            let success = eina_hash_del_by_hash(self.ptr, key as *const _ as *const c_void, key_length, key_hash, data as *const _ as *const c_void);
            match success {
                EINA_TRUE => true,
                _ => false,
            }
        }
    }

    /// Retrieve a specific entry in the given hash table
    ///
    /// This function retrieves the entry associated to key of length key_length in hash. key_hash is the hash that always match key. It is ignored if key is NULL. Do not forget to count '\0' for string when setting the value of key_length. If hash is NULL, this function returns immediately NULL. This function returns the data pointer on success, NULL otherwise. 
    pub fn find_by_hash<K, D>(&mut self, key: &K, key_length: c_int, key_hash: c_int) -> Option<&mut D> {
        unsafe {
            let stuff = eina_hash_find_by_hash(self.ptr, key as *const _ as *const c_void, key_length, key_hash);
            match stuff.is_null() {
                false => Some(&mut *(stuff as *mut _ as *mut D)),
                true => None,
            }
        }
    }

    /// Modify the entry pointer at the specified key and returns the old entry
    pub fn modify_by_hash<K, ND, OD>(&mut self, key: &K, key_length: c_int, key_hash: c_int, data: &ND) -> Option<&mut OD> {
        unsafe {
            let old = eina_hash_modify_by_hash(self.ptr, key as *const _ as *const c_void, key_length, key_hash, data as *const _ as *const c_void);
            match old.is_null() {
                false => Some(&mut *(old as *mut _ as *mut OD)),
                true => None,
            }
        }
    }

    /// Return a new iterator associated to hash keys
    /// This function returns a newly allocated iterator associated to hash. If hash is not populated, this function still returns a valid iterator that will always return false on eina_iterator_next(), thus keeping API sane.
    /// 
    /// If the memory can not be allocated, NULL is returned. Otherwise, a valid iterator is returned.
    /// Warning
    /// if the hash structure changes then the iterator becomes invalid! That is, if you add or remove items this iterator behavior is undefined and your program may crash! 
    pub fn new_key_iterator(&self) -> Option<EinaIterator> {
        unsafe {
            let itr =eina_hash_iterator_key_new(self.ptr);
            match itr.is_null() {
                false => Some(EinaIterator{ptr: itr}),
                true => None,
            }
        }
    }

    /// Return a new iterator associated to hash data
    /// This function returns a newly allocated iterator associated to hash. If hash is not populated, this function still returns a valid iterator that will always return false on eina_iterator_next(), thus keeping API sane.
    /// 
    /// If the memory can not be allocated, NULL is returned. Otherwise, a valid iterator is returned.
    /// Warning
    /// if the hash structure changes then the iterator becomes invalid! That is, if you add or remove items this iterator behavior is undefined and your program may crash! 
    pub fn new_data_iterator(&self) -> Option<EinaIterator> {
        unsafe {
            let itr =eina_hash_iterator_data_new(self.ptr);
            match itr.is_null() {
                false => Some(EinaIterator{ptr: itr}),
                true => None,
            }
        }
    }
    
    /// Return a new iterator associated to hash keys and data
    ///
    /// This function returns a newly allocated iterator associated to hash. If hash is not populated, this function still returns a valid iterator that will always return false on eina_iterator_next(), thus keeping API sane.
    /// 
    /// If the memory can not be allocated, NULL is returned. Otherwise, a valid iterator is returned.
    /// Warning
    /// if the hash structure changes then the iterator becomes invalid! That is, if you add or remove items this iterator behavior is undefined and your program may crash! 
    pub fn new_tuple_iterator(&self) -> Option<EinaIterator> {
        unsafe {
            let itr =eina_hash_iterator_tuple_new(self.ptr);
            match itr.is_null() {
                false => Some(EinaIterator{ptr: itr}),
                true => None,
            }
        }
    }
    
    /// Call a function on every member stored in the hash table
    ///
    /// This function goes through every entry in the hash table hash and calls the function func on each member. The function should not modify the hash table contents if it returns 1. If the hash table contents are modified by this function or the function wishes to stop processing it must return 0, otherwise return 1 to keep processing.
    pub fn foreach<D>(&self, func: EinaHashForeach, fdata: &D) {
        unsafe {
            eina_hash_foreach(self.ptr, func, fdata as *const _ as *const c_void);
        }
    }

    /// Append data to an EinaList inside a hash
    ///
    /// This function is identical to the sequence of calling eina_hash_find(), eina_list_append(), eina_hash_set(), but with one fewer required hash lookup. 
    pub fn list_append<K, D>(&mut self, key: &K, data: &D) {
        unsafe {
            eina_hash_list_append(self.ptr, key as *const _ as *const c_void, data as *const _ as *const c_void);
        }
    }

    /// Prepend data to an EinaList inside a hash
    ///
    /// This function is identical to the sequence of calling eina_hash_find(), eina_list_prepend(), eina_hash_set(), but with one fewer required hash lookup. 
    pub fn list_prepend<K, D>(&mut self, key: &K, data: &D) {
        unsafe {
            eina_hash_list_prepend(self.ptr, key as *const _ as *const c_void, data as *const _ as *const c_void);
        }
    }

    /// Remove data from an EinaList inside a hash
    ///
    /// This function is identical to the sequence of calling eina_hash_find(), eina_list_remove(), eina_hash_set(), but with one fewer required hash lookup. 
    pub fn list_remove<K,D>(&mut self, key: &K, data: &D) {
        unsafe {
            eina_hash_list_remove(self.ptr, key as *const _ as *const c_void , data as *const _ as *const c_void);
        }
    }

    /// Paul Hsieh (http://www.azillionmonkeys.com/qed/hash.html) hash functionused by WebCore (http://webkit.org/blog/8/hashtables-part-2/) 
    pub fn superfast(key: &CStr, len: c_int) -> c_int {
        unsafe {
            eina_hash_superfast(key.as_ptr(), len)
        }
    }

    /// Hash function first reported by Dan Bernstein many years ago in comp.lang.c. 
    pub fn djb2(key: &CStr) -> c_int {
        unsafe {
            let mut hash_num: c_uint = 5381 ^ eina_seed;
            let bytes = key.to_bytes();
            
            for i in 0..bytes.len() {
                hash_num = ((hash_num << 5) + hash_num) ^ bytes[i] as c_uint;
            }
            hash_num as c_int
        }
    }

    /// Hash function first reported by Dan Bernstein many years ago in comp.lang.c. 

    /// Hash function from http://www.concentric.net/~Ttwang/tech/inthash.htm. 
    pub fn int32(key_val: c_uint) -> c_int {
        let mut key = key_val;
        key = !key + (key << 15);
        key ^= key >> 12;
        key += key << 2;
        unsafe {
        key *= 2057 ^ eina_seed;
        }
        key ^= key >> 16;
        key as c_int
    }

    /// Hash function from http://www.concentric.net/~Ttwang/tech/inthash.htm. 
    pub fn int64(key_val: c_ulonglong) -> c_int {
        let mut key = key_val;
        key = !key + (key << 18);
        key ^= key >> 31;
        unsafe {
        key *= 21 ^ eina_seed as c_ulonglong;
        }
        key ^= key >> 11;
        key += key << 6;
        key ^= key >> 22;
        key as c_int           
    }

    /// Hash function from http://sites.google.com/site/murmurhash/. 
    pub fn murmur3(key: &str) -> c_int {
        unsafe {
        let c1: c_uint = 0xcc9e2d51 ^ eina_seed;
        let c2: c_uint = 0x1b873593 ^ eina_seed;        
        let mut chunk: c_uint = 0;
        let mut hash: c_uint = 0;
        let bytes = key.as_bytes();
        let nblocks = bytes.len() / 4;
        for i in 0..nblocks {
            chunk = get_32bit_word(&bytes[i*4..(i*4)+4]);
            chunk *= c1;
            chunk.rotate_left(15);
            chunk *= c2;
            hash ^= chunk;
            hash.rotate_left(13);
            hash = hash*5 + 0xe6546b64;
        }
        chunk = get_32bit_word(&bytes[nblocks*4..bytes.len()]);
        chunk *= c1;
        chunk.rotate_left(15);
        chunk *= c2;
        hash ^= chunk;
        hash ^= bytes.len() as c_uint;
        fmix32(hash);
        hash as c_int
        }
    }

    /// Hash function using crc-32 algorithm and and 0xEDB88320 polynomial. 
    pub fn crc(key: &CStr, len: c_int) -> c_int {
        unsafe {
            let seed: c_uint = 0xFFFFFFFF;
            _eina_crc(key.as_ptr(), len, seed, EINA_TRUE) as c_int
        }
    }
}

impl Drop for EinaHash {
    fn drop(&mut self) {
        unsafe {
            eina_hash_free_buckets(self.ptr);
            eina_hash_free(self.ptr)
        }
    }
}

// Helper stuff
fn fmix32(mut h: c_uint) -> c_uint {
    h ^= h >> 16;
    h *= 0x85ebca6b;
    h ^= h >> 13;
    h *= 0xc2b2ae35;
    h ^= h >> 16;
    h
}

fn get_32bit_word(bytes: &[u8]) -> c_uint {
    let word: c_uint = match bytes.len() {
        4 => {( ((bytes[3] as c_uint) << 24) + ((bytes[2] as c_uint) << 16) + ((bytes[1] as c_uint) << 8) + (bytes[0] as c_uint) ) as c_uint},
        3 => {( ((bytes[2] as c_uint) << 16) + ((bytes[1] as c_uint) << 8) + (bytes[0] as c_uint) ) as c_uint},
        2 => {( ((bytes[1] as c_uint) << 8) + (bytes[0] as c_uint) ) as c_uint},
        1 => {bytes[0] as c_uint},
        _ => 0,
    };
    word
} 

