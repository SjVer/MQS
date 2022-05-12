use crate::deref_source;
use super::source::Source;

pub struct Location {
	pub file: String,
	pub line: Option<usize>,
	pub column: Option<usize>,

	pub source: *const Source,
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

impl Span {
	pub fn get_part_before(&self) -> Option<&str> {
		match self.start.line {
			Some(line) => deref_source!(self.start).slice_line(line),
			None => None,
		}
	}
}