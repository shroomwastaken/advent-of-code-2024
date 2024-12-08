use std::collections::{HashMap, HashSet};

fn gather_input(test: bool) -> Vec<Vec<char>> {
	let mut res: Vec<Vec<char>> = vec![];
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		for line in f.split("\n") {
			res.push(line.chars().collect());
		}
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn part1(data: &Vec<Vec<char>>) -> usize {
	let mut poss: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
	for row in 0..data.len() {
		for col in 0..data[row].len() {
			let c: char = data[row][col];
			if c == '.' { continue; }
			if poss.contains_key(&c) { poss.get_mut(&c).unwrap().push((row as isize, col as isize)); }
			else { poss.insert(c, vec![(row as isize, col as isize)]); }
		}
	};

	let mut placed: HashSet<(isize, isize)> = HashSet::new();
	for key in poss.keys() {
		if poss[key].len() == 1 { continue; }
		for init in &poss[key] {
			for target in &poss[key] {
				if init == target { continue; }
				let diffrow: isize = init.0 - target.0;
				let diffcol: isize = init.1 - target.1;
				let antinode: (isize, isize) = (init.0 + diffrow, init.1 + diffcol);
				if antinode.0 < 0 || antinode.0 >= data.len() as isize ||
				antinode.1 < 0 || antinode.1 >= data[0].len() as isize {
					continue;
				}
				placed.insert(antinode);
			}
		}
	}

	return placed.len();
}

fn part2(data: &Vec<Vec<char>>) -> usize {
	let mut poss: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
	for row in 0..data.len() {
		for col in 0..data[row].len() {
			let c: char = data[row][col];
			if c == '.' { continue; }
			if poss.contains_key(&c) { poss.get_mut(&c).unwrap().push((row as isize, col as isize)); }
			else { poss.insert(c, vec![(row as isize, col as isize)]); }
		}
	};

	let mut placed: HashSet<(isize, isize)> = HashSet::new();
	for key in poss.keys() {
		if poss[key].len() == 1 { continue; }
		for init in &poss[key] {
			for target in &poss[key] {
				if init == target { continue; }
				let diffrow: isize = init.0 - target.0;
				let diffcol: isize = init.1 - target.1;
				let mut antinode: (isize, isize) = (init.0 + diffrow, init.1 + diffcol);
				loop {
					if antinode.0 < 0 || antinode.0 >= data.len() as isize ||
					antinode.1 < 0 || antinode.1 >= data[0].len() as isize {
						break;
					}
					placed.insert(antinode);
					antinode = (antinode.0 + diffrow, antinode.1 + diffcol);
				}
			}
			placed.insert(*init);
		}
	}

	return placed.len();
}

pub fn run() {
	use std::time::Instant;
	let data = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}