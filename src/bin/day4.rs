#[derive(Clone, Debug)]
struct Board {
	cells: [Option<u32>; Board::NUM_ROWS * Board::NUM_PER_ROW],
	loaded_rows: usize,
	dead: bool,
}

impl Board {
	const NUM_PER_ROW: usize = 5;
	const NUM_ROWS: usize = 5;

	fn new() -> Board {
		Board {
			cells: [None; Board::NUM_ROWS * Board::NUM_PER_ROW],
			loaded_rows: 0,
			dead: false,
		}
	}

	fn mark(&mut self, value: u32) {
		for num in self.cells.iter_mut() {
			if *num == Some(value) {
				*num = None;
			}
		}
	}

	fn get(&self, row: usize, col: usize) -> Option<u32> {
		self.cells[row * Board::NUM_PER_ROW + col]
	}

	fn is_winner(&mut self) -> bool {
		if self.dead {
			return false;
		}

		// Iterate columns
		for col in 0..Board::NUM_PER_ROW {
			let mut found_value = false;
			for row in 0..Board::NUM_ROWS {
				if self.get(row, col).is_some() {
					found_value = true;
					break;
				}
			}
			if !found_value {
				self.dead = true;
				return true;
			}
		}

		// Iterate rows
		for row in 0..Board::NUM_ROWS {
			let mut found_value = false;
			for col in 0..Board::NUM_PER_ROW {
				if self.get(row, col).is_some() {
					found_value = true;
					break;
				}
			}
			if !found_value {
				self.dead = true;
				return true;
			}
		}

		false
	}

	fn total(&self) -> u32 {
		self.cells.iter().map(|x| x.unwrap_or(0)).sum()
	}

	fn load(&mut self, line: &str) -> bool {
		let numbers: Vec<u32> = line
			.replace("  ", " ")
			.trim()
			.split(" ")
			.map(|x| x.parse::<u32>().unwrap())
			.collect();
		for (idx, number) in numbers.iter().enumerate() {
			self.cells[self.loaded_rows * Board::NUM_PER_ROW + idx] = Some(*number);
		}
		if self.loaded_rows == 4 {
			true
		} else {
			self.loaded_rows += 1;
			false
		}
	}
}

impl std::fmt::Display for Board {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for (idx, number) in self.cells.iter().enumerate() {
			if let Some(n) = number {
				write!(f, "{:3} ", n)?;
			} else {
				write!(f, "  * ")?;
			}
			if [4, 9, 14, 19, 24].contains(&idx) {
				writeln!(f)?;
			}
		}
		Ok(())
	}
}

fn main() {
	let mut header = Vec::new();
	let mut boards = Vec::new();

	let mut next_board = Board::new();
	let mut has_header = false;
	advent2021::load("./day4.input", |x| {
		if !has_header {
			header = x.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
			has_header = true;
		} else if x.len() > 0 {
			if next_board.load(x) {
				println!("Board is:\n{}", next_board);
				boards.push(next_board.clone());
				next_board = Board::new();
				println!("Found {} boards", boards.len());
			}
		}
	});

	for item in header {
		for board in boards.iter_mut() {
			board.mark(item);
			if board.is_winner() {
				println!("{}", board.total() * item);
			}
		}
	}
}
