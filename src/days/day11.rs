fn gather_input(test: bool) -> Vec<usize> {
	let res: Vec<usize>;
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		res = f.split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn part1(data: &Vec<usize>) -> usize {
	let mut initial: Vec<usize> = data.clone();
	let mut new: Vec<usize> = vec![];
	for _ in 0..25 {
		for stone in 0..initial.len() {
			if initial[stone] == 0 { new.push(1); continue; }
			let digits = initial[stone].ilog10() + 1;
			if digits & 1 == 0 {
				let ten_to_power = 10usize.pow(digits / 2);
				new.extend([initial[stone] / ten_to_power, initial[stone] % ten_to_power]);
				continue;
			}
			new.push(initial[stone] * 2024);
		}
		initial = new.clone();
		new = vec![];
	}
	return initial.len();
}

fn part2(data: &Vec<usize>) -> usize {
	// this took me way too long to solve

	use std::collections::HashMap;
	fn iteration(nums: HashMap<usize, usize>) -> HashMap<usize, usize> {
		let mut new: HashMap<usize, usize> = HashMap::new();
		for (num, count) in nums {
			if num == 0 {
				if !new.contains_key(&1) { new.insert(1, count); }
				else { *new.get_mut(&1).unwrap() += count }
				continue;
			}
			let digits = num.ilog10() + 1;
			if digits & 1 == 0 {
				let ten_to_power: usize = 10usize.pow(digits >> 1);
				let left_half = num / ten_to_power;
				let right_half = num % ten_to_power;
				if !new.contains_key(&left_half) { new.insert(left_half, count); }
				else { *new.get_mut(&left_half).unwrap() += count }
				if !new.contains_key(&right_half) { new.insert(right_half, count); }
				else { *new.get_mut(&right_half).unwrap() += count }
				continue;
			}
			if !new.contains_key(&(num * 2024)) { new.insert(num * 2024, count); }
			else { *new.get_mut(&(num * 2024)).unwrap() += count }
		}
		return new;
	}

	let mut map: HashMap<usize, usize> = data.iter().map(|x| (*x, 1)).collect();
	for _ in 0..75 {
		map = iteration(map);
	}
	return map.values().sum();
}

pub fn run() {
	use std::time::Instant;
	let data: Vec<usize> = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}