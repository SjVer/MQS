pub struct Location {
	pub file: String,
	pub line: Option<u32>,
	pub column: Option<u32>,
}

impl Location {
	pub fn to_string(&self) -> String {
		// line and col -> file:line:col
		if matches!(self.line, Some(_)) && matches!(self.column, Some(_)) {
			format!("{}:{}:{}", self.file, self.line.unwrap(), self.column.unwrap())
		}
		// just line -> file:line
		else if matches!(self.line, Some(_)) {
			format!("{}:{}", self.file, self.line.unwrap())
		}
		// nothing -> file
		else {
			format!("{}", self.file)
		}
	}
}

pub struct Span {
	pub start: Location,
	pub length: usize,
}