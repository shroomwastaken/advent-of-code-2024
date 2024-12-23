fn gather_input(test: bool) -> (Vec<Vec<u8>>, Vec<u8>) {
	let mut res: (Vec<Vec<u8>>, Vec<u8>) = (vec![], vec![]);
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		let mut parts: std::str::Split<'_, &str> = f.split("\n\n");
		for line in parts.next().unwrap().split("\n") {
			res.0.push(line.chars().map(|x| match x {'.' => 0, '#' => 1, 'O' => 2, '@' => 3, _ => { panic!() },}).collect::<Vec<u8>>());
		}
		for line in parts.next().unwrap().split("\n") {
			res.1.extend(line.chars().map(|x| match x { '^' => 0, '>' => 1, 'v' => 2, '<' => 3, _ => { panic!() },}));
		}
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn part1(map: &Vec<Vec<u8>>, moves: &Vec<u8>) -> usize {
	// part 1 solution by me (evidently)
	let mut map: Vec<Vec<u8>> = map.clone();
	let mut row: usize = 0;
	let mut col: usize = 0;
	// find the robot
	for r in 0..map.len() {
		for c in 0..map[r].len() {
			if map[r][c] == 3 { row = r; col = c; break; }
		}
	}
	// simulate movement
	for m in moves {
		match *m {
			// up
			0 => {
				if map[row - 1][col] == 0 {
					// if we can move freely in this direction
					map[row][col] = 0;
					map[row - 1][col] = 3;
					row -= 1;
				} else if map[row - 1][col] == 2 {
					// if we need to push a box
					let mut crow: usize = row - 1;
					// find the next empty space (or discover the lack thereof)
					while crow != 0 && map[crow][col] != 0 && map[crow][col] != 1 {
						crow -= 1;
					}
					if crow != 0 && map[crow][col] != 1 {
						// if we can indeed push boxes
						while crow != row {
							// swap the empty space with each box and robot
							// (effectively this moves everything up by one space)
							let store: u8 = map[crow][col];
							map[crow][col] = map[crow + 1][col];
							map[crow + 1][col] = store;
							crow += 1;
						}
						row -= 1;
					}
				}
			}
			// right
			// the same comments from above apply here just with different row and col values
			1 => {
				if map[row][col + 1] == 0 {
					map[row][col] = 0;
					map[row][col + 1] = 3;
					col += 1;
				} else if map[row][col + 1] == 2 {
					let mut ccol: usize = col + 1;
					while ccol < map[row].len() && map[row][ccol] != 0 && map[row][ccol] != 1 {
						ccol += 1;
					}
					if ccol != map[row].len() && map[row][ccol] != 1 {
						while ccol != col {
							let store: u8 = map[row][ccol];
							map[row][ccol] = map[row][ccol - 1];
							map[row][ccol - 1] = store;
							ccol -= 1;
						}
						col += 1;
					}
				}
			}
			// down
			2 => {
				if map[row + 1][col] == 0 {
					map[row][col] = 0;
					map[row + 1][col] = 3;
					row += 1;
				} else if map[row + 1][col] == 2 {
					let mut crow: usize = row + 1;
					while crow != map.len() && map[crow][col] != 0 && map[crow][col] != 1 {
						crow += 1;
					}
					if crow != map.len() && map[crow][col] != 1 {
						while crow != row {
							let store: u8 = map[crow][col];
							map[crow][col] = map[crow - 1][col];
							map[crow - 1][col] = store;
							crow -= 1;
						}
						row += 1;
					}
				}
			}
			// left
			3 => {
				if map[row][col - 1] == 0 {
					map[row][col] = 0;
					map[row][col - 1] = 3;
					col -= 1;
				} else if map[row][col - 1] == 2 {
					let mut ccol: usize = col - 1;
					while ccol != 0 && map[row][ccol] != 0 && map[row][ccol] != 1 {
						ccol -= 1;
					}
					if ccol != 0 && map[row][ccol] != 1 {
						while ccol != col {
							let store: u8 = map[row][ccol];
							map[row][ccol] = map[row][ccol + 1];
							map[row][ccol + 1] = store;
							ccol += 1;
						}
						col -= 1;
					}
				}
			}
			_ => {},
		}
	}
	let mut res: usize = 0;
	for line in 0..map.len() {
		for ch in 0..map[line].len() {
			if map[line][ch] == 2 { res += 100 * line + ch };
		}
	}
	return res;
}

fn part2(map: &Vec<Vec<u8>>, moves: &Vec<u8>) -> usize {
	// part 2 solution by HyperNeutrino (translated from python)
	// double the map
	let mut map: Vec<Vec<u8>> = map.iter().map(|x| {
		let mut nl: Vec<u8> = vec![];
		x.iter().for_each(|y| {
			match y {
				0 => { nl.push(0); nl.push(0); },
				1 => { nl.push(1); nl.push(1); },
				2 => { nl.push(4); nl.push(5); },
				3 => { nl.push(3); nl.push(0); },
				_ => {},
			}
		});
		return nl;
	}).collect();
	let mut row: i8 = 0;
	let mut col: i8 = 0;
	// find the robot
	for r in 0..map.len() {
		for c in 0..map[r].len() {
			if map[r][c] == 3 { row = r as i8; col = c as i8; break; }
		}
	}
	// simulate the movement
	for m in moves {
		// how much we move vertically
		let dr: i8 = [-1, 0, 1, 0][*m as usize];
		// how much we move horizontally
		let dc: i8 = [0, 1, 0, -1][*m as usize];
		// if we should move at all
		let mut go: bool = true;
		// coords of things that are going to be moving
		let mut targets: Vec<(i8, i8)>;
		// for continuously updating targets in the loop
		let mut new_targets: Vec<(i8, i8)> = vec![(row, col)];
		loop {
			targets = new_targets.clone();
			for (cr, cc) in &targets {
				// where the object will have moved
				let nr: i8 = *cr as i8 + dr;
				let nc: i8 = *cc as i8 + dc;
				// don't double count
				if targets.contains(&(nr, nc)) { continue; }
				// what we're looking to move
				let comp: u8 = map[nr as usize][nc as usize];
				// if we've hit a wall, stop
				if comp == 1 { go = false; break; }
				// if we've hit the side of a box, add it and the other side to targets
				if comp == 4 { new_targets.push((nr, nc)); new_targets.push((nr, nc + 1)); }
				if comp == 5 { new_targets.push((nr, nc)); new_targets.push((nr, nc - 1)); }
			}
			// if we havent added any more moving things (meaning everything is good to go)
			// or if we hit a wall, break out of the loop
			if new_targets == targets || go == false { break; }
		}
		// if we hit a wall, don't move
		if !go { continue; }
		let copy: Vec<Vec<u8>> = map.clone();
		// move the robot
		map[row as usize][col as usize] = 0;
		map[(row + dr) as usize][(col + dc) as usize] = 3;
		// clear places where the moving objects were before they moved
		for (br, bc) in &targets[1..] {
			map[*br as usize][*bc as usize] = 0;
		}
		// put moved objects in their correct places
		for (br, bc) in &targets[1..] {
			map[(*br + dr) as usize][(*bc + dc) as usize] = copy[*br as usize][*bc as usize];
		}
		// update robot position
		row += dr;
		col += dc;
	}
	let mut res: usize = 0;
	for line in 0..map.len() {
		for ch in 0..map[line].len() {
			if map[line][ch] == 4 { res += 100 * line + ch };
		}
	}
	return res;
}

pub fn run() {
	use std::time::Instant;
	let (map, moves) = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&map, &moves), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&map, &moves), Instant::now().duration_since(start));
}