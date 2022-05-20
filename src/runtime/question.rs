use crate::parse::ast::TheoryNode;

#[derive(Clone)]
pub struct Question {
	pub name: String,
	pub theory: TheoryNode,
}