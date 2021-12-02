
fn main()  {
	let numbers = advent2021::load("./day1.input", |x| x.parse::<u32>().unwrap());
	let mut count = 0;
	for (x,y) in numbers.iter().zip(numbers[1..].iter()) {
		if y > x {
			count += 1;
		}
	}
	println!("{}", count);

	count = 0;
	let mut window_vals = Vec::new();
	for window in numbers.windows(3) {
		window_vals.push(window[0] + window[1] + window[2])
	}
	for (x,y) in window_vals.iter().zip(window_vals[1..].iter()) {
		if y > x {
			count += 1;
		}
	}
	println!("{}", count);
}
