fn gather_input(test: bool) -> Vec<Vec<isize>> {
	let mut res: Vec<Vec<isize>> = vec![];
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		for line in f.split("\n") {
			let mut l: Vec<isize> = vec![];
			for num in line.split(" ") {
				l.push(num.parse::<isize>().unwrap());
			}
			res.push(l);
		}
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn part1(data: &Vec<Vec<isize>>) -> usize {
	return data.iter()
		.filter(|x| {
			if x[0] < x[1] {
				for i in 0..x.len() - 1 {
					if x[i + 1] - x[i] > 3 || x[i + 1] - x[i] < 1 {
						return false;
					}
				}
			} else if x[0] > x[1] {
				for i in 0..x.len() - 1 {
					if x[i] - x[i + 1] > 3 || x[i] - x[i + 1] < 1 {
						return false;
					}
				}
			} else {
				return false;
			}
			return true;
		})
		.count();
}

fn part2(data: &Vec<Vec<isize>>) -> usize {
	fn has_unique_elements(a: &Vec<isize>) -> bool {
		use std::collections::HashSet;
		let mut s: HashSet<isize> = HashSet::new();
		for i in 0..a.len() {
			if !s.insert(a[i]) { return false }
		}
		return true;
	}

	fn is_good(a: &Vec<isize>, inner_call: bool) -> bool {
		// first check if the initial vec is good,
		// then try removing each of the elements to make it good
		let mut good_inc: Vec<isize> = a.clone();
		good_inc.sort();
		let mut good_dec: Vec<isize> = a.clone();
		good_dec.sort();
		good_dec.reverse();
		let mut flag: bool = true;
		if *a == good_inc && has_unique_elements(a) {
			for i in 0..a.len() - 1 {
				let diff = a[i + 1]- a[i];
				if diff < 1 || diff > 3 {
					flag = false;
					break;
				}
			}
		} else if *a == good_dec && has_unique_elements(a) {
			for i in 0..a.len() - 1 {
				let diff = a[i]- a[i + 1];
				if diff < 1 || diff > 3 {
					flag = false;
					break;
				}
			}
		} else {
			flag = false;
		}
		if !flag && !inner_call {
			for i in 0..a.len() {
				let mut new: Vec<isize> = a.clone();
				new.remove(i);
				if is_good(&new, true) {
					flag = true;
					break;
				}
			}
		}
		return flag;
	}

	return data.iter()
		.filter(|x| is_good(x, false))
		.count();
}

pub fn run() {
	use std::time::Instant;
	let data: Vec<Vec<isize>> = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}
