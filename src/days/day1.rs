use std::{iter::zip, process::exit};

fn gather_input(test: bool) -> Vec<(usize, usize)> {
	let mut res: Vec<(usize, usize)> = vec![];
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		for line in f.split("\n") {
			res.push((
				line.split(" ").next().unwrap().parse::<usize>().unwrap(),
				line.split(" ").last().unwrap().parse::<usize>().unwrap(),
			));
		}
	} else {
		println!("couldn't find input file!");
		exit(1);
	}
	return res;
}

fn part1() -> usize {
	let data: Vec<(usize, usize)> = gather_input(false);
	let mut l1: Vec<usize> = data.iter().map(|x| { x.0 }).collect();
	let mut l2: Vec<usize> = data.iter().map(|x| { x.1 }).collect();
	l1.sort();
	l2.sort();
	let res: usize = zip(l1, l2)
		.map(|x| { x.0.abs_diff(x.1) })
		.sum();
	return res;
}

fn part2() -> usize {
	let data: Vec<(usize, usize)> = gather_input(false);
	let l1: Vec<usize> = data.iter().map(|x| { x.0 }).collect();
	let l2: Vec<usize> = data.iter().map(|x| { x.1 }).collect();
	let res: usize = l1.iter()
		.map(|x| {
			x * l2.iter()
				.filter(|y| {x == *y})
				.count()
		})
		.sum();
	return res;
}

pub fn run() {
	println!("part 1 answer: {}", part1());
	println!("part 2 answer: {}", part2());
}