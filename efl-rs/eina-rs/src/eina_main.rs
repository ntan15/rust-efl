#![allow(dead_code)]
use eina_ffi::*;
use libc::*;

/// Initialize the Eina library
///
/// This function sets up all the eina modules. It returns 0 on failure (that is, when one of the module fails to initialize), otherwise it returns the number of times it has already been called.
///
/// When Eina is not used anymore, call eina_shutdown() to shut down the Eina library. 
pub fn rs_eina_init() -> c_int {
    unsafe {
        eina_init()
    }
}

/// Shut down the Eina library
///
/// This function shuts down the Eina library. It returns 0 when it has been called the same number of times than eina_init(). In that case it shut down all the Eina modules.
///
/// Once this function succeeds (that is, 0 is returned), you must not call any of the Eina function anymore. You must call eina_init() again to use the Eina functions again. 
pub fn rs_eina_shutdown() -> c_int {
    unsafe {
        eina_shutdown() 
    }
}

/// Initializes the mutexes of the Eina library.
///
/// This function sets up all the mutexes in all eina modules. It returns 0 on failure (that is, when one of the module fails to initialize), otherwise it returns the number of times it has already been called.
/// 
/// When the mutexes are not used anymore, call eina_threads_shutdown() to shut down the mutexes.
/// 
/// This function should never be called outside of the main loop. 
pub fn rs_eina_threads_init() -> c_int {
    unsafe {
        eina_threads_init()
    }
}

/// Shut down mutexes in the Eina library.
/// 
/// Returns
///     0 when all mutexes are completely shut down, 1 or greater otherwise.
/// 
/// This function shuts down the mutexes in the Eina library. It returns 0 when it has been called the same number of times than eina_threads_init(). In that case it shut down all the mutexes.
/// 
/// Once this function succeeds (that is, 0 is returned), you must not call any of the Eina function in a thread anymore. You must call eina_threads_init() again to use the Eina functions in a thread again.
///
/// This function should never be called outside of the main loop. 
pub fn rs_eina_threads_shutdown() -> c_int {
    unsafe {
        eina_threads_shutdown()
    }
}

/// Check if you are calling this function from teh same thread Eina was initialized or not.
/// Most EFL function are not thread safe and all the call need to happen in the main loop. With this call you could know if you can call an EFL function or not. 
pub fn rs_eina_main_loop_is() -> bool {
    unsafe {
        match eina_main_loop_is() {
            EINA_TRUE => true,
            _ => false,
        }
    }
}

/// You should never use that function excpet if you really really know what your are doing.
/// 
/// If you are reading this documentation, that certainly means you don't know what is the purpose of this call and you should just not use it.
/// 
/// You should never use that function excpet if you really really know what your are doing. 
pub fn rs_eina_main_loop_define() {
    unsafe {
        eina_main_loop_define()
    }
}

