use crate::runtime::question::Question;
use std::collections::HashMap;

use super::ast::ExprNode;

#[derive(Clone, Debug)]
pub struct Context {
	pub variables: HashMap<String, ExprNode>,
	pub questions: Vec<Question>,
	pub sections: HashMap<String, Self>,
}

impl Context {
	pub fn new() -> Self {
		Self {
			variables: HashMap::new(),
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

	pub fn set_variable(&mut self, name: String, expr: ExprNode) {
		self.variables.insert(name, expr);
	}

	pub fn has_variable(&mut self, name: String) -> bool {
		self.variables.contains_key(&name)
	}
}