const INCLUDE_DIAGONALS: bool = true;

#[derive(Clone)]
struct Line {
	x1: i32,
	x2: i32,
	y1: i32,
	y2: i32,
}

impl std::fmt::Debug for Line {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{},{} -> {},{}", self.x1, self.y1, self.x2, self.y2)
	}
}

impl Line {
	fn max_x(&self) -> i32 {
		self.x1.max(self.x2)
	}

	fn max_y(&self) -> i32 {
		self.y1.max(self.y2)
	}

	fn compare(a: i32, b: i32) -> i32 {
		if a > b {
			-1
		} else if b > a {
			1
		} else {
			0
		}
	}

	fn points(&self) -> Vec<(i32, i32)> {
		let delta_x = Self::compare(self.x1, self.x2);
		let delta_y = Self::compare(self.y1, self.y2);

		let mut result = Vec::new();

		let mut x = self.x1;
		let mut y = self.y1;
		loop {
			result.push((x, y));
			if x == self.x2 && y == self.y2 {
				break;
			}
			x += delta_x;
			y += delta_y;
		}
		result
	}
}

impl TryFrom<&str> for Line {
	type Error = ();

	fn try_from(input: &str) -> Result<Line, ()> {
		let parts: Vec<i32> = input
			.replace(" -> ", ",")
			.split(",")
			.map(|x| x.parse::<i32>().unwrap())
			.collect();
		let delta_x = (parts[0] - parts[2]).abs();
		let delta_y = (parts[1] - parts[3]).abs();
		if delta_x == 0 || delta_y == 0 || (INCLUDE_DIAGONALS && delta_x == delta_y) {
			Ok(Line {
				x1: parts[0],
				x2: parts[2],
				y1: parts[1],
				y2: parts[3],
			})
		} else {
			Err(())
		}
	}
}

fn main() {
	let mut lines: Vec<Line> = Vec::new();
	advent2021::load("./day5.input", |x| {
		if let Ok(line) = x.try_into() {
			lines.push(line);
		}
	});
	println!("Lines: {:?}", lines);

	let max_x = lines.iter().map(|line| line.max_x()).max().unwrap();
	let max_y = lines.iter().map(|line| line.max_y()).max().unwrap();

	println!("Bounds {},{}", max_x, max_y);

	let mut point_map: std::collections::HashMap<(i32, i32), i32> =
		std::collections::HashMap::new();

	for line in lines.iter() {
		println!("Line {:?}", line);
		for point in line.points().iter() {
			println!("  @ {:?}", point);
			let entry = point_map.entry(*point).or_insert(0);
			*entry = *entry + 1;
		}
		//print_map(&point_map, max_x, max_y);
	}

	let mut total = 0;
	for (point, count) in point_map.iter() {
		if *count >= 2 {
			println!("{},{}", point.0, point.1);
			total += 1;
		}
	}
	println!("{}", total);
}

fn print_map(map: &std::collections::HashMap<(i32, i32), i32>, max_x: i32, max_y: i32) {
	for y in 0..=max_y {
		for x in 0..=max_x {
			if let Some(count) = map.get(&(x, y)) {
				print!("{}", count);
			} else {
				print!(".");
			}
		}
		println!();
	}
}
