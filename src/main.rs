extern crate npuzzle_lib;

use std::env;
use npuzzle_lib::*;

fn main() {
	let args : Vec<String> = env::args().collect();
	match args.len() {
		1 => input_parser::parse(None),
		_ => input_parser::parse(Some(&args[1])),
	};
}
