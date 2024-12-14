#[derive(Debug)]
struct Robot {
	x: isize,
	y: isize,
	vx: isize,
	vy: isize,
}

fn gather_input(test: bool) -> Vec<Robot> {
	let mut res: Vec<Robot> = vec![];
	if let Ok(f) = std::fs::read_to_string(if test { "ti.txt" } else { "i.txt" }) {
		for line in f.split("\n") {
			let x: isize = line.split(" ").nth(0).unwrap()[2..].split(",").nth(0).unwrap().parse::<isize>().unwrap();
			let y: isize = line.split(" ").nth(0).unwrap()[2..].split(",").nth(1).unwrap().parse::<isize>().unwrap();
			let vx: isize = line.split(" ").nth(1).unwrap()[2..].split(",").nth(0).unwrap().parse::<isize>().unwrap();
			let vy: isize = line.split(" ").nth(1).unwrap()[2..].split(",").nth(1).unwrap().parse::<isize>().unwrap();
			res.push(Robot { x, y, vx, vy });
		}
	} else {
		println!("couldn't find input file!");
		std::process::exit(1);
	}
	return res;
}

fn part1(data: &Vec<Robot>) -> usize {
	const W: isize = 101;
	const H: isize = 103;
	const STEPS: isize = 100;
	let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
	data.iter().map(|r| {
		return Robot {
			x: (r.x + r.vx * STEPS).rem_euclid(W),
			y: (r.y + r.vy * STEPS).rem_euclid(H),
			vx: r.vx,
			vy: r.vy
		};
	}).for_each(|r| {
		if r.x > W / 2 && r.y > H / 2 {  q1 += 1; }
		else if r.x < W / 2 && r.y > H / 2 { q2 += 1; }
		else if r.x < W / 2 && r.y < H / 2 { q3 += 1; }
		else if r.x > W / 2 && r.y < H / 2 { q4 += 1; }
	});
	return q1 * q2 * q3 * q4;
}

fn part2(data: &Vec<Robot>) -> usize {
	// well this is something
	const W: isize = 101;
	const H: isize = 103;
	for steps in 1..10000 {
		let new: Vec<Robot> = data.iter().map(|r| {
			return Robot {
				x: (r.x + r.vx * steps).rem_euclid(W),
				y: (r.y + r.vy * steps).rem_euclid(H),
				vx: r.vx,
				vy: r.vy
			};
		}).collect();
		let mut map = [['.'; W as usize]; H as usize];
		new.iter().for_each(|r| map[r.y as usize][r.x as usize] = '#');
		for l in 0..map.len() {
			// this required some manual checking lol
			if let Some(_) = map[l].iter().collect::<String>().find("###########") { return steps as usize; }
		}
	}
	return 0;
}

pub fn run() {
	use std::time::Instant;
	let data: Vec<Robot> = gather_input(false);
	let mut start: Instant = Instant::now();
	println!("part 1 answer: {}\ntook {:?}", part1(&data), Instant::now().duration_since(start));
	start = Instant::now();
	println!("part 2 answer: {}\ntook {:?}", part2(&data), Instant::now().duration_since(start));
}