fn gather_input(test: bool) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
	let mut rules: Vec<(usize, usize)> = vec![];
	let mut pages: Vec<Vec<usize>> = vec![];
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		for line in f.split("\n") {
			if line == "" { break; }
			rules.push((
				line.split("|").nth(0).unwrap().parse::<usize>().unwrap(),
				line.split("|").nth(1).unwrap().parse::<usize>().unwrap(),
			));
		}
		for line in f.split("\n") {
			if line.contains("|") || line == "" { continue; }
			pages.push(line.split(",").map(|x| x.parse::<usize>().unwrap()).collect())
		}
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return (rules, pages);
}

fn part1(rules: &Vec<(usize, usize)>, pages: &Vec<Vec<usize>>) -> usize {
	return pages.iter()
		.filter(|x| {
			for y in rules {
				if !x.contains(&y.0) || !x.contains(&y.1) { continue; }
				if x.iter().position(|x| *x == y.0).unwrap() > x.iter().position(|x| *x == y.1).unwrap() { return false; }
			}
			return true;
		})
		.map(|x| x[x.len() / 2])
		.sum();
}

fn part2(rules: &Vec<(usize, usize)>, pages: &Vec<Vec<usize>>) -> usize {
	let check_arr = |x: &Vec<usize>| -> bool {
		for y in rules {
			if !x.contains(&y.0) || !x.contains(&y.1) { continue; }
			if x.iter().position(|x| *x == y.0).unwrap() > x.iter().position(|x| *x == y.1).unwrap() { return false; }
		}
		return true;
	};
	return pages.iter()
		.filter(|x| { return !check_arr(x); })
		.map(|x| {
			let mut cloned: Vec<usize> = x.clone();
			while !check_arr(&cloned) {
				for y in rules {
					if !cloned.contains(&y.0) || !cloned.contains(&y.1) { continue; }
					let a = cloned.iter().position(|x| *x == y.0).unwrap();
					let b = cloned.iter().position(|x| *x == y.1).unwrap();
					if a > b { cloned.swap(a, b); }
				}
			}
			return cloned[cloned.len() / 2];
		})
		.sum();
}

pub fn run() {
	use std::time::Instant;
	let (rules, pages): (Vec<(usize, usize)>, Vec<Vec<usize>>) = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&rules, &pages), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&rules, &pages), Instant::now().duration_since(start));
}