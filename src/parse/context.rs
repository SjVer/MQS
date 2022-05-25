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

	pub fn get_section(&mut self, name: String) -> Option<&mut Context> {
		self.sections.get_mut(&name)
	}

	pub fn set_variable(&mut self, name: String, expr: ExprNode) {
		self.variables.insert(name, expr);
	}

	pub fn get_variable(&self, name: String) -> Option<&ExprNode> {
		self.variables.get(&name)
	}
}