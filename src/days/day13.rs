#[derive(Debug)]
struct Machine {
	ax: isize,
	ay: isize,
	bx: isize,
	by: isize,
	px: isize,
	py: isize,
}

fn gather_input(test: bool) -> Vec<Machine> {
	let mut res: Vec<Machine> = vec![];
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		for line in f.split("\n\n") {
			let mlines: Vec<&str> = line.split("\n").collect();
			let al: Vec<isize> = mlines[0][10..].split(", ").map(|x| x[2..].parse::<isize>().unwrap()).collect();
			let bl: Vec<isize> = mlines[1][10..].split(", ").map(|x| x[2..].parse::<isize>().unwrap()).collect();
			let prize: Vec<isize> = mlines[2][7..].split(", ").map(|x| x[2..].parse::<isize>().unwrap()).collect();
			res.push(
				Machine { ax: al[0], ay: al[1], bx: bl[0], by: bl[1], px: prize[0], py: prize[1] }
			);
		}
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn part1(data: &Vec<Machine>) -> isize {
	let mut tokens: isize = 0;
	for m in data {
		let denom: f64 = (m.ay * m.bx - m.ax * m.by) as f64;
		let aa: f64 = ((m.py * m.bx - m.px * m.by) as f64) / denom;
		let ab: f64 = ((m.px * m.ay - m.py * m.ax) as f64) / denom;
		if aa.fract() == 0.0 && ab.fract() == 0.0 {
			tokens += aa as isize * 3 + ab as isize
		}
	}
	return tokens;
}

fn part2(data: &Vec<Machine>) -> isize {
	let mut tokens: isize = 0;
	for m in data {
		let px: isize = m.px + 10000000000000;
		let py: isize = m.py + 10000000000000;
		let denom: f64 = (m.ay * m.bx - m.ax * m.by) as f64;
		let aa: f64 = ((py * m.bx - px * m.by) as f64) / denom;
		let ab: f64 = ((px * m.ay - py * m.ax) as f64) / denom;
		if aa.fract() == 0.0 && ab.fract() == 0.0 {
			tokens += aa as isize * 3 + ab as isize
		}
	}
	return tokens;
}

pub fn run() {
	use std::time::Instant;
	let data: Vec<Machine> = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}