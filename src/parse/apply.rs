#[derive(PartialEq)]
pub enum PathPrefix {
	Root, // '/'
	Home, // '~'
	Working, // '.'
	None,
}

pub struct Path {
	prefix: PathPrefix,
	segments: Vec<String>,
}

impl Path {
	pub fn new(prefix: PathPrefix) -> Self {
		Self {
			prefix,
			segments: Vec::new(),
		}
	}

	pub fn has_prefix(&self) -> bool {
		self.prefix != PathPrefix::None
	}

	pub fn append(&mut self, segment: impl ToString) {
		self.segments.push(segment.to_string());
	}
}

impl ToString for Path {
	fn to_string(&self) -> String {
		let prefix = match &self.prefix {
			PathPrefix::Root => "//",
			PathPrefix::Home => "~/",
			PathPrefix::Working => "./",
			PathPrefix::None => "",
		}.to_string();

		prefix + &self.segments.join("/")
	}
}