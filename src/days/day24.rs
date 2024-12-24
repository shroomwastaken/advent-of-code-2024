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
		tries += 1;
		if tries == 100 { return usize::MAX; }
	}
	let mut z_wires: Vec<(String, bool) >= all_the_wires.iter()
		.filter(|x| x.0.starts_with("z"))
		.map(|x| (x.0.clone(), x.1.unwrap()))
		.collect();
	z_wires.sort_by_key(|z| z.0.clone());
	println!("{tries:?}");
	return usize::from_str_radix(
		&z_wires.iter().map(|x| if x.1 { '1' } else { '0' }).rev().collect::<String>(), 2
	).unwrap();
}

fn part2(initial: &Vec<(String, bool)>, operations: &Vec<(String, String, u8, String)>) -> String {
	// solved using python and dot to visualize the operations and my eyes to catch the mistakes
	return "dkr,ggk,hhh,htp,rhv,z05,z15,z20".to_string();
}

pub fn run() {
	use std::time::Instant;
	let (initial, operations) = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&initial, &operations), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&initial, &operations), Instant::now().duration_since(start));
}