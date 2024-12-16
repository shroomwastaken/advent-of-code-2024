use std::collections::{HashMap, HashSet};

fn gather_input(test: bool) -> Vec<Vec<char>> {
	let mut res: Vec<Vec<char>> = vec![];
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		res = f.split("\n").map(|x| x.chars().collect()).collect();
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn part1(data: &Vec<Vec<char>>) -> usize {
	// i had to learn what dijkstras algorithm is for this
	let mut si: (i16, i16) = (0, 0);
	let mut ei: (i16, i16) = (0, 0);
	for line in 0..data.len() {
		if let Some(x) = data[line].iter().position(|x| *x == 'S') { si = (line as i16, x as i16) }
		if let Some(x) = data[line].iter().position(|x| *x == 'E') { ei = (line as i16, x as i16) }
	}

	// east, north, west, south
	const DIRECTIONS: [(i16, i16); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

	// so we don't check cells we've checked before
	let mut visited_cells: HashSet<(i16, i16, usize)> = HashSet::new();
	// list of cells with (cost, r, c, direction (index))
	let mut queue: Vec<(usize, i16, i16, usize)> = vec![(0, si.0, si.1, 0)];
	while !queue.is_empty() {
		// sort by cost
		queue.sort_by(|x, y| x.0.cmp(&y.0));
		// get the least costly cell
		let (cost, r, c, dir) = queue.remove(0);
		// if we've checked this before, don't check it again
		if visited_cells.contains(&(r, c, dir)) { continue; }
		// if we're done
		if (r, c) == ei { return cost; }
		visited_cells.insert((r, c, dir));
		// if forwards isn't a wall, add forwards cell to queue
		let (nr, nc) = (r + DIRECTIONS[dir].0, c + DIRECTIONS[dir].1);
		if data[nr as usize][nc as usize] != '#' { queue.push((cost + 1, nr, nc, dir)); }
		// add all possible turns to queue
		queue.push((cost + 1000, r, c, (dir + 1).rem_euclid(4)));
		queue.push((cost + 1000, r, c, (dir - 1).rem_euclid(4)));
	}
	return usize::MAX;
}

fn part2(data: &Vec<Vec<char>>) -> usize {
	return 0;
}

pub fn run() {
	use std::time::Instant;
	let data: Vec<Vec<char>> = gather_input(true);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}
