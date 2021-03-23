#![cfg_attr(not(feature = "std"), no_std)]

// extern crate libc;

#[link(name = "test", kind = "static")]
extern "C" {
    pub fn say_hello();
	pub fn double_int(input: i32) -> i32;
	pub fn double_uint(input: u32) -> u32;
}
