// i hate mazes so much

use std::collections::HashSet;

const DIRECTIONS: [(i16, i16); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

fn gather_input(test: bool) -> Vec<Vec<char>> {
	let res: Vec<Vec<char>>;
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		res = f.split("\n").map(|x| x.chars().collect()).collect();
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn part1(data: &Vec<Vec<char>>) -> usize {
	let mut sr: i16 = 0;
	let mut sc: i16 = 0;
	let mut er: i16 = 0;
	let mut ec: i16 = 0;
	let mut distances: Vec<Vec<i16>> = vec![vec![0; data[0].len()]; data.len()];
	distances[sr as usize][sc as usize] = 0;
	for xr in 0..data.len() {
		for xc in 0..data[0].len() {
			if data[xr][xc] == 'S' { sr = xr as i16; sc = xc as i16; }
			if data[xr][xc] == 'E' { er = xr as i16; ec = xc as i16; }
			if data[xr][xc] == '#' { distances[xr][xc] = -1; }
		}
	}
	let mut r: i16 = sr;
	let mut c: i16 = sc;
	while (r, c) != (er, ec) {
		for dir in DIRECTIONS {
			let (nr, nc) = (r + dir.0, c + dir.1);
			if distances[nr as usize][nc as usize] == 0 {
				distances[nr as usize][nc as usize] = distances[r as usize][c as usize] + 1;
				r = nr;
				c = nc;
				break;
			}
		}
 	}

	let mut res: usize = 0;
	for cr in 1..(distances.len() - 1) as i16 {
		for cc in 1..(distances[0].len() - 1) as i16 {
			if distances[cr as usize][cc as usize] == -1 { continue; }
			let end_up: [(i16, i16); 8] = [
				(cr - 2, cc), (cr, cc + 2), (cr + 2, cc), (cr, cc - 2),
				(cr - 1, cc - 1), (cr - 1, cc + 1), (cr + 1, cc + 1), (cr + 1, cc - 1),
			];
			for (dr, dc) in end_up {
				if dr < 0 || dc < 0 || dr >= distances.len() as i16 || dc >= distances[0].len() as i16 { continue; }
				if distances[cr as usize][cc as usize] + 102 <= distances[dr as usize][dc as usize] { res += 1; }
			}
		}
	}

	return res;
}

fn part2(data: &Vec<Vec<char>>) -> usize {
	let mut sr: i16 = 0;
	let mut sc: i16 = 0;
	let mut er: i16 = 0;
	let mut ec: i16 = 0;
	let mut distances: Vec<Vec<i16>> = vec![vec![0; data[0].len()]; data.len()];
	distances[sr as usize][sc as usize] = 0;
	for xr in 0..data.len() {
		for xc in 0..data[0].len() {
			if data[xr][xc] == 'S' { sr = xr as i16; sc = xc as i16; }
			if data[xr][xc] == 'E' { er = xr as i16; ec = xc as i16; }
			if data[xr][xc] == '#' { distances[xr][xc] = -1; }
		}
	}
	let mut r: i16 = sr;
	let mut c: i16 = sc;
	while (r, c) != (er, ec) {
		for dir in DIRECTIONS {
			let (nr, nc) = (r + dir.0, c + dir.1);
			if distances[nr as usize][nc as usize] == 0 {
				distances[nr as usize][nc as usize] = distances[r as usize][c as usize] + 1;
				r = nr;
				c = nc;
				break;
			}
		}
 	}

	let mut res: usize = 0;
	for cr in 1..(distances.len() - 1) as i16 {
		for cc in 1..(distances[0].len() - 1) as i16 {
			if distances[cr as usize][cc as usize] == -1 { continue; }
			let mut end_up: HashSet<(i16, i16)> = HashSet::new();
			for steps in 2..21 {
				for cur in 0..steps + 1 {
					end_up.insert((cr + cur, cc - steps + cur));
					end_up.insert((cr + cur, cc + steps - cur));
					end_up.insert((cr - cur, cc + steps - cur));
					end_up.insert((cr - cur, cc - steps + cur));
				}
			}
			for (dr, dc) in end_up {
				if dr < 0 || dc < 0 || dr >= distances.len() as i16 || dc >= distances[0].len() as i16 { continue; }
				let dist: i16 = (cr as i16 - dr as i16).abs() + (cc as i16 - dc as i16).abs();
				if distances[cr as usize][cc as usize] + 100 + dist <= distances[dr as usize][dc as usize] { res += 1; }
			}
		}
	}

	return res;
}

pub fn run() {
	use std::time::Instant;
	let data: Vec<Vec<char>> = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}