//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;

extern crate cfun;
fn main() -> sc_cli::Result<()> {
	unsafe {
		cfun::say_hello();
		let input1: i32 = 10;
		let input2: u32 = 11;
		println!("double_int({}) = {}", input1, cfun::double_int(input1));
		println!("double_uint({}) = {}", input2, cfun::double_uint(input2));
	}

	command::run()
}
