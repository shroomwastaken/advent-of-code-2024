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
			10 => { day10::run(); }
			11 => { day11::run(); }
			12 => { day12::run(); }
			13 => { day13::run(); }
			14 => { day14::run(); }
			15 => { day15::run(); }
			16 => { day16::run(); }
			17 => { day17::run(); }
			18 => { day18::run(); }
			19 => { day19::run(); }
			20 => { day20::run(); }
			21 => { day21::run(); }
			22 => { day22::run(); }
			23 => { day23::run(); }
			24 => { day24::run(); }
			25 => { day25::run(); }
			_ => {
				println!("bad day number");
			}
		}
	} else {
		println!("bad day number");
	}
}
