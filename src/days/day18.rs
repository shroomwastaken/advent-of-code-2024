use std::collections::HashSet;

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

fn gather_input(test: bool) -> Vec<(usize, usize)> {
	let res: Vec<(usize, usize)>;
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		res = f.split("\n")
			.map(|x| {
					let t = x.split_once(",").unwrap();
					return (t.0.parse::<usize>().unwrap(), t.1.parse::<usize>().unwrap());
				}
			).collect();
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn part1(data: &Vec<(usize, usize)>) -> usize {
	let mut map: Vec<Vec<u8>> = vec![vec![0; 71]; 71];
	for item in &data[..1024] {
		map[item.0][item.1] = 1;
	}
	let mut queue: Vec<(isize, isize, usize)> = vec![(0, 0, 0)];
	let mut visited: HashSet<(isize, isize)> = HashSet::new();
	// dijkstra ftw
	while !queue.is_empty() {
		queue.sort_by(|x, y| x.2.cmp(&y.2));
		let (r, c, cost) = queue.remove(0);
		if visited.contains(&(r, c)) { continue; }
		visited.insert((r, c));
		if (r, c) == (70, 70) { return cost; }
		for dir in DIRECTIONS {
			let nr = r + dir.0;
			let nc = c + dir.1;
			if 0 > nr || nr >= map.len() as isize { continue; }
			if 0 > nc || nc >= map[0].len() as isize { continue; }
			if map[nr as usize][nc as usize] != 1 {
				queue.push((r + dir.0, c + dir.1, cost + 1));
			}
		}
	}
	return 0;
}

fn part2(data: &Vec<(usize, usize)>) -> String {
	let mut map: Vec<Vec<u8>> = vec![vec![0; 71]; 71];
	for item in &data[..1024] {
		map[item.0][item.1] = 1;
	}
	let mut cur: usize = 1024;
	loop {
		let block: (usize, usize) = data[cur];
		map[block.0][block.1] = 1;
		// flood fill and check
		let mut queue: Vec<(isize, isize)> = vec![(0, 0)];
		let mut visited: HashSet<(isize, isize)> = HashSet::new();
		while !queue.is_empty() {
			let (r, c) = queue.remove(0);
			if visited.contains(&(r, c)) { continue; }
			if map[r as usize][c as usize] != 1 { visited.insert((r, c)); }
			for dir in DIRECTIONS {
				let nr: isize = r + dir.0;
				let nc: isize = c + dir.1;
				if 0 > nr || nr >= map.len() as isize { continue; }
				if 0 > nc || nc >= map[0].len() as isize { continue; }
				if map[nr as usize][nc as usize] != 1 {
					queue.push((r + dir.0, c + dir.1));
				}
			}
		}
		if !visited.contains(&(70, 70)) { break }
		cur += 1;
	}
	return format!("{},{}", data[cur].0, data[cur].1);
}

pub fn run() {
	use std::time::Instant;
	let data: Vec<(usize, usize)> = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}