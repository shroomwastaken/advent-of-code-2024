fn gather_input(test: bool) -> Vec<Vec<u8>> {
	let mut res: Vec<Vec<u8>> = vec![];
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		for line in f.split("\n") {
			let mut new: Vec<u8> = vec![];
			for ch in line.chars() {
				match ch {
					'.' => { new.push(0) }
					'#' => { new.push(1) }
					'^' => { new.push(2) }
					_ => {}
				}
			}
			res.push(new);
		}
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn part1(data: &Vec<Vec<u8>>) -> usize {
	// find lil guy
	let mut grow: isize = 0;
	let mut gcol: isize = 0;
	for line in 0..data.len() {
		if data[line].contains(&2) {
			grow = line as isize;
			gcol = data[line].iter().position(|x| *x == 2).unwrap() as isize;
			break;
		}
	}

	// 0 - up
	// 1 - right
	// 2 - down
	// 3 - left
	let mut direction: u8 = 0;

	let mut visited_buffer: Vec<Vec<bool>> = vec![];
	// idr how to fill up arrays fast ok
	for _ in 0..data.len() {
		let mut new: Vec<bool> = vec![];
		for _j in 0..data[1].len() {
			new.push(false);
		}
		visited_buffer.push(new)
	}

	while grow >= 0 && grow < data.len() as isize && gcol >= 0 && gcol < data[1].len() as isize {
		visited_buffer[grow as usize][gcol as usize] = true;
		match direction {
			0 => {
				if grow == 0 { grow -= 1; continue; }
				if data[(grow - 1) as usize][gcol as usize] == 1 { direction += 1; continue; }
				grow -= 1;
			}
			1 => {
				if gcol == (data[1].len() - 1) as isize { gcol += 1; continue; }
				if data[grow as usize][(gcol + 1) as usize] == 1 { direction += 1; continue; }
				gcol += 1;
			}
			2 => {
				if grow == (data.len() - 1) as isize {grow += 1; continue; }
				if data[(grow + 1) as usize][gcol as usize] == 1 { direction += 1; continue; }
				grow += 1;
			}
			3 => {
				if gcol == 0 { gcol -= 1; continue; }
				if data[grow as usize][(gcol - 1) as usize] == 1 { direction = 0; continue; }
				gcol -= 1;
			}
			_ => {}
		}
	}

	return visited_buffer.iter()
		.map(|x| {
			let mut y: usize = 0;
			// idr what the equivalent to python's count function is
			for i in x { if *i {y += 1} }
			return y;
		})
		.sum();
}

fn part2(data: &Vec<Vec<u8>>) -> usize {
	return 0;
}

pub fn run() {
	use std::time::Instant;
	let data: Vec<Vec<u8>> = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}