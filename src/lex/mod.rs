pub mod span;
pub mod token;
pub mod source;

use span::{Span, Location};
use token::{Token, TokenKind::{self, *}};
use source::Source;

pub struct Lexer<'a> {
	source: Source<'a>,
	start_offset: usize,
	current_offset: usize,
	
	line: u32,
	column: u32,
	filename: String,
}

// private stuff
impl<'a> Lexer<'a> {

	fn at_end(&self) -> bool {
		self.source.at(self.current_offset) == '\0'
	}

	fn advance(&mut self) -> char {
		self.column += 1;
		self.current_offset += 1;
		self.source.at(self.current_offset - 1)
	}
	fn peek(&self) -> char {
		self.source.at(self.current_offset)
	}
	fn peek_at(&self, offset: usize) -> char {
		if self.at_end() || self.source.buff.len() - self.current_offset <= offset {
			'\0'	
		} else {
			self.source.at(self.current_offset + offset)
		}
	}

	fn make_token(&self, kind: TokenKind) -> Token {
		Token {
			kind,
			span: Span {
				start: Location {
					file: self.filename.clone(),
					line: Some(self.line),
					column: Some(self.column),
				},
				length: self.current_offset - self.start_offset,
			}
		}
	}
	fn error_token(&self, message: String) -> Token {
		Token {
			kind: Error(message),
			span: Span {
				start: Location {
					file: self.filename.clone(),
					line: Some(self.line),
					column: Some(self.column),
				},
				length: self.current_offset - self.start_offset,
			}
		}
	}

	fn skip_ignored(&mut self) {
		loop {
			match self.peek() {
				' ' | '\r' | '\t' => { self.advance(); },
				'\n' => {
					self.line += 1;
					self.column = 0;
					self.advance();
					continue;
				},
				'-' => {
					if self.peek_at(1) == '-' {
						// comment
						
						self.advance(); // skip first '-'
						self.advance(); // skip second '-'
						
						if self.peek() == '*' {
							// block
							loop {
								if self.peek() == '*'
								&& self.peek_at(1) == '-'
								&& self.peek_at(2) == '-' {
									self.advance();
									self.advance();
								}
								else if self.at_end() { return; }
								else if self.peek() == '\n' {
									self.line += 1;
									self.column = 0;
								}

								println!("sc: {:?}", self.peek());
								self.advance();
							}
						} else {
							// line
							while self.peek() != '\n' && !self.at_end() {
								self.advance();
							}
						}
					}
					else { return; }
				},
				_ => return
			}
		}
	}
}
	
// public stuff
impl<'a> Lexer<'a> {
	pub fn new(filename: String, source: &'a str) -> Self {
		Lexer{
			source: Source { buff: source },
			start_offset: 0,
			current_offset: 0,

			line: 1,
			column: 1,
			filename,
		}
	}

	pub fn next(&mut self) -> Token {
		self.skip_ignored();

		self.start_offset = self.current_offset;

		if self.at_end() { return self.make_token(EOF); }

		let c = self.advance();

		// if c.is_alphabetic() { return self.identifier(); }
		// if c.is_numeric() { return self.number(); }

		if let Some(kind) = TokenKind::from_char(c) {
			return self.make_token(kind);
		} else if let Some(kind) = TokenKind::from_chars(c, self.peek()) {
			self.advance();
			return self.make_token(kind);
		}
		
		self.error_token(format!("unexpected character {:?}", c))
	}
}