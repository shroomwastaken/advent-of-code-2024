mod days;
use days::*;

fn main() {
	let args: Vec<String> = std::env::args().collect();

	if args.len() != 2 {
		println!("please provide the number of the day you want to get the answer for");
		return;
	}

	if let Ok(res) = args[1].parse::<usize>() {
		match res {
			1 => { day1::run(); }
			2 => { day2::run(); }
			3 => { day3::run(); }
			4 => { day4::run(); }
			5 => { day5::run(); }
			6 => { day6::run(); }
			7 => { day7::run(); }
			8 => { day8::run(); }
			9 => { day9::run(); }
			_ => {
				println!("bad day number");
			}
		}
	} else {
		println!("bad day number");
	}
}
