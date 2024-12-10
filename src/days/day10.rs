fn gather_input(test: bool) -> Vec<Vec<usize>> {
	let res: Vec<Vec<usize>>;
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		res = f.split("\n").map(|x| {
			x.chars().map(|y| y.to_digit(10).unwrap_or(255) as usize).collect::<Vec<usize>>()
		}).collect();
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn find_next_points(data: &Vec<Vec<usize>>, start: (usize, usize)) -> Vec<(usize, usize)> {
	let points_to_check: [(usize, usize); 4] = [
		(start.0.checked_sub(1).unwrap_or(255), start.1),
		(start.0, if start.1 + 1 >= data[0].len() { 255 } else { start.1 + 1 }),
		(if start.0 + 1 >= data.len() { 255 } else { start.0 + 1 }, start.1),
		(start.0, start.1.checked_sub(1).unwrap_or(255)),
	];
	let mut res: Vec<(usize, usize)> = vec![];
	for point in points_to_check {
		if point.0 == 255 || point.1 == 255 { continue; }
		if data[point.0][point.1] == data[start.0][start.1] + 1 {
			res.push(point)
		}
	}
	return res;
}

fn part1(data: &Vec<Vec<usize>>) -> usize {
	let mut zeroes: Vec<(usize, usize)> = vec![];
	for line in 0..data.len() {
		for ch in 0..data[line].len() {
			if data[line][ch] == 0 { zeroes.push((line, ch)) }
		}
	}

	let mut res: usize = 0;
	for pos in zeroes {
		let points: Vec<(usize, usize)> = find_next_points(data, pos);
		let mut to_be_checked: Vec<(usize, usize)> = points.clone();
		let mut found_nines: Vec<(usize, usize)> = vec![];
		while !to_be_checked.is_empty() {
			let mut new: Vec<(usize, usize)> = vec![];
			for p in to_be_checked {
				if data[p.0][p.1] == 9 {
					if !found_nines.contains(&p) { found_nines.push(p); };
					continue;
				}
				new.extend(find_next_points(data, p));
			}
			to_be_checked = new.clone();
		}
		res += found_nines.len();
	}
	return res;
}

fn part2(data: &Vec<Vec<usize>>) -> usize {
	let mut zeroes: Vec<(usize, usize)> = vec![];
	for line in 0..data.len() {
		for ch in 0..data[line].len() {
			if data[line][ch] == 0 { zeroes.push((line, ch)) }
		}
	}

	let mut res: usize = 0;
	for pos in zeroes {
		let mut trails: usize = 0;
		let points: Vec<(usize, usize)> = find_next_points(data, pos);
		let mut to_be_checked: Vec<(usize, usize)> = points.clone();
		while !to_be_checked.is_empty() {
			let mut new: Vec<(usize, usize)> = vec![];
			for p in to_be_checked {
				if data[p.0][p.1] == 9 { trails += 1; continue; }
				new.extend(find_next_points(data, p));
			}
			to_be_checked = new.clone();
		}
		res += trails;
	}
	return res;
}

pub fn run() {
	use std::time::Instant;
	let data: Vec<Vec<usize>> = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}