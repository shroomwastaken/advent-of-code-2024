mod days;

use std::env::args;
use days::*;

fn main() {
	let args: Vec<String> = args().collect();

	if args.len() != 2 {
		println!("please provide the number of the day you want to get the answer for");
	}

	if let Ok(res) = args[1].parse::<usize>() {
		match res {
			1 => { day1::run(); }
			_ => {
				println!("bad day number");
			}
		}
	} else {
		println!("bad day number");
	}
}
