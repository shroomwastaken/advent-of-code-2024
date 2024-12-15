fn gather_input(test: bool) -> (Vec<Vec<u8>>, Vec<u8>) {
	let mut res: (Vec<Vec<u8>>, Vec<u8>) = (vec![], vec![]);
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		let mut parts = f.split("\n\n");
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
	let mut map: Vec<Vec<u8>> = map.clone();
	let mut row: usize = 0;
	let mut col: usize = 0;
	for r in 0..map.len() {
		for c in 0..map[r].len() {
			if map[r][c] == 3 { row = r; col = c; break; }
		}
	}
	for m in moves {
		match *m {
			// up
			0 => {
				if map[row - 1][col] == 0 {
					map[row][col] = 0;
					map[row - 1][col] = 3;
					row -= 1;
				} else if map[row - 1][col] == 2 {
					let mut crow: usize = row - 1;
					while crow != 0 && map[crow][col] != 0 && map[crow][col] != 1 {
						crow -= 1;
					}
					if crow != 0 && map[crow][col] != 1 {
						while crow != row {
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
	let mut res = 0;
	for line in 0..map.len() {
		for ch in 0..map[line].len() {
			if map[line][ch] == 2 { res += 100 * line + ch };
		}
	}
	return res;
}

fn part2(map: &Vec<Vec<u8>>, moves: &Vec<u8>) -> usize {
	return 0;
}

pub fn run() {
	use std::time::Instant;
	let (map, moves) = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&map, &moves), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&map, &moves), Instant::now().duration_since(start));
}