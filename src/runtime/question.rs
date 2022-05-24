use crate::{parse::ast::TheoryNode, lex::token::Token};

#[derive(Clone, Debug)]
pub struct Question {
	pub name: String,
	pub token: Token,
	pub theory: TheoryNode,
}