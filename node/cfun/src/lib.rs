#![cfg_attr(not(feature = "std"), no_std)]

extern crate libc;

#[link(name = "test", kind = "static")]
extern "C" {
    pub fn say_hello();
	pub fn double_int(input: libc::c_int) -> libc::c_int;
	pub fn double_uint(input: libc::c_uint) -> libc::c_uint;
}
