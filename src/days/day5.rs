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
	return 0;
}

pub fn run() {
	use std::time::Instant;
	let (rules, pages): (Vec<(usize, usize)>, Vec<Vec<usize>>) = gather_input(true);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&rules, &pages), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&rules, &pages), Instant::now().duration_since(start));
}