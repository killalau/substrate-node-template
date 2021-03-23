#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate cfun;

use sp_runtime_interface::runtime_interface;

#[runtime_interface]
pub trait CFunctions {
	fn c_double_uint(input: u32) -> u32 {
		unsafe {
			cfun::double_uint(input)
		}
	}
}
