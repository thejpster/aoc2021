fn main() {
	let steps = advent2021::load("./day3.input", |x| x.to_owned());

	let mut counts = [0i32; 12];
	for step in steps.iter() {
		for (idx, ch) in step.chars().enumerate() {
			if ch == '1' {
				counts[idx] += 1;
			}
		}
	}

	let mut epsilon = 0;
	let mut gamma = 0;
	for (idx, count) in counts.iter().enumerate() {
		println!("Count {} is {}/{}", idx, count, steps.len() / 2);
	}
	println!();

	for count in counts.iter() {
		gamma *= 2;
		epsilon *= 2;
		if *count >= 500 {
			gamma |= 1;
		} else {
			epsilon |= 1;
		}
	}
	println!("{}", gamma * epsilon);
	println!();

	let mut oxygen_subset = steps.clone();
	let mut co2_subset = steps.clone();
	let mut oxygen_result = 0;
	let mut co2_result = 0;
	for bit in 0..12 {
		let o_counts = find_counts(&oxygen_subset);
		println!("o2_counts {:?}", o_counts);
		oxygen_subset = oxygen_subset
			.iter()
			.filter(|x| {
				x.chars().nth(bit).unwrap()
					== if (o_counts[bit] * 2) >= oxygen_subset.len() as i32 {
						'1'
					} else {
						'0'
					}
			})
			.cloned()
			.collect();
		if oxygen_subset.len() == 1 {
			println!("{:?}", oxygen_subset);
			oxygen_result = bin_to_dec(&oxygen_subset[0]);
			break;
		}
	}
	for bit in 0..12 {
		let co2_counts = find_counts(&co2_subset);
		println!("co2_counts {:?}", co2_counts);
		co2_subset = co2_subset
			.iter()
			.filter(|x| {
				x.chars().nth(bit).unwrap()
					== if (co2_counts[bit] * 2) >= co2_subset.len() as i32 {
						'0'
					} else {
						'1'
					}
			})
			.cloned()
			.collect();
		if co2_subset.len() == 1 {
			println!("{:?}", co2_subset);
			co2_result = bin_to_dec(&co2_subset[0]);
			break;
		}
	}
	println!("{}", oxygen_result * co2_result);
}

fn bin_to_dec(input: &str) -> i32 {
	let mut total = 0;
	for inp in input.chars() {
		total *= 2;
		if inp == '1' {
			total |= 1;
		}
	}
	total
}

fn find_counts(bin_list: &[String]) -> [i32; 12] {
	let mut counts = [0i32; 12];
	for step in bin_list.iter() {
		for (idx, ch) in step.chars().enumerate() {
			if ch == '1' {
				counts[idx] += 1;
			}
		}
	}
	counts
}
