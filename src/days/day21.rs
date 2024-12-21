use std::{collections::{HashMap, HashSet}, vec};

fn gather_input(test: bool) -> Vec<String> {
	let res: Vec<String>;
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		res = f.split("\n").map(|x| x.to_string()).collect();
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;

}

const NUMPAD: [[char; 3]; 4] = [
	['7', '8', '9'],
	['4', '5', '6'],
	['1', '2', '3'],
	['\0', '0', 'A'],
];

const DIRPAD: [[char; 3]; 2] = [
	['\0', '^', 'A'],
	['<', 'v', '>'],
];

// i don't want to add dependencies
// this function was brought to you by perplexity.ai
fn permute<T: Clone>(arr: &mut Vec<T>, start: usize, result: &mut Vec<Vec<T>>) {
	if arr.len() == 0 {
        return;
    }

	if start == arr.len() - 1 {
		result.push(arr.clone());
		return;
	}

	for i in start..arr.len() {
		arr.swap(start, i);

		permute(arr, start + 1, result);

		arr.swap(start, i);
	}
}

fn path_is_good(numpad: bool, r1: usize, c1: usize, path: &Vec<char>) -> bool {
	let mut cr: usize = r1;
	let mut cc: usize = c1;
	let h: usize = if numpad {3} else {1};
	let w: usize = 2;
	for p in path {
		match p {
			'^' => { if cr == 0 { return false; }; cr -= 1; },
			'>' => { if cc == w { return false; }; cc += 1; },
			'v' => { if cr == h { return false; }; cr += 1; },
			'<' => { if cc == 0 { return false; }; cc -= 1; },
			_ => {},
		};
		if numpad && NUMPAD[cr][cc] == '\0' { return false; }
		else if !numpad && DIRPAD[cr][cc] == '\0' { return false; }
	}
	return true;
}

// step 1. compute all the shortest paths from all keys to all other keys

fn compute_numpad_paths() -> HashMap<(char, char), HashSet<Vec<char>>> {
	let mut numpad_paths: HashMap<(char, char), HashSet<Vec<char>>> = HashMap::new();
	for r1 in 0..NUMPAD.len() {
		for c1 in 0..NUMPAD[r1].len() {
			for r2 in 0..NUMPAD.len() {
				for c2 in 0..NUMPAD[r2].len() {
					// we are avoiding the gap in the keypad like the plague
					if (r1 == 3 && c1 == 0) || (r2 == 3 && c2 == 0) { continue; }
					let mut path: Vec<char> = vec![];
					let mut cr: usize = r1;
					let mut cc: usize = c1;
					// if r1c1 is 0 or A, go to r2 then c2, else go to c2 then r2
					if r1 == 3 { while cr != r2 { if r2 < cr { path.push('^'); cr -= 1; } else { path.push('v'); cr += 1; }} }
					while cc != c2 { if c2 < cc { path.push('<'); cc -= 1; } else { path.push('>'); cc += 1; } }
					if r1 != 3 { while cr != r2 { if r2 < cr { path.push('^'); cr -= 1; } else { path.push('v'); cr += 1; }} }
					let mut permutations: Vec<Vec<char>> = vec![];
					permute::<char>(&mut path, 0, &mut permutations);
					let mut paths: HashSet<Vec<char>> = HashSet::new();
					for item in permutations {
						let mut new: Vec<char> = item;
						if new.is_empty() {println!("here");}
						new.push('A');
						if path_is_good(true, r1, c1, &new) {
							paths.insert(new);
						}
					}
					if paths.is_empty() { paths.insert(vec!['A']); }
					numpad_paths.insert((NUMPAD[r1][c1], NUMPAD[r2][c2]), paths);
				}
			}
		}
	}
	return numpad_paths;
}

