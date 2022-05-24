use crate::runtime::question::Question;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Context {
	pub questions: Vec<Question>,
	pub sections: HashMap<String, Self>,
}

impl Context {
	pub fn new() -> Self {
		Self {
			questions: Vec::new(),
			sections: HashMap::new(),
		}
	}

	pub fn add_section(&mut self, name: String, context: Self) {
		self.sections.insert(name, context);
	}

	pub fn has_section(&mut self, name: String) -> bool {
		self.sections.contains_key(&name)
	}
}