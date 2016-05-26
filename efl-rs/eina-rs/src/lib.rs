extern crate libc;
extern crate eina_sys as eina_ffi;

pub mod eina_accessor;
pub mod eina_iterator;
pub mod eina_array;
pub mod eina_list;
pub mod eina_hash;
pub mod eina_stringshare;
pub mod eina_main;




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }

	use eina_main::*;
	use eina_array::*;
	#[test]
	fn count_test() {
		let mut data = [0; 20];
		rs_eina_init();
		let arr = EinaArray::new(10).unwrap();
		for i in 0..20 {
			data[i] = i;
			arr.push(&mut data[i]);
		}
		arr.set_data(19, &mut data[0]);
		let get: &mut usize = arr.get_data(19).unwrap();
		println!("{}", get);
		let last1: &mut usize =  arr.pop().unwrap();
		println!("{}", last1);
		assert_eq!(arr.get_count(), 19);
		arr.clean();
		assert_eq!(arr.get_count(), 0);
		rs_eina_shutdown();
	}
}
