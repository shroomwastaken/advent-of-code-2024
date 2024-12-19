fn gather_input(test: bool) -> (Vec<String>,  Vec<String>) {
	let mut res: (Vec<String>,  Vec<String>) = (vec![], vec![]);
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		let parts: (&str, &str) = f.split_once("\n\n").unwrap();
		res.0 = parts.0.split(", ").map(|x| x.to_string()).collect();
		res.1 = parts.1.split("\n").map(|x| x.to_string()).collect();
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn part1(designs: &Vec<String>, patterns: &Vec<String>) -> usize {
	let mut res: usize = 0;
	for pattern in patterns {
		let mut possible: Vec<String> = vec![];
		for towel in designs {
			if pattern.starts_with(towel) { possible.push(towel.clone()); }
		}
		if possible.is_empty() { continue; }
		while !possible.is_empty() {
			let poss = possible.pop().unwrap();
			if poss == *pattern { res += 1; break; }
			for design in designs {
				if pattern[poss.len()..].starts_with(design) { possible.push(poss.clone() + design); }
			}
		}
	}
	return res;
}

fn part2(designs: &Vec<String>, patterns: &Vec<String>) -> usize {
	use lru::LruCache;

	fn solve(p: &String, designs: &Vec<String>, maxlen: usize, cache: &mut LruCache<String, usize>) -> usize {
		if let Some(x) = cache.get(p) {
			return *x;
		}
		let mut count: usize = 0;
		// base case, if we've found a good combination
		if p == "" { return 1; }
		// only check designs that fit into p if p is smaller than the maximum design length
		for i in 1..p.len().min(maxlen) + 1 {
			// if there's a fitting design
			if designs.contains(&p[..i].to_string()) {
				count += solve(&p[i..].to_string(), designs, maxlen, cache);
			}
		}
		cache.put(p.clone(), count);
		return count;
	}

	let maxlen: usize = patterns.iter().map(|x| x.len()).max().unwrap();
	let mut cache: LruCache<String, usize> = LruCache::unbounded();
	let mut res: usize = 0;
	for pat in patterns {
		res += solve(pat, designs, maxlen, &mut cache)
	}
	return res;
}

pub fn run() {
	use std::time::Instant;
	let data: (Vec<String>, Vec<String>) = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data.0, &data.1), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data.0, &data.1), Instant::now().duration_since(start));
}