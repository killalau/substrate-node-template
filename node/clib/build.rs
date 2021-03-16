extern crate dunce;
use std::{env, path::PathBuf};

fn main() {
	let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
	let library_dir = dunce::canonicalize(root.join("lib")).unwrap();
	println!("cargo:rustc-link-search=native={}", env::join_paths(&[library_dir]).unwrap().to_str().unwrap());
	println!("cargo:rustc-link-lib=static=test");
}
