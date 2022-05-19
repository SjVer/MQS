pub mod ast;
pub mod astprinter;
pub mod context;
mod apply;
use ast::{ASTNode, ASTItem, Literal};
use apply::{Path, PathPrefix, STDLIB_DIR};
use crate::{
	SOURCES,
	report::{error, Report},
	lex::{Lexer, token::{*, TokenKind::*}},
	new_formatted_error
};
pub use context::Context;

type PResult<T> = Result<T, Report>;

pub struct Parser {
	context: Context,

	had_error: bool,

	// skip_newlines: bool,
	next_token: usize,
	tokens: Vec<Token>,
}

macro_rules! b {
	($what:expr) => (Box::new($what));
}
macro_rules! new_node {
	($tok:expr => $kind:ident @s $($field:tt)*) => {
		ASTNode { token: $tok, item: ASTItem::$kind { $($field)* } }
	};
	($tok:expr => $kind:ident @t $($field:tt)*) => {
		ASTNode { token: $tok, item: ASTItem::$kind ( $($field)* ) }
	};
}

// token stuff
impl Parser {
	fn matches(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, kind: TokenKind, what: impl ToString) -> PResult<Token> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
			println!("{:?}", self.peek().kind);
			Err(new_formatted_error!(ExpectedToken what.to_string())
				.with_quote(self.peek().span, Some("unexpected token here"))
			)
        }
    }

	/*
	fn consume_newline(&mut self, after: impl ToString) -> PResult<()> {
		self.skip_newlines = false;
		let consumed = self.consume(Newline, "newline");
		self.skip_newlines = true;

		if let Err(mut report) = consumed {
			report.message += &format!(" after {}", after.to_string());
			self.skip_newlines = true;
			return Err(report);
		}

		Ok(())
	}
	*/

    fn check(&mut self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().kind == kind
        }
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
			self.next_token += 1;

			if let Error(code, msg, fake) = self.peek().kind {
				error(msg, Some(code))
					.with_quote(self.peek().span, None::<String>)
					.dispatch();
				
				if let Some(fake) = fake {
					self.tokens[self.next_token] = *fake;
				}
			}
		}

		// if self.skip_newlines && self.current().kind == Newline {
			// self.advance()
		// } else {
			self.current()
		// }
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().kind == TokenKind::EOF
    }

    fn peek(&mut self) -> Token {
		self.tokens[self.next_token].clone()
    }

    fn current(&mut self) -> Token {
		self.tokens[self.next_token - 1].clone()
    }

    fn synchronize(&mut self, top_level: bool) {
    	self.advance();

    	while !self.is_at_end() {
    		// if self.current().kind == Newline { return; }

            match self.peek().kind {

            	// top-level only
                Apply | Identifier if top_level => { return; }

                // declaration stuff
                Variable | Function | Theorem | Conclusion | Question => { return; }

                _ => {},
            }

            self.advance();
    	}
    }
}

// top-level stuff
impl Parser {
	fn top_level(&mut self) -> PResult<()> {
		match self.advance().kind {
			Apply => self.apply(),
			Question => self.question() ,
			_ => Err(new_formatted_error!(ExpectedTopLevel)
					.with_quote(self.current().span.clone(), None::<String>)
				)
		}
	}

	fn apply(&mut self) -> PResult<()> {
		// get optional prefix
		let mut path = Path::new(
			if self.matches(&[Divide]) { PathPrefix::Root }
			else if self.matches(&[Tilde]) { PathPrefix::Home }
			else if self.matches(&[Dot]) { PathPrefix::Work }
			else { PathPrefix::None }
		);

		// consume '/' if prefix was given
		if path.has_prefix() { self.consume(Divide, "/")?; }

		// get segments
		loop {
			self.consume(Identifier, "identifier")?;
			path.append(self.current().span.get_part().unwrap_or(""));

			if !self.matches(&[Divide]) { break; }
		}

		// find file
		let fspath = match path.find_file() {
			Ok(p) => p,
			Err(why) => { return Err(new_formatted_error!(CannotApply path.to_string(), why)); },
		};

		// get source
		let src = match SOURCES!().new_source(fspath.clone()) {
			Ok(src) => src,
			Err(mut report) => {
				if !path.has_prefix() {
					report.message = report.message.replace(STDLIB_DIR, "");
				}
				return Err(report);
			},
		};

		// lex and parse
		let tokens = Lexer::new(fspath, src).lex();
        if let Ok(c) = Self::new().parse(path.to_string(), tokens) {
	        self.context.add_section(path.get_ident(), c);
        }
		
		Ok(())
	}

