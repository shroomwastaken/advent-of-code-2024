use std::collections::HashMap;

fn gather_input(test: bool) -> (Vec<(String, bool)>, Vec<(String, String, u8, String)>) {
	let mut res: (Vec<(String, bool)>, Vec<(String, String, u8, String)>) = (vec![], vec![]);
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		res.0 = f.split("\n\n")
			.nth(0)
			.unwrap()
			.split("\n")
			.map(|x|
				(x.split(": ").nth(0).unwrap().to_string(),
				match x.split(": ").nth(1).unwrap().parse::<u8>().unwrap() {
					0 => { false },
					1 => { true },
					_ => { panic!() }
				})
			).collect();
		res.1 = f.split("\n\n")
			.nth(1)
			.unwrap()
			.split("\n")
			.map(|x|
				(x.split(" ").nth(0).unwrap().to_string(),
				x.split(" ").nth(2).unwrap().to_string(),
				match x.split(" ").nth(1).unwrap() {
					"AND" => { 0u8 },
					"OR" => { 1 },
					"XOR" => { 2 },
					_ => { panic!(); }
				},
				x.split(" ").nth(4).unwrap().to_string())
			).collect();
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn part1(initial: &Vec<(String, bool)>, operations: &Vec<(String, String, u8, String)>) -> usize {
	let mut all_the_wires: HashMap<String, Option<bool>> = initial.iter().map(|x| (x.0.clone(), Some(x.1))).collect();
	for op in operations {
		if !all_the_wires.contains_key(&op.0) {
			all_the_wires.insert(op.0.clone(), None);
		}
		if !all_the_wires.contains_key(&op.1) {
			all_the_wires.insert(op.1.clone(), None);
		}
		if !all_the_wires.contains_key(&op.3) {
			all_the_wires.insert(op.3.clone(), None);
		}
	}
	let mut tries: usize = 0;
	while !all_the_wires.iter().all(|x| x.1.is_some()) {
		for op in operations {
			if all_the_wires.contains_key(&op.0) && all_the_wires.contains_key(&op.1)
			&& all_the_wires[&op.0].is_some() && all_the_wires[&op.1].is_some() {
				all_the_wires.insert(op.3.clone(), match op.2 {
					0 => { Some(all_the_wires[&op.0].unwrap() & all_the_wires[&op.1].unwrap()) },
					1 => { Some(all_the_wires[&op.0].unwrap() | all_the_wires[&op.1].unwrap()) },
					2 => { Some(all_the_wires[&op.0].unwrap() ^ all_the_wires[&op.1].unwrap()) },
					_ => { panic!(); },
				});
			}
		}
	}
	let mut z_wires: Vec<(String, bool) >= all_the_wires.iter()
		.filter(|x| x.0.starts_with("z"))
		.map(|x| (x.0.clone(), x.1.unwrap()))
		.collect();
	z_wires.sort_by_key(|z| z.0.clone());
	return usize::from_str_radix(
		&z_wires.iter().map(|x| if x.1 { '1' } else { '0' }).rev().collect::<String>(), 2
	).unwrap();
}

fn part2(initial: &Vec<(String, bool)>, operations: &Vec<(String, String, u8, String)>) -> String {
	// i initially solved this using python and dot to visualize the operations and my eyes to catch the mistakes
	// here's a solution that hopefully works for any input (excuse the dirtiness of the code i am never going to clean it up)
	// this works for my input but it may not work for yours if i missed something

	use std::collections::HashSet;

	let mut affected: HashSet<String> = HashSet::new();

	// values starting with z should always be the result of an XOR, except for the last one
	let mut z_ops: Vec<(String, String, u8, String)> = operations.iter()
		.filter(|x| x.3.starts_with("z") && x.2 != 2)
		.map(|x| x.clone())
		.collect();
	// so that the last z actually is last
	z_ops.sort_by_key(|x| x.3.clone());
	for op in &z_ops[..z_ops.len() - 1] { affected.insert(op.3.clone()); }

	// if a value is a carry bit (the result of an OR), it should come from values that were the result of an AND
	let carry_ops: Vec<(String, String, u8, String)> = operations.iter()
		.filter(|x| x.2 == 1 )
		.map(|x| x.clone())
		.collect();
	for op in carry_ops {
		let x = operations.iter().find(|x| op.0 == x.3 || op.1 == x.3);
		if x.is_none() { /* if we cant find one that means the op gives a zxx value, meaning we've printed it already */ continue; }
		// let source_x = operations.iter().find(|y| x.)
		if x.unwrap().2 != 0 {
			affected.insert(x.unwrap().3.clone());
		}
	}

	// if a value is the result of Xxx AND Yxx it should only have one operation where it's used, that being an OR
	// except y00 AND x00
	let check_ops: Vec<(String, String, u8, String)> = operations.iter()
		.filter(|x| (
			(x.0.starts_with("x") && x.1.starts_with("y")) ||
			(x.0.starts_with("y") && x.1.starts_with("x"))) &&
			x.2 == 0
		).map(|x| x.clone())
		.collect();
	for op in check_ops {
		let x = operations.iter().filter(|y| [y.0.clone(), y.1.clone()].contains(&op.3)).count();
		if x > 1 && !op.0.contains("00") { affected.insert(op.3.clone()); }
	}

	// similarly if a value is the result of Xxx XOR Yxx it should have exactly 2 operations where it's used (a XOR and an AND)
	// and the result of an XOR operation with that value should be a Zxx
	// except for y00 XOR x00
	let check_ops: Vec<(String, String, u8, String)> = operations.iter()
	.filter(|x| (
		(x.0.starts_with("x") && x.1.starts_with("y")) ||
		(x.0.starts_with("y") && x.1.starts_with("x"))) &&
		x.2 == 2
	).map(|x| x.clone())
	.collect();
	for op in check_ops {
		let x: usize = operations.iter()
			.filter(|y| [y.0.clone(), y.1.clone()].contains(&op.3)).count();
		let y = operations.iter().find(|j| [j.0.clone(), j.1.clone()].contains(&op.3) && j.2 == 2);
		if y.is_none() && !op.3.starts_with("z") { affected.insert(op.3.clone()); continue }
		if y.is_none() && op.3.starts_with("z") { continue; }
		if !y.unwrap().3.starts_with("z") { affected.insert(y.unwrap().3.clone()); }
		if x != 2 && !op.0.contains("00") { affected.insert(op.3.clone()); }
	}

	let mut affv: Vec<String> = affected.into_iter().collect();
	affv.sort();
	return affv.join(",");
}

pub fn run() {
	use std::time::Instant;
	let (initial, operations) = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&initial, &operations), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&initial, &operations), Instant::now().duration_since(start));
}