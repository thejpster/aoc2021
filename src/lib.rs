//! # Advent 2021
//!
//! Useful library functions for writing an Advent of Code solution.

use std::io::BufRead;

pub fn load<P, F, T>(filename: P, mut parser: F) -> Vec<T>
where
	P: AsRef<std::path::Path>,
	F: FnMut(&str) -> T,
{
	let f = std::fs::File::open(filename).expect("valid file");
	let br = std::io::BufReader::new(f);
	br.lines().map(|x| parser(&x.unwrap())).collect()
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