	fn question(&mut self) -> PResult<()> {
		let ident = if self.matches(&[Identifier]) {
			self.current().span.get_part().unwrap().to_string()
		} else {
			self.context.questions.len().to_string()
		};

		// TODO: parameters

		self.consume(Define, ":=")?;

		self.expression()?;

		Ok(())
	}
}

// expression stuff
impl Parser {
	fn expression(&mut self) -> PResult<ASTNode> {
		self.equality()
	}

	fn equality(&mut self) -> PResult<ASTNode> {
		let mut expr = self.term()?;

		while self.matches(&[Equals, NotEquals]) {
			let tok = self.current();
			let rhs = self.term()?;
			expr = new_node!(tok => Equality @s lhs: b!(expr), rhs: b!(rhs) );
		}

		Ok(expr)
	}

	fn term(&mut self) -> PResult<ASTNode> {
		let mut expr = self.factor()?;

		while self.matches(&[Plus, Minus]) {
			let tok = self.current();
			let rhs = self.factor()?;
			expr = new_node!(tok => Term @s lhs: b!(expr), rhs: b!(rhs) );
		}

		Ok(expr)
	}

	fn factor(&mut self) -> PResult<ASTNode> {
		let mut expr = self.power()?;

		while self.matches(&[Multiply, Divide]) {
			let tok = self.current();
			let rhs = self.power()?;
			expr = new_node!(tok => Factor @s lhs: b!(expr), rhs: b!(rhs) );
		}

		Ok(expr)
	}

	fn power(&mut self) -> PResult<ASTNode> {
		let mut expr = self.unary()?;

		while self.matches(&[Power]) {
			let tok = self.current();
			let rhs = self.power()?;
			expr = new_node!(tok => Power @s base: b!(expr), power: b!(rhs) );
		}

		Ok(expr)
	}

	fn unary(&mut self) -> PResult<ASTNode> {
		if self.matches(&[Minus]) {
			let tok = self.current();
			let expr = self.unary()?;
			Ok(new_node!(tok => Unary @t b!(expr)))
		} else {
			self.primary()	
		}
	}

	fn primary(&mut self) -> PResult<ASTNode> {
		let token = self.peek();

		if self.matches(&[Integer]) {
			let mut text = token.span.get_part().unwrap_or("0").to_string();
			// let intval = text.parse::<u64>().unwrap();

			let base = if text.len() >= 2 {
				match text.chars().nth(1).unwrap() {
					'b' | 'B' => { text = text[2..].to_string(); 2 },
					'c' | 'C' => { text = text[2..].to_string(); 7 },
					'x' | 'X' => { text = text[2..].to_string(); 16 },
					_ => 10,
				}
			} else { 10 };
			
			let intval= u64::from_str_radix(&text, base);
			Ok(new_node!(token => Literal @t Literal::Integer(intval.unwrap_or(0))))
		}
		else if self.matches(&[Float]) {
			let floatval = token.span.get_part().unwrap_or("0.0")
				.parse::<f64>().unwrap();
			Ok(new_node!(token => Literal @t Literal::Float(floatval)))
		}
		else if self.matches(&[LeftParen]) {
			let expr = self.expression()?;
			self.consume(TokenKind::RightParen, "(")?;
			Ok(new_node!(token => Grouping @t b!(expr)))
		}
		
		else /* expected expression */ {
			Err(new_formatted_error!(ExpectedExpression)
				.with_quote(self.peek().span, None::<String>)
			)
		}
	}
}

// public stuff
impl Parser {
	pub fn new() -> Self {
		Self {
			context: Context::new(),
			had_error: false,
			// skip_newlines: true,
			next_token: 0,
			tokens: vec![],
		}
	}

	pub fn parse(&mut self, filename: String, tokens: Vec<Token>) -> PResult<Context> {
		self.tokens = tokens;
		self.next_token = 0;

		while !self.is_at_end() {
			match self.top_level() {
				Err(report) => {
					report.dispatch();
					self.synchronize(true);
					self.had_error = true;
				},
				Ok(_) => (),
			};
		}
		
		// return context
		if self.had_error {
			Err(new_formatted_error!(CouldNotCompile &filename))
		} else {
			Ok(self.context.clone())
		}
	}
}