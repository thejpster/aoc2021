fn main() { 
	let steps = advent2021::load("./day2.input", |x| {
		let parts: Vec<&str> = x.split(" ").collect();
		(parts[0].to_owned(), parts[1].parse::<i32>().unwrap())
	});

	let mut aim = 0;
	let mut x = 0;
	let mut y = 0;
	for step in steps.iter() {
		println!("'{}' @ {}", step.0, step.1);
		match step.0.as_str() {
			"forward" => {
				x += step.1;
				y += aim * step.1;
			}
			"down" => {
				// y += step.1;
				aim += step.1;
			}
			"up" => {
				// y -= step.1;
				aim -= step.1;
			}
			_ => {
				panic!("Bad input {:?}", step);
			}
		}
		println!("{}, {}, {}", x, y, aim);
	}
	println!("{}, {}, {}", x, y, x * y);
}