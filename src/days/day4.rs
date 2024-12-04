fn gather_input(test: bool) -> Vec<Vec<char>> {
	let mut res: Vec<Vec<char>> = vec![];
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		for line in f.split("\n") {
			res.push(line.to_string().chars().collect());
		}
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn part1(data: &Vec<Vec<char>>) -> usize {
	let mut res: usize = 0;
	// vertical
	for i in 0..data.len() - 3 {
		for j in 0..data[i].len() {
			let comb: String = format!("{}{}{}{}", data[i][j], data[i+1][j], data[i+2][j], data[i+3][j]);
			if comb == "XMAS" || comb == "SAMX" { res += 1; }
		}
	}
	// horizontal
	for i in 0..data.len() {
		for j in 0..data[i].len() - 3 {
			let comb: String = format!("{}{}{}{}", data[i][j], data[i][j+1], data[i][j+2], data[i][j+3]);
			if comb == "XMAS" || comb == "SAMX" { res += 1; }
		}
	}
	// diagonal \
	for i in 0..data.len() - 3 {
		for j in 0..data[i].len() - 3 {
			let comb: String = format!("{}{}{}{}", data[i][j], data[i+1][j+1], data[i+2][j+2], data[i+3][j+3]);
			if comb == "XMAS" || comb == "SAMX" { res += 1; }
		}
	}
	// diagonal /
	for i in 0..data.len() - 3 {
		for j in 3..data[i].len() {
			let comb: String = format!("{}{}{}{}", data[i][j], data[i+1][j-1], data[i+2][j-2], data[i+3][j-3]);
			if comb == "XMAS" || comb == "SAMX" { res += 1; }
		}
	}

	return res;
}

fn part2(data: &Vec<Vec<char>>) -> usize {
	let mut res: usize = 0;

	for i in 0..data.len() - 2 {
		for j in 0..data[i].len() - 2 {
			let comb: String = format!("{}{}{}{}{}", data[i][j], data[i][j + 2], data[i+2][j], data[i+2][j+2], data[i+1][j+1]);
			if ["MMSSA", "MSMSA", "SSMMA", "SMSMA"].contains(&comb.as_str()) { res += 1 }
		}
	}

	return res
}

pub fn run() {
	use std::time::Instant;
	let data: Vec<Vec<char>> = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}