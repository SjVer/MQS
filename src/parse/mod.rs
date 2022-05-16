pub mod ast;
pub mod context;

use crate::lex::{Lexer, token::*};
use crate::report::code::ErrorCode;
use crate::report::{error, Report};
pub use context::Context;

pub struct Parser {
	context: Context,

	had_error: bool,
	panic_mode: bool,

	lexer: Lexer,
	next_token: Token,
	current_token: Token,
}

macro_rules! err {
	($self:ident $fn:ident $($farg:expr)* => $code:ident $($earg:tt)*) => {
		$self.$fn($($farg)* crate::fmt_error_msg!($code $($earg)*), Some(ErrorCode::$code))
	};
}

// error stuff
impl Parser {
	fn error_at(&mut self, token: &Token, msg: impl ToString, code: Option<ErrorCode>) -> Report {
		self.had_error = true;
		self.panic_mode = true;

		error(msg, code)
			.with_quote(token.span, None::<String>)
	}

	// displays an error at the current token with the given message
	fn error(&mut self, msg: impl ToString, code: Option<ErrorCode>) -> Report {
		//
		self.error_at(&self.current_token, msg, code)
	}

	// displays an error at the next token with the given message
	fn error_at_next(&mut self, msg: impl ToString, code: Option<ErrorCode>) -> Report {
		//
		self.error_at(&self.next_token, msg, code)
	}
}

// token stuff
impl Parser {
	fn advance(&mut self) {
		self.current_token = self.next_token;

		loop {
			self.current_token = self.lexer.next();

			if let TokenKind::Error(c, m, _) = &self.current_token.kind {
				error(m, Some(*c))
					.with_quote(self.current_token.span, None::<String>)
					.dispatch();
			}
			else { break; }
		}
	}

	fn check(&self, kind: TokenKind) -> bool {
		self.current_token.kind == kind
	}

	fn consume(&mut self, what: impl ToString, kind: TokenKind, msg: impl ToString) -> bool {
		if self.current_token.kind == kind {
			self.advance();
			true
		} else {
			err!(self error_at_next => ExpectedToken what.to_string());
			false
		}
	}
}

// top-level
impl Parser {
}

// public stuff
impl Parser {
	pub fn new(lexer: Lexer) -> Self {
		Self {
			lexer,
			context: Context::new(),

			had_error: false,
			panic_mode: false,

			current_token: Token::empty(),
			next_token: Token::empty()}
		}
	}

	pub fn parse(&mut self) -> Context {
		while 
		
		// return context
		self.context.clone()
	}
}