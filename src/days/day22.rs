use std::collections::HashMap;

fn gather_input(test: bool) -> Vec<usize> {
	let res: Vec<usize>;
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		res = f.split("\n").map(|x| x.parse::<usize>().unwrap()).collect();
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn part1(data: &Vec<usize>) -> usize {
	return data.iter()
		.map(|x| {
			let mut nx: usize = *x;
			for _ in 0..2000 {
				let step1: usize = (nx ^ (nx << 6)) & 16777215;
				let step2: usize = (step1 ^ (step1 >> 5)) & 16777215;
				let step3: usize = (step2 ^ (step2 << 11)) & 16777215;
				nx = step3;
			}
			return nx;
		}).sum();
}

fn part2(data: &Vec<usize>) -> usize {
	let secret_digits: Vec<Vec<u8>> = data.iter()
		.map(|x| {
			let mut resvec: Vec<u8> = vec![(*x % 10) as u8,];
			let mut nx: usize = *x;
			for _ in 0..2000 {
				let step1: usize = (nx ^ (nx << 6)) & 16777215;
				let step2: usize = (step1 ^ (step1 >> 5)) & 16777215;
				let step3: usize = (step2 ^ (step2 << 11)) & 16777215;
				nx = step3;
				resvec.push((nx % 10) as u8);
			}
			return resvec;
		}).collect();
	let diffs: Vec<Vec<i8>> = secret_digits.iter()
		.map(|x| {
			let mut diff: Vec<i8> = vec![];
			for i in 1..x.len() {
				diff.push(x[i] as i8 - x[i - 1] as i8);
			}
			return diff;
		}).collect();
	let occurences: Vec<HashMap<[i8; 4], u8>> = diffs.iter()
		.enumerate()
		.map(|(i, x)| {
			let mut occ: HashMap<[i8; 4], u8> = HashMap::new();
			for s in 0..x.len() - 3 {
				if !occ.contains_key(&x[s..s+4]) {
					occ.insert(x[s..s+4].try_into().unwrap(), secret_digits[i][s+4]);
				}
			}
			return occ;
		}).collect();
	let mut possible_answers: HashMap<[i8; 4], usize> = HashMap::new();
	for occ in occurences {
		for k in occ {
			if !possible_answers.contains_key(&k.0) { possible_answers.insert(k.0, k.1 as usize); }
			else { (*possible_answers.get_mut(&k.0).unwrap()) += k.1 as usize; }
		}
	}
	return *possible_answers.values().max().unwrap();
}

pub fn run() {
	use std::time::Instant;
	let data: Vec<usize> = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}