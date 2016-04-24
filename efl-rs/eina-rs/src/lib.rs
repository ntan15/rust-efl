extern crate libc;
extern crate eina_sys as eina_ffi;

use libc::*;

pub use eina_ffi::*;

pub mod eina_accessor;
pub mod eina_iterator;
pub mod eina_array;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
