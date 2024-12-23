use std::collections::{HashMap, HashSet};

fn gather_input(test: bool) -> Vec<(String, String)> {
	let mut res: Vec<(String, String)> = vec![];
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		for line in f.split("\n") {
			res.push((line.split_once('-').unwrap().0.to_string(), line.split_once('-').unwrap().1.to_string()));
		}
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn part1(data: &Vec<(String, String)>) -> usize {
	let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
	for d in data {
		if !connections.contains_key(&d.0) {
			connections.insert(d.0.clone(), HashSet::new());
		}
		connections.get_mut(&d.0).unwrap().insert(d.1.clone());
		if !connections.contains_key(&d.1) {
			connections.insert(d.1.clone(), HashSet::new());
		}
		connections.get_mut(&d.1).unwrap().insert(d.0.clone());
	}

	let connections2: HashMap<String, Vec<String>> = connections.iter().map(|x| {
		return (x.0.clone(), x.1.iter().map(|y| y.clone()).collect());
	}).collect();

	let mut sets_of_3: HashSet<[String; 3]> = HashSet::new();

	for conns in &connections2 {
		for i in 0..conns.1.len() - 1 {
			for j in i + 1..conns.1.len() {
				let part1 = conns.0.clone();
				let part2 = conns.1[i].clone();
				let part3 = conns.1[j].clone();
				if connections2[&part2].contains(&part1) && connections2[&part2].contains(&part3)
				&& connections2[&part3].contains(&part1) && connections2[&part3].contains(&part2) {
					sets_of_3.insert([part1, part2, part3]);
				}
			}
		}
	}

	return sets_of_3.iter().filter(|x| {
		return x.iter().any(|x| x.starts_with("t"));
	}).count() / 3;
}

fn combinations<T: Clone>(iterable: Vec<T>, r: usize) -> Vec<Vec<T>> {
	let n = iterable.len();
	if r > n {
		return vec![];
	}

	let mut indices: Vec<usize> = (0..r).collect();
	let mut result: Vec<Vec<T>> = Vec::new();

	result.push(indices.iter().map(|&i| iterable[i].clone()).collect());

	loop {
		let mut i: usize = r;
		while i > 0 {
			i -= 1;
			if indices[i] != i + n - r { break; }
		}
		if i == 0 { break; }
		indices[i] += 1;
		for j in (i + 1)..r { indices[j] = indices[j - 1] + 1; }
		result.push(indices.iter().map(|&i| iterable[i].clone()).collect());
	}

	return result;
}

fn part2(data: &Vec<(String, String)>) -> String {
	let mut connections: HashMap<String, Vec<String>> = HashMap::new();
	for d in data {
		if !connections.contains_key(&d.0) {
			connections.insert(d.0.clone(), vec![]);
		}
		connections.get_mut(&d.0).unwrap().push(d.1.clone());
		if !connections.contains_key(&d.1) {
			connections.insert(d.1.clone(), vec![]);
		}
		connections.get_mut(&d.1).unwrap().push(d.0.clone());
	}

	let connections2: HashMap<String, Vec<String>> = connections.iter().map(|x| {
		return (x.0.clone(), x.1.iter().map(|y| y.clone()).collect());
	}).collect();

	let mut sets: Vec<Vec<String>> = vec![];

	for conn in &connections2 {
		for i in 3..conn.1.len() + 1 {
			let combs: Vec<Vec<String>> = combinations(conn.1.clone(), i);
			for comb in combs {
				let mut conds: Vec<bool> = vec![];
				for elem in 0..comb.len() {
					let mut conds2: Vec<bool> = vec![];
					for elem1 in 0..comb.len() {
						if elem1 == elem { continue; }
						conds2.push(connections2[&comb[elem]].contains(&comb[elem1]));
					}
					conds.push(conds2.iter().all(|x| *x == true))
				}
				if conds.iter().all(|x| *x == true) {
					sets.push(comb);
				}
			}
		}
	}

	let maxlen: usize = sets.iter().map(|x| x.len()).max().unwrap();
	let good_sets: Vec<Vec<String>> = sets.iter().filter(|x| x.len() == maxlen).map(|x| x.clone()).collect();
	let mut comps: Vec<String> = vec![];
	for set in good_sets {
		for item in set {
			if !comps.contains(&item) {
				comps.push(item);
			}
		}
	}

	comps.sort();

	return comps.join(",");
}

pub fn run() {
	use std::time::Instant;
	let data: Vec<(String, String)> = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}