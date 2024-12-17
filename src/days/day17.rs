fn gather_input(test: bool) -> (usize, usize, usize, Vec<usize>) {
	let mut res: (usize, usize, usize, Vec<usize>) = (0, 0, 0, vec![]);
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		let mut lines = f.split("\n");
		res.0 = lines.next().unwrap()[12..].parse::<usize>().unwrap();
		res.1 = lines.next().unwrap()[12..].parse::<usize>().unwrap();
		res.2 = lines.next().unwrap()[12..].parse::<usize>().unwrap();
		lines.next();
		res.3 = lines.next().unwrap()[9..].split(",").map(|x| x.parse::<usize>().unwrap()).collect();
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn emulate(data: &(usize, usize, usize, Vec<usize>)) -> String {
	fn combo_eval(x: usize, regs: &(usize, usize, usize)) -> usize {
		return match x {
			0 => { 0 }, 1 => { 1 }, 2 => { 2 }, 3 => { 3 }
			4 => { regs.0 }, 5 => { regs.1 }, 6 => { regs.2 },
			_ => { panic!("invalid combo operand") },
		}
	}

	let mut regs: (usize, usize, usize) = (data.0, data.1, data.2);
	let mut pc: usize = 0;
	let mut output: Vec<usize> = vec![];

	while pc < data.3.len() {
		match data.3[pc] {
			0 => {
				let numerator: usize = regs.0;
				let denominator: usize = 2usize.pow(combo_eval(data.3[pc + 1], &regs) as u32);
				regs.0 = numerator / denominator;
				pc += 2;
			}
			1 => { regs.1 = regs.1 ^ data.3[pc + 1]; pc += 2; }
			2 => { regs.1 = combo_eval(data.3[pc + 1], &regs) % 8; pc += 2; }
			3 => { if regs.0 != 0 { pc = data.3[pc + 1]; } else { pc += 2; } }
			4 => { regs.1 = regs.1 ^ regs.2; pc += 2; }
			5 => { output.push(combo_eval(data.3[pc + 1], &regs) % 8); pc += 2; }
			6 => {
				let numerator: usize = regs.0;
				let denominator: usize = 2usize.pow(combo_eval(data.3[pc + 1], &regs) as u32);
				regs.1 = numerator / denominator;
				pc += 2;
			}
			7 => {
				let numerator: usize = regs.0;
				let denominator: usize = 2usize.pow(combo_eval(data.3[pc + 1], &regs) as u32);
				regs.2 = numerator / denominator;
				pc += 2;
			}
			_ => { panic!("invalid opcode!") }
		}
	}
	return output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
}

fn part1(data: &(usize, usize, usize, Vec<usize>)) -> String {
	return emulate(data);
}

fn part2(data: &(usize, usize, usize, Vec<usize>)) -> usize {
	// this solution only works for my input due to the nature of this puzzle
	let mut targets: Vec<(usize, usize)> = vec![(data.3[data.3.len() - 1], 1)];
	while !targets.iter().map(|x| x.1).all(|x| x == 17) {
		let mut new_targets = vec![];
		for target in &targets {
			for a in (target.0 << 3)..((target.0 << 3) + 0b1000) {
				let mut b: usize = a % 8;
				b = b ^ 3;
				let c: usize = a >> b;
				b = b ^ 5;
				b = b ^ c;
				if b % 8 == data.3[data.3.len() - target.1] {
					new_targets.push((a, target.1 + 1));
				}
			}
		}
		targets = new_targets.clone();
	}
	println!();
	return targets.iter().map(|x| x.0).min().unwrap();
}

pub fn run() {
	use std::time::Instant;
	let data: (usize, usize, usize, Vec<usize>) = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}