fn gather_input(test: bool) -> Vec<Vec<Vec<char>>> {
	let mut res: Vec<Vec<Vec<char>>> = vec![];
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		for item in f.split("\n\n") {
			let mut n: Vec<Vec<char>> = vec![];
			for line in item.split("\n") {
				n.push(line.chars().collect());
			}
			res.push(n);
		}
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn get_col_height(col: usize, item: &Vec<Vec<char>>) -> usize {
	let mut res = 0;
		for i in 0..item.len() {
			if item[i][col] == '#' {
				res += 1;
			}
		}
	return res;
}

fn part1(data: &Vec<Vec<Vec<char>>>) -> usize {
	let locks: Vec<Vec<usize>> = data.iter().filter_map(|x| {
		if x[0].iter().all(|x| *x == '#') {
			let mut hvec: Vec<usize> = vec![];
			for c in 0..x[0].len() { hvec.push(get_col_height(c, x)); }
			return Some(hvec)
		}
		return None
	}).collect();
	let keys: Vec<Vec<usize>> = data.iter().filter_map(|x| {
		if x[0].iter().all(|x| *x == '.') {
			let mut hvec: Vec<usize> = vec![];
			for c in 0..x[0].len() { hvec.push(get_col_height(c, x)); }
			return Some(hvec)
		}
		return None
	}).collect();
	let mut res: usize = 0;
	for l in &locks {
		for k in &keys {
			let mut t: usize = 0;
			for i in 0..l.len() {
				if l[i] + k[i] <= data[0].len() { t += 1; }
			}
			if t == l.len() { res += 1; }
		}
	}
	return res;
}

fn part2(_: &Vec<Vec<Vec<char>>>) -> usize {
	println!("there is no part 2!");
	return 73;
}

pub fn run() {
	use std::time::Instant;
	let data: Vec<Vec<Vec<char>>> = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}