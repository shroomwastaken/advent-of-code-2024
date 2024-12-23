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

fn find_next_points(data: &mut Vec<Vec<(char, bool)>>, start: (usize, usize)) -> Vec<(usize, usize)> {
	let points_to_check = [
		(start.0.checked_sub(1).unwrap_or(255), start.1),
		(start.0, if start.1 + 1 >= data[0].len() { 255 } else { start.1 + 1 }),
		(if start.0 + 1 >= data.len() { 255 } else { start.0 + 1 }, start.1),
		(start.0, start.1.checked_sub(1).unwrap_or(255)),
	];
	let initial_letter = data[start.0][start.1];
	let mut resvec: Vec<(usize, usize)> = vec![];
	for p in points_to_check {
		if p.0 == 255 || p.1 == 255 { continue; }
		let c: (char, bool) =  data[p.0][p.1];
		if c.0 == initial_letter.0 && !c.1 { data[p.0][p.1].1 = true; resvec.push(p); }
	}
	data[start.0][start.1].1 = true;
	return resvec;
}

fn part1(data: &Vec<Vec<char>>) -> usize {
	let mut data: Vec<Vec<(char, bool)>> = data.iter().map(|x| x.iter().map(|y| (*y, false)).collect()).collect();

	fn find_perimeter(points: Vec<(usize, usize)>) -> usize {
		fn find_neighbors(data: &Vec<Vec<u8>>, start: (usize, usize)) -> usize {
			let points_to_check: [(usize, usize); 4] = [
				(start.0.checked_sub(1).unwrap_or(255), start.1),
				(start.0, if start.1 + 1 >= data[0].len() { 255 } else { start.1 + 1 }),
				(if start.0 + 1 >= data.len() { 255 } else { start.0 + 1 }, start.1),
				(start.0, start.1.checked_sub(1).unwrap_or(255)),
			];
			let mut c: usize = 0;
			for p in points_to_check {
				if p.0 == 255 || p.1 == 255 { continue; }
				if data[p.0][p.1] == 1 { c += 1; }
			}
			return c;
		}

		let mut matrix: Vec<Vec<u8>> = vec![];
		// arbitrary 140 by 140 matrix
		for _ in 0..140 { matrix.push(vec![0; 140]) }

		for p in &points { matrix[p.0][p.1] = 1; }
		let mut perimeter: usize = 0;
		for p in points { perimeter += 4 - find_neighbors(&matrix, p); }

		return perimeter;
	}

	let mut crow: usize = 0;
	let mut ccol: usize = 0;
	let mut price: usize = 0;
	// while we havent checked everything
	while crow < data.len() {
		if data[crow][ccol].1 { ccol += 1; if ccol >= data[0].len() { crow += 1; ccol = 0; }; continue; }
		let mut region_area: usize = 1;
		let mut points: Vec<(usize, usize)> = find_next_points(&mut data, (crow, ccol));
		let mut all_points: Vec<(usize, usize)> = vec![(crow, ccol)];
		all_points.extend(points.clone());
		while !points.is_empty() {
			region_area += points.len();
			let mut new: Vec<(usize, usize)> = vec![];
			for p in points {
				let found_points: Vec<(usize, usize)> = find_next_points(&mut data, p);
				new.extend(found_points);
			}
			points = new.clone();
			all_points.extend(points.clone());
		}
		let region_perimeter = find_perimeter(all_points);
		price += region_area * region_perimeter;
		ccol += 1;
		if ccol >= data[0].len() { crow += 1; ccol = 0; }
	}
	return price;
}

