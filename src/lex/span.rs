pub struct Position {
	pub file: String,
	pub line: u32,
	pub column: u32,
}

impl std::fmt::Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.line != 0 && self.column != 0 {
			write!(f, "{}:{}:{}", self.file, self.line, self.column)
		}
		else if self.line != 0 {
			write!(f, "{}:{}", self.file, self.line)
		}
		else {
			write!(f, "{}", self.file)
		}
	}
}

pub struct Span {
	pub start: Position,
	pub length: u32,
}