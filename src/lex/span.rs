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
	pub fn get_line_before(&self) -> Option<&str> {
		match self.start.line {
			Some(line) => if line <= 1 {
				None
			} else {
				deref_source!(self.start).slice_line(line - 1)
			},
			None => None,
		}
	}
	
	pub fn get_line(&self) -> Option<&str> {
		match self.start.line {
			Some(line) => deref_source!(self.start).slice_line(line),
			None => None,
		}
	}

	pub fn get_part_before(&self) -> Option<&str> {
		// get full line or return None
		let line = self.get_line();
		if let None = line { return None; }

		let end = self.start.column.unwrap_or(1) - 1;

		// unwrap line or return None
		match line {
			Some(line) => Some(&line[..end]),
			None => None,
		}
	}

	pub fn get_part_after(&self) -> Option<&str> {
		// get full line or return None
		let line = self.get_line();
		if let None = line { return None; }

		let start = self.start.column.unwrap_or(1) + 1 + self.length;

		// unwrap line or return None
		match line {
			Some(line) => Some(&line[start..]),
			None => None,
		}
	}

	pub fn get_part(&self) -> Option<&str> {
		// get full line or return None
		let line = self.get_line();
		if let None = line { return None; }

		let start = self.start.column.unwrap_or(1) + 1;
		let end = start + self.length;

		// unwrap line or return None
		match line {
			Some(line) => Some(&line[start..end]),
			None => None,
		}
	}
}