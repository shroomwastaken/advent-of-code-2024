fn gather_input(test: bool) -> Vec<u8> {
	let res: Vec<u8>;
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		res = f.chars().map(|x| x.to_string().parse::<u8>().unwrap()).collect();
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn part1(data: &Vec<u8>) -> usize {
	let mut new: Vec<usize> = vec![];
	// we'll start with one to be able to represent empty space with 0
	// we'll do i-1 when calculating the checksum
	let mut cur_f: usize = 1;
	for i in 0..data.len() {
		if i % 2 == 0 {
			for _ in 0..data[i] { new.push(cur_f); }
			cur_f += 1;
		} else {
			for _ in 0..data[i] { new.push(0); }
		}
	}
	for i in 0..new.len() {
		if new[i] == 0 {
			let swapidx: usize = new.len() - new.iter().rev().position(|x| *x != 0).unwrap() - 1;
			if swapidx < i { break; }
			new.swap(i, swapidx);
		}
	}
	let mut res: usize = 0;
	for i in 0..new.len() {
		if new[i] == 0 { break; }
		res += (new[i] - 1) * i;
	}
	return res;
}

fn part2(data: &Vec<u8>) -> usize {
	use std::ops::Range;
	let mut new: Vec<usize> = vec![];
	let mut cur_f: usize = 1;
	for i in 0..data.len() {
		if i % 2 == 0 {
			for _ in 0..data[i] { new.push(cur_f); }
			cur_f += 1;
		} else {
			for _ in 0..data[i] { new.push(0); }
		}
	}

	// the range itself, has it been moved at all, the id of the file
	let mut rs: Vec<(Range<usize>, bool, usize)> = vec![];

	let mut i: usize = 0;
	while i < new.len() {
		if new[i] != 0 {
			let start: usize = i;
			let start_val: usize = new[i];
			while i < new.len() && new[i] != 0 && new[i] == start_val { i += 1; }
			rs.push((start..i, false, start_val));
		} else { i += 1; }
	}

	// we go in order of decreasing id, so from the end of the vec to the beginning
	i = rs.len() - 1;
	while i != 0 {
		let mut moved: bool = false;
		// if the current block hasn't been moved yet at all
		if !rs[i].1 {
			// this + 1 covers the stupidest possible fucking edge case of all time
			// the block can be moved in the same in-between area between blocks
			// 12200044000055 can become 12244000000055
			// (actually took me more than 2 hours to realize this)
			for j in 1..i + 1 {
				// gap we'll try to fit the block into
				let free_space: usize = rs[j].0.start - rs[j - 1].0.end;

				// if block can't fit into the gap
				if rs[i].0.len() > free_space { continue; }

				let moved_range: Range<usize> = rs[j - 1].0.end..rs[j - 1].0.end+rs[i].0.len();
				let val = rs[i].2;
				rs.remove(i);
				rs.insert(j, (moved_range, true, val)); // put the block in its correct place
				moved = true;
				break;
			}
		}
		// if we weren't able to move the current block, try the one at index i - 1,
		// if we were able to do it the next block to be checked is still at index i
		// the loop ends when we aren't able to move any blocks so i goes to 0
		if !moved { i -= 1; }
	}
	let mut res: usize = 0;
	for item in 0..rs.len() {
		for j in 0..rs[item].0.len() {
			res += (rs[item].2 - 1) * (rs[item].0.start + j);
		}
	}
	return res;
}

pub fn run() {
	use std::time::Instant;
	let data: Vec<u8> = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}