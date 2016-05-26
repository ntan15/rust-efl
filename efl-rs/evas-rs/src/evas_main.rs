#![allow(dead_code)]
use libc::*;
use evas_ffi::*;
use eina_ffi::*;

/// Initialize Evas.
///
/// This function initializes Evas and increments a counter of the number of calls to it. It returns the new counter's value.
pub fn rs_evas_init() -> c_int {
    unsafe {
        evas_init()
    }
}

/// Shutdown Evas.
///
/// This function finalizes Evas, decrementing the counter of the number of calls to the function evas_init(). This nwe value for the counter is returned.
pub fn rs_evas_shutdown() {
    unsafe {
        evas_shutdown()
    }
}

/// Return if any allocation errors have occurred during the prior function
///
/// This function will return if any memory allocation errors occurred during, and what kind they were. The return value will be one of EVAS_ALLOC_ERROR_NONE, EVAS_ALLOC_ERROR_FATAL or EVAS_ALLOC_ERROR_RECOVERED with each meaning something different.

/// EVAS_ALLOC_ERROR_NONE means that no errors occurred at all and the function worked as expected.
/// 
/// EVAS_ALLOC_ERROR_FATAL means the function was completely unable to perform its job and will have exited as cleanly as possible. The programmer should consider this as a sign of very low memory and should try and safely recover from the prior functions failure (or try free up memory elsewhere and try again after more memory is freed).
/// 
/// EVAS_ALLOC_ERROR_RECOVERED means that an allocation error occurred, but was recovered from by evas finding memory of its own it has allocated and freeing what it sees as not really usefully allocated memory. What is freed may vary. Evas may reduce the resolution of images, free cached images or fonts, throw out pre-rendered data, reduce the complexity of change lists etc. Evas and the program will function as per normal after this, but this is a sign of low memory, and it is suggested that the program try and identify memory it doesn't need, and free it.
pub fn rs_evas_alloc_error() -> EvasAllocError {
    unsafe {
        evas_alloc_error()
    }
}

/// Get evas' internal asynchronous events read file descriptor
///
/// Evas' asynchronous events are meant to be dealt with internally, i. e., when building stuff to be glued together into the EFL infrastructure – a module, for example. The context which demands its use is when calculations need to be done out of the main thread, asynchronously, and some action must be performed after that.
///
/// An example of actual use of this API is for image asynchronous preload inside evas. If the canvas was instantiated through ecore-evas usage, ecore itself will take care of calling those events' processing.
/// 
/// This function returns the read file descriptor where to get the asynchronous events of the canvas. Naturally, other mainloops, apart from ecore, may make use of it. 
pub fn rs_evas_async_events_fd_get() -> c_int {
    unsafe {
        evas_async_events_fd_get()
    }
}