fn compute_dirpad_paths() -> HashMap<(char, char), HashSet<Vec<char>>> {
	let mut dirpad_paths: HashMap<(char, char), HashSet<Vec<char>>> = HashMap::new();
	for r1 in 0..DIRPAD.len() {
		for c1 in 0..DIRPAD[r1].len() {
			for r2 in 0..DIRPAD.len() {
				for c2 in 0..DIRPAD[r2].len() {
					if (r1 == 0 && c1 == 0) || (r2 == 0 && c2 == 0) { continue; }
					let mut cr: usize = r1;
					let mut cc: usize = c1;
					let mut path: Vec<char> = vec![];
					if r1 == 0 { while cr != r2 { path.push('v'); cr += 1; } }
					while cc != c2 { if c2 < cc { path.push('<'); cc -= 1; } else { path.push('>'); cc += 1; } }
					if r1 != 0 { while cr != r2 { path.push('^'); cr -= 1; } }
					let mut permutations: Vec<Vec<char>> = vec![];
					permute::<char>(&mut path, 0, &mut permutations);
					let mut paths: HashSet<Vec<char>> = HashSet::new();
					for item in permutations {
						let mut new: Vec<char> = item;
						new.push('A');
						if path_is_good(false, r1, c1, &new) {
							paths.insert(new);
						}
					}
					if paths.is_empty() { paths.insert(vec!['A']); }
					dirpad_paths.insert((DIRPAD[r1][c1], DIRPAD[r2][c2]), paths);
				}
			}
		}
	}
	return dirpad_paths;
}

fn part1(data: &Vec<String>) -> usize {
	let mut res: usize = 0;
	let numpad_paths: HashMap<(char, char), HashSet<Vec<char>>> = compute_numpad_paths();
	let dirpad_paths: HashMap<(char, char), HashSet<Vec<char>>> = compute_dirpad_paths();
	// step 2. trace the letters back to our needed inputs
	for code in data {
		let mut complexity: usize = code[..code.len() - 1].parse::<usize>().unwrap();
		let amount_of_presses: usize;

		let poss1: [HashSet<Vec<char>>; 4] = [
			numpad_paths[&('A', code.chars().nth(0).unwrap())].clone(),
			numpad_paths[&(code.chars().nth(0).unwrap(), code.chars().nth(1).unwrap())].clone(),
			numpad_paths[&(code.chars().nth(1).unwrap(), code.chars().nth(2).unwrap())].clone(),
			numpad_paths[&(code.chars().nth(2).unwrap(), code.chars().nth(3).unwrap())].clone(),
		];
		let mut poss1all: Vec<Vec<char>> = vec![];
		for item1 in &poss1[0] {
			for item2 in &poss1[1] {
				for item3 in &poss1[2] {
					for item4 in &poss1[3] {
						let possible_path: Vec<char> = [item1.as_slice(), item2, item3, item4].concat();
						poss1all.push(possible_path);
					}
				}
			}
		}
		let mut poss2all: Vec<Vec<char>> = vec![];
		for path in poss1all {
			let mut pathposs: Vec<HashSet<Vec<char>>> = vec![dirpad_paths[&('A', path[0])].clone()];
			for i in 1..path.len() {
				pathposs.push(dirpad_paths[&(path[i - 1], path[i])].clone());
			}
			let mut p: Vec<Vec<char>> = pathposs[0].iter().map(|x| (*x).clone()).collect::<Vec<Vec<char>>>();
			for i in 1..pathposs.len() {
				let mut new_p = vec![];
				for it in &p {
					for it2 in &pathposs[i] {
						let mut new = it.clone();
						new.extend(it2);
						new_p.push(new.clone());
					}
				}
				p = new_p;
			};
			poss2all.extend(p);
		}
		let mut poss3all: Vec<Vec<char>> = vec![];
		for path in poss2all {
			let mut pathposs: Vec<HashSet<Vec<char>>> = vec![dirpad_paths[&('A', path[0])].clone()];
			for i in 1..path.len() {
				pathposs.push(dirpad_paths[&(path[i - 1], path[i])].clone());
			}
			let mut p: Vec<Vec<char>> = pathposs[0].iter().map(|x| (*x).clone()).collect::<Vec<Vec<char>>>();
			for i in 1..pathposs.len() {
				let mut new_p = vec![];
				for it in &p {
					for it2 in &pathposs[i] {
						let mut new = it.clone();
						new.extend(it2);
						new_p.push(new.clone());
					}
				}
				p = new_p;
			};
			poss3all.extend(p);
		}
		amount_of_presses = poss3all.iter().min_by_key(|x| (*(*x)).len()).unwrap().len();
		complexity *= amount_of_presses;
		res += complexity;
	}
	return res;
}

fn part2(data: &Vec<String>) -> usize {
	let mut res: usize = 0;
	return res;
}

pub fn run() {
	use std::time::Instant;
	let data: Vec<String> = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}