use crate::lex::Lexer;

pub mod ast;

pub struct Parser {
	lexer: Lexer
}

impl Parser {
	fn parse_(&mut self) {
			
	}
}

impl Parser {
	pub fn parse(lexer: Lexer) {
		Self {
			lexer,
		}.parse_();
	}
}