fn part2(data: &Vec<Vec<char>>) -> usize {
	let mut data: Vec<Vec<(char, bool)>> = data.iter().map(|x| x.iter().map(|y| (*y, false)).collect()).collect();

	fn find_number_of_sides(points: Vec<(usize, usize)>) -> usize {
		let mut matrix: Vec<Vec<u8>> = vec![];
		// arbitrarily sized matrix
		for _ in 0..160 { matrix.push(vec![0; 160]) }

		for p in &points { matrix[p.0+1][p.1+1] = 1; }

		let mut sides: usize = 0;

		// traverse the outer shape
		let beginning_point: usize = matrix.iter()
			.map(|x| x.iter().position(|y| *y == 1).unwrap_or(255))
			.position(|z| z < 255)
			.unwrap();
		let mut prow: usize = beginning_point;
		let mut pcol: usize = matrix[beginning_point].iter().position(|x| *x == 1).unwrap();
		let return_point: (usize, usize) = (prow, pcol);

		// 0 - left, 1 - up, 2 - right, 3 - down
		let mut direction: usize = 0;

		// probably way overcomplicated shape border traversal algorithm
		// the amount of sides of a shape == the amount of times we change direction while traversing its borders
		let mut returned: bool = false;
		while !returned {
			match direction {
				0 => {
					if matrix[prow - 1][pcol] != 0 && matrix[prow][pcol - 1] == 0 { prow -= 1; }
					else if matrix[prow][pcol - 1] == 1 { direction = 3; sides += 1; pcol -= 1; }
					else if matrix[prow - 1][pcol] == 0 { direction = 1; sides += 1; if matrix[prow][pcol + 1] == 1 { pcol += 1; } }
				}
				1 => {
					if matrix[prow][pcol + 1] != 0 && matrix[prow - 1][pcol] == 0 { pcol += 1; }
					else if matrix[prow - 1][pcol] == 1 { direction = 0; sides += 1; prow -= 1; }
					else if matrix[prow][pcol + 1] == 0 { direction = 2; sides += 1; if matrix[prow + 1][pcol] == 1 { prow += 1; } }
				}
				2 => {
					if matrix[prow + 1][pcol] != 0 && matrix[prow][pcol + 1] == 0 { prow += 1; }
					else if matrix[prow][pcol + 1] == 1 { direction = 1; sides += 1; pcol += 1; }
					else if matrix[prow + 1][pcol] == 0 { direction = 3; sides += 1; if matrix[prow][pcol - 1] == 1 { pcol -= 1; } }
				}
				3 => {
					if matrix[prow][pcol - 1] != 0 && matrix[prow + 1][pcol] == 0 { pcol -= 1; }
					else if matrix[prow + 1][pcol] == 1 { direction = 2; sides += 1; prow += 1; }
					else if matrix[prow][pcol - 1] == 0 { direction = 0; sides += 1; if matrix[prow - 1][pcol] == 1 { prow -= 1; } }
				}
				_ => {}
			}
			returned = (prow, pcol) == return_point && direction == 0;
		}

		// now we need to find all the shapes that are enclosed in the outer shape
		let mut nmatrix: Vec<Vec<(char, bool)>> = vec![];
		for _ in 0..160 { nmatrix.push(vec![(0 as char, false); 160]) }

		// flood-fill everything, all thats left are the enclosed shapes which we need to check
		let frow: usize = matrix.iter().position(|x| x.contains(&0)).unwrap();
		let fcol: usize = matrix[frow].iter().position(|x| *x == 0).unwrap();

		let target_color: u8 = matrix[frow][fcol];
		let new_color: u8 = 1;

		let mut stack: Vec<(usize, usize)> = vec![(frow, fcol)];

		while let Some((x, y)) = stack.pop() {
			matrix[x][y] = new_color;

			let directions: [(isize, isize); 8] = [
				(-1, 0), (-1, 1), (0, 1),
				(1, 1), (1, 0), (1, -1),
				(0, -1), (-1, -1),
			];

			for &(dx, dy) in &directions {
				let mut nx: isize = x as isize + dx;
				let mut ny: isize = y as isize + dy;

				while nx >= 0 && nx < matrix.len() as isize && ny >= 0 && ny < matrix[0].len() as isize {
					if matrix[nx as usize][ny as usize] == target_color {
						matrix[nx as usize][ny as usize] = new_color;
						stack.push((nx as usize, ny as usize));
					} else { break; }
					nx += dx;
					ny += dy;
				}
			}
		}

		// invert the flood-filled matrix, now we have only the enclosed shapes
		// everything else is 0
		for row in 0..matrix.len() {
			for col in 0..matrix[row].len() {
				if matrix[row][col] == 0 { nmatrix[row][col] = ('A', false) }
			}
		}

		// find amount of sides of shapes that were enclosed in the outer shape
		// then add them to the amount of sides of the outer shape
		// this is the same thing we do outside of this function
		// i did this since we need to traverse all the inner shapes individualy
		// and at the moment we have *all* the points of *all* the inner shapes
		// (i hope this made sense)
		let mut crow: usize = 0;
		let mut ccol: usize = 0;
		while crow < nmatrix.len() {
			if nmatrix[crow][ccol].1 || nmatrix[crow][ccol].0 == 0 as char { ccol += 1; if ccol >= nmatrix[0].len() { crow += 1; ccol = 0; }; continue; }
			let mut points: Vec<(usize, usize)> = find_next_points(&mut nmatrix, (crow, ccol));
			let mut all_points: Vec<(usize, usize)> = vec![(crow, ccol)];
			all_points.extend(points.clone());
			while !points.is_empty() {
				let mut new: Vec<(usize, usize)> = vec![];
				for p in points {
					let found_points: Vec<(usize, usize)> = find_next_points(&mut nmatrix, p);
					new.extend(found_points);
				}
				points = new.clone();
				all_points.extend(points.clone());
			}
			sides += find_number_of_sides(all_points); // traverse inner shape that we got
			ccol += 1;
			if ccol >= nmatrix[0].len() { crow += 1; ccol = 0; }
		}

		return sides;
	}

	let mut crow: usize = 0;
	let mut ccol: usize = 0;
	let mut price: usize = 0;
	while crow < data.len() {
		if data[crow][ccol].1 { ccol += 1; if ccol >= data[0].len() { crow += 1; ccol = 0; }; continue; }
		let mut region_area: usize = 1;
		let mut points: Vec<(usize, usize)> = find_next_points(&mut data, (crow, ccol));
		let mut all_points: Vec<(usize, usize)> = vec![(crow, ccol)];
		all_points.extend(points.clone());
		while !points.is_empty() {
			region_area += points.len();
			let mut new: Vec<(usize, usize)> = vec![];
			for p in points {
				let found_points: Vec<(usize, usize)> = find_next_points(&mut data, p);
				new.extend(found_points);
			}
			points = new.clone();
			all_points.extend(points.clone());
		}
		let number_of_sides: usize = find_number_of_sides(all_points);
		price += region_area * number_of_sides;
		ccol += 1;
		if ccol >= data[0].len() { crow += 1; ccol = 0; }
	}
	return price;
}

pub fn run() {
	use std::time::Instant;
	let data: Vec<Vec<char>> = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}