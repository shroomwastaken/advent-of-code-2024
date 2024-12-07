fn gather_input(test: bool) -> Vec<(usize, Vec<usize>)> {
	let mut res: Vec<(usize, Vec<usize>)> = vec![];
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		for line in f.split("\n") {
			let r: usize = line.split(":").nth(0).unwrap().parse::<usize>().unwrap();
			let nums: Vec<usize> = line.split(" ")
				.skip(1)
				.map(|x| x.parse::<usize>().unwrap())
				.collect();
			res.push((r, nums));
		}
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn part1(data: &Vec<(usize, Vec<usize>)>) -> usize {
	return data.iter()
		.filter(|x| {
			// all possible combinations of + and * condensed into bits
			// where an unset bit is a + and a set bit is a *
			for op in 0usize..(1 << x.1.len() - 1) {
				let mut r: usize = x.1[0];
				for i in 1..x.1.len() {
					if (op & (1 << (i - 1))) == 0 { r += x.1[i]; }
					else { r *= x.1[i]; }
				}
				if r == x.0 { return true; }
			}
			return false;
		})
		.map(|x| x.0)
		.sum();
}

fn part2(data: &Vec<(usize, Vec<usize>)>) -> usize {
	return data.iter()
		.filter(|x| {
			// all possible combinations of +, * and || condensed into a base 3 number
			// where 0 is +, 1 is * and 2 is ||
			for op in 0..3usize.pow((x.1.len() - 1) as u32) {
				let mut r: usize = x.1[0];
				for i in 1..x.1.len() {
					// curop is the nth base3 digit of the number op
					let curop: usize = (op / 3usize.pow((x.1.len() - i - 1) as u32)) % 3;
					if curop == 0 { r += x.1[i]; }
					else if curop == 1 { r *= x.1[i]; }
					else {
						// log10().floor() + 1 is the amount of digits a number has
						r *= 10usize.pow(((x.1[i] as f32).log10().floor() + 1f32) as u32) as usize;
						r += x.1[i];
					}
				}
				if r == x.0 { return true; }
			}
			return false;
		})
		.map(|x| x.0)
		.sum();
}

pub fn run() {
	use std::time::Instant;
	let data: Vec<(usize, Vec<usize>)> = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}