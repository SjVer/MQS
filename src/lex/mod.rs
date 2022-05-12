pub mod span;
pub mod token;
pub mod source;

use span::{Span, Location};
use token::{Token, TokenKind::{self, *}};
use source::Source;
use crate::{fmt_error_msg, deref_source};
use crate::report::code::ErrorCode;

pub struct Lexer {
	source: *const Source,
	start_offset: usize,
	current_offset: usize,
	
	line: usize,
	column: usize,
	filename: String,
}


macro_rules! formatted_error_token {
	($self:ident $code:ident $($arg:tt)*) => {
		$self.error_token(
			ErrorCode::$code,
			fmt_error_msg!($code $($arg)*),
			None
		)
	};
	($kind:expr => $self:ident $code:ident $($arg:tt)*) => {
		$self.error_token(
			ErrorCode::$code,
			fmt_error_msg!($code $($arg)*),
			Some($kind)
		)
	};
}

// private stuff
impl Lexer {

	fn at_end(&self) -> bool {
		//
		deref_source!(self).at(self.current_offset) == '\0'
	}

	fn advance(&mut self) -> char {
		self.column += 1;
		self.current_offset += 1;
		deref_source!(self).at(self.current_offset - 1)
	}
	fn peek(&self) -> char {
		// return current char
		deref_source!(self).at(self.current_offset)
	}
	fn peek_at(&self, offset: usize) -> char {
		//
		deref_source!(self).at(self.current_offset + offset)
	}

	fn make_token(&self, kind: TokenKind) -> Token {
		Token {
			kind,
			span: Span {
				start: Location {
					file: self.filename.clone(),
					line: Some(self.line),
					column: Some(self.column),
					source: self.source.clone(),
				},
				length: self.current_offset - self.start_offset,
			}
		}
	}
	fn error_token(&self, code: ErrorCode, message: impl ToString, kind: Option<TokenKind>) -> Token {
		let kind = match kind {
			Some(kind) => Some(Box::new(kind)),
			None => None,
		};

		Token {
			kind: Error(code, message.to_string(), kind),
			span: Span {
				start: Location {
					file: self.filename.clone(),
					line: Some(self.line),
					column: Some(self.column),
					source: self.source.clone(),
				},
				length: self.current_offset - self.start_offset,
			}
		}
	}

	fn number(&mut self, first: char) -> Token {
		let base = match (first, self.peek()) {
			('0', 'b' | 'B') => { self.advance(); 2 },
			('0', 'c' | 'C') => { self.advance(); 7 },
			('0', 'x' | 'X') => { self.advance(); 16 },
			_ => 10,
		};

		let mut kind = Integer;

		while self.peek().is_alphanumeric() { self.advance(); }
		if base == 10 && self.peek() == '.' {
			kind = Float;
			self.advance();
			while self.peek().is_alphanumeric() { self.advance(); }
		}

		// validate digits
		let start = if base == 10 { self.start_offset } else { self.start_offset + 2 };

		for c in deref_source!(self).slice(start, self.current_offset).chars() {
			// c is digit or '.' if float
			if !c.is_digit(base) && (if kind == Float { c != '.' } else { true }) {
				// invalid digit!
				let base_str = match base {
					2 => "binary",
					7 => "octal",
					10 => "decimal",
					16 => "hexadecimal",
					_ => unreachable!(),
				};

				return formatted_error_token!(kind =>
					self InvalidDigit c, base_str,
					if kind == Integer { "integer" } else { "float" }
				);
			}
		}

		self.make_token(kind)
	}

	fn skip_ignored(&mut self) {
		'base: loop {
			match self.peek() {
				' ' | '\r' | '\t' => { self.advance(); },
				'\n' => {
					self.advance();
					self.line += 1;
					self.column = 0;
					continue;
				},
				'-' => {
					if self.peek_at(1) == '-' {
						// comment
						
						self.advance(); // skip first '-'
						self.advance(); // skip second '-'
						
						if self.peek() == '*' {
							// block
							self.advance(); // skip '*'

							loop {
								if self.peek() == '*'
								&& self.peek_at(1) == '-'
								&& self.peek_at(2) == '-' {
									self.advance();
									self.advance();
									self.advance();
									continue 'base;
								}
								else if self.at_end() { return; }
								else if self.advance() == '\n' {
									self.line += 1;
									self.column = 0;
								}
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
impl Lexer {
	pub fn new(filename: String, source: *const Source) -> Self {
		Lexer{
			source,
			start_offset: 0,
			current_offset: 0,

			line: 1,
			column: 0,
			filename,
		}
	}

	pub fn next(&mut self) -> Token {
		self.skip_ignored();

		self.start_offset = self.current_offset;

		if self.at_end() {
			self.advance();
			return self.make_token(EOF);
		}

		let c = self.advance();

		// if c.is_alphabetic() { return self.identifier(); }
		if c.is_numeric() { return self.number(c); }

		if let Some(kind) = TokenKind::from_chars(c, self.peek()) {
			self.advance();
			return self.make_token(kind);
		} else if let Some(kind) = TokenKind::from_char(c) {
			return self.make_token(kind);
		}
		
		formatted_error_token!(self UnExpectedChar c)
	}
}