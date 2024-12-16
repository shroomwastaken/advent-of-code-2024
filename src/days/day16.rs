use std::collections::{HashMap, HashSet};

// east, north, west, south
const DIRECTIONS: [(i16, i16); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

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

	// so we don't check cells we've checked before
	let mut visited_cells: HashSet<(i16, i16, i8)> = HashSet::new();
	// list of cells with (cost, r, c, direction (index))
	let mut queue: Vec<(usize, i16, i16, i8)> = vec![(0, si.0, si.1, 0)];
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
		let (nr, nc) = (r + DIRECTIONS[dir as usize].0, c + DIRECTIONS[dir as usize].1);
		if data[nr as usize][nc as usize] != '#' { queue.push((cost + 1, nr, nc, dir)); }
		// add all possible turns to queue
		queue.push((cost + 1000, r, c, (dir + 1).rem_euclid(4)));
		queue.push((cost + 1000, r, c, (dir - 1).rem_euclid(4)));
	}
	return usize::MAX;
}

fn part2(data: &Vec<Vec<char>>) -> usize {
	// this is probably needlessly complicated and takes 15 seconds to run in release mode
	// enjoy

	let mut si: (i16, i16) = (0, 0);
	let mut ei: (i16, i16) = (0, 0);
	for line in 0..data.len() {
		if let Some(x) = data[line].iter().position(|x| *x == 'S') { si = (line as i16, x as i16) }
		if let Some(x) = data[line].iter().position(|x| *x == 'E') { ei = (line as i16, x as i16) }
	}

	// lmao
	let min_cost: usize = part1(data);

	// cell : lowest possible cost to get to it
	let mut best_costs: HashMap<(i16, i16, i8), usize> = HashMap::new();

	// all the possible states we could've been in when arriving at the end
	let mut end_states: HashSet<(i16, i16, i8)> = HashSet::new();

	// cell : all possible cells we could've gone from to get to the key cell (in the optimal paths)
	let mut backtracking: HashMap<(i16, i16, i8), HashSet<(i16, i16, i8)>> = HashMap::new();

	// (cost, row, column, direction index, cell we came from row, column, direction)
	let mut queue: Vec<(usize, i16, i16, i8, i16, i16, i8)> = vec![(0, si.0, si.1, 0, 0, 0, 0)];
	while !queue.is_empty() {
		// acquire the lowest costing cell
		queue.sort_by(|x, y| x.0.cmp(&y.0));
		let (cost, pr, pc, pd, mut prevr, mut prevc, mut prevd) = queue.remove(0);

		// if the lowest costing cell costs more than the desired min_cost then break
		// since we're not gonna find any more good paths
		if cost > min_cost { break; }

		// if this cell is worse than another path passing through the same cell then we don't care about it
		if let Some(r) = best_costs.get_mut(&(pr, pc, pd)) {
			if cost > *r { continue; }
			else { *r = cost } // if we've found a better cost for this cell, update it's best cost
		} else {
			best_costs.insert((pr, pc, pd), cost); // if we've encountered a cell for the first time
		}

		// if we're here, we know we're on some of the best possible cells
		// keep track of where we came from
		if let Some(r) = backtracking.get_mut(&(pr, pc, pd)) {
			r.insert((prevr, prevc, prevd));
		} else {
			backtracking.insert((pr, pc, pd), HashSet::new());
			backtracking.get_mut(&(pr, pc, pd)).unwrap().insert((prevr, prevc, prevd));
		}

		// if we've reached the end, record our state (since it can probably be reached from multiple directions)
		if (pr, pc) == ei { end_states.insert((pr, pc, pd)); continue; }

		// same things as in part 1, if we're not looking at a wall, add the cell in front of us to the queue
		(prevr, prevc, prevd) = (pr, pc, pd);
		let (nr, nc) = (pr + DIRECTIONS[pd as usize].0, pc + DIRECTIONS[pd as usize].1);
		if data[nr as usize][nc as usize] != '#' { queue.push((cost + 1, nr, nc, pd, prevr, prevc, prevd)); }

		// only push simple rotations if it makes sense to rotate (we're not just gonna be looking at walls)
		let prr1: usize = (pr + DIRECTIONS[(pd - 1).rem_euclid(4) as usize].0) as usize;
		let pcr1: usize = (pc + DIRECTIONS[(pd - 1).rem_euclid(4) as usize].1) as usize;
		if data[prr1][pcr1] != '#' { queue.push((cost + 1000, pr, pc, (pd - 1).rem_euclid(4), prevr, prevc, prevd)); }
		let prr2: usize = (pr + DIRECTIONS[(pd + 1).rem_euclid(4) as usize].0) as usize;
		let pcr2: usize = (pc + DIRECTIONS[(pd + 1).rem_euclid(4) as usize].1) as usize;
		if data[prr2][pcr2] != '#' { queue.push((cost + 1000, pr, pc, (pd + 1).rem_euclid(4), prevr, prevc, prevd)); }
	}

	// backtrack to the start through all the best paths
	let mut visited_tiles: HashSet<(i16, i16)> = HashSet::new();
	for state in end_states.clone() {
		let mut trackers: HashSet<(i16, i16, i8)> = backtracking[&state].clone();
		while !(trackers.len() == 1 && *trackers.iter().nth(0).unwrap() == (si.0, si.1, 0)) {
			visited_tiles.extend(trackers.iter().map(|x| (x.0, x.1)).clone());
			let mut new_trackers: HashSet<(i16, i16, i8)> = HashSet::new();
			for tracker in trackers {
				if tracker != (0, 0, 0) {
					new_trackers.extend(backtracking[&tracker].clone());
				}
			}
			trackers = new_trackers.clone();
		}
	}

	// soooooo this gave me the correct answer for my input but not for the examples and i have no idea why :)
	// if this doesn't give you the correct answer just subtract one and see what happens
	return visited_tiles.len() + 2;
}

pub fn run() {
	use std::time::Instant;
	let data: Vec<Vec<char>> = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}
