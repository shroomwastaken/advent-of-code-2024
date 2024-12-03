use regex::Regex;

fn gather_input(test: bool) -> String {
	let res: String;
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		res = f.replace("\n", "");
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn part1(data: &String) -> usize {
	let re: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
	let mut res: usize = 0;
	for m in re.captures_iter(data) {
		let (_, [num1, num2]) = m.extract();
		res += num1.parse::<usize>().unwrap() * num2.parse::<usize>().unwrap();
	}
	return res;
}

fn part2(data: &String) -> usize {
	use std::ops::Range;
	let re1: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
	let mulsress: Vec<usize> = re1.captures_iter(data).map(|m| {
		let (_, [num1, num2]) = m.extract();
		return num1.parse::<usize>().unwrap() * num2.parse::<usize>().unwrap();
	}).collect();
	let re2: Regex = Regex::new(r"do\(\)").unwrap();
	let re3: Regex = Regex::new(r"don't\(\)").unwrap();
	let mul_idxs: Vec<usize> = re1.find_iter(data).map(|m| m.end()).collect();
	let do_idxs: Vec<usize> = re2.find_iter(data).map(|m| m.end()).collect();
	let dont_idxs: Vec<usize> = re3.find_iter(data).map(|m| m.end()).collect();

	let mut range_scheme1: Vec<(usize, bool)> = vec![];
	for i in do_idxs {
		range_scheme1.push((i, true))
	}
	for i in dont_idxs {
		range_scheme1.push((i, false))
	}
	range_scheme1.sort_by(|a,b| {a.0.cmp(&b.0)});

	let mut range_scheme2: Vec<Range<usize>> = vec![];
	for i in 0..range_scheme1.len() - 1 {
		if range_scheme1[i].1 == false {
			range_scheme2.push(range_scheme1[i].0..range_scheme1[i + 1].0)
		}
	}
	let mut res: usize = 0;
	for idx in 0..mul_idxs.len() {
		let mut flag: bool = false;
		for r in &range_scheme2 {
			if r.contains(&mul_idxs[idx]) { flag = true; break; }
		}
		if !flag { res += mulsress[idx] }
	}
	return res;
}

pub fn run() {
	use std::time::Instant;
	let data: String = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}