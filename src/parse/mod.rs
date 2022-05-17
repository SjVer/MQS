pub mod ast;
pub mod context;

use crate::lex::{Lexer, TokenIter, token::*};
// use crate::report::code::ErrorCode;
// use crate::report::{error, Report};
pub use context::Context;

pub struct Parser<'a> {
	context: Context,

	had_error: bool,
	panic_mode: bool,

	tokens: TokenIter<'a>,
}

// token stuff
impl Parser<'_> {
	fn matches(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, type_: &TokenType, message: &str) -> Result<Token> {
        if self.check(type_) {
            Ok(self.advance())
        } else {
            crate::error_at_token(&self.peek(), message);
            Err(anyhow!("Parse error"))
        }
    }

    fn check(&self, type_: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().type_ == type_
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().type_ == Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}

// public stuff
impl Parser<'_> {
	pub fn new() -> Self {
		Self {
			context: Context::new(),
			had_error: false,
			panic_mode: false,
			tokens: vec![].iter(),
		}
	}

	pub fn parse(&mut self, tokens: TokenIter) -> Context {

		self.advance();
		
		
		// return context
		self.context.clone()
	}
}