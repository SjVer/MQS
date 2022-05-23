pub mod ast;
pub mod astprinter;
pub mod context;
mod apply;
use ast::*;
use apply::{Path, PathPrefix, STDLIB_DIR};
use crate::{
	SOURCES,
	report::{error, Report, code::ErrorCode},
	lex::{Lexer, token::{*, TokenKind::*}},
	runtime::question::Question as rQuestion,
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
macro_rules! expr_node {
	($tok:expr => $kind:ident @s $($field:tt)*) => {
		ExprNode { token: $tok, item: ExprItem::$kind { $($field)* } }
	};
	($tok:expr => $kind:ident @t $($field:tt)*) => {
		ExprNode { token: $tok, item: ExprItem::$kind ( $($field)* ) }
	};
}
macro_rules! theory_node {
	($tok:expr => $kind:ident @s $($field:tt)*) => {
		TheoryNode { token: $tok, item: TheoryItem::$kind { $($field)* } }
	};
	($tok:expr => $kind:ident @t $($field:tt)*) => {
		TheoryNode { token: $tok, item: TheoryItem::$kind ( $($field)* ) }
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
		let token = self.current();

		// get optional prefix
		let mut path = Path::new(
			if self.matches(&[Divide]) { PathPrefix::Root }
			else if self.matches(&[RoughlyEquals]) { PathPrefix::Home }
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
			Err(why) => {
				return Err(
					new_formatted_error!(FailedToResolve path.to_string(), why)
						.with_quote(token.span, None::<String>)
				);
			},
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
		} else {
			new_formatted_error!(FailedToApply path.to_string())
				.with_quote(token.span, None::<String>)
				.dispatch();
			self.had_error = true;
		}
		
		Ok(())
	}

	fn question(&mut self) -> PResult<()> {
		// get ident of next available number
		let ident = if self.matches(&[Identifier]) {
			self.current().span.get_part().unwrap().to_string()
		} else {
			self.context.questions.len().to_string()
		};
		let token = self.current();

		// TODO: parameters

		// parse theory
		self.consume(Define, ":=")?;
		let th = self.theory()?;

		// add question to context and Ok
		self.context.questions.push(rQuestion{
			token,
			name: ident,
			theory: th,
		});
		Ok(())
	}
}

// theory stuff
impl Parser {
	fn theory(&mut self) -> PResult<TheoryNode> {
		self.or()
	}

	fn or(&mut self) -> PResult<TheoryNode> {
		let mut th = self.xor()?;

		while self.matches(&[Or]) {
			let tok = self.current();
			let rhs = self.xor()?;
			th = theory_node!(tok => Logical @s lhs: b!(th), rhs: b!(rhs) );
		}

		Ok(th)
	}

	fn xor(&mut self) -> PResult<TheoryNode> {
		let mut th = self.and()?;

		while self.matches(&[XOr]) {
			let tok = self.current();
			let rhs = self.and()?;
			th = theory_node!(tok => Logical @s lhs: b!(th), rhs: b!(rhs) );
		}

		Ok(th)
	}

	fn and(&mut self) -> PResult<TheoryNode> {
		let mut th = self.solveable()?;

		while self.matches(&[And]) {
			let tok = self.current();
			let rhs = self.solveable()?;
			th = theory_node!(tok => Logical @s lhs: b!(th), rhs: b!(rhs) );
		}

		Ok(th)
	}

	fn solveable(&mut self) -> PResult<TheoryNode> {
		let atom = self.atom()?;
		let tok = self.peek();

		if self.matches(&[Matches, NotMatches]) {
			let rhs = self.atom()?;
			Ok(theory_node!(tok => Match @s lhs: b!(atom), rhs: b!(rhs) ))
		}
		else if self.matches(&[DefEquals, DefNotEquals, RoughlyEquals, Greater, GreaterEqual, Lesser, LesserEqual]) {
			let rhs = self.atom()?;
			Ok(theory_node!(tok => Comparison @s lhs: b!(atom), rhs: b!(rhs) ))
		}
		else if self.matches(&[Divisible]) {
			let rhs = self.atom()?;
			Ok(theory_node!(tok => Divisible @s expr: b!(atom), divisor: b!(rhs) ))
		}
		else if self.matches(&[Exists]) {
			Ok(theory_node!(tok => Exists @t b!(atom) ))
		}

		else { /* expected theory */
			Err(new_formatted_error!(ExpectedTheory)
				.with_quote(self.peek().span, None::<String>)
			)
		}
	}

	fn atom(&mut self) -> PResult<TheoryNode> {
		let token = self.peek();

		if self.matches(&[LeftParen]) {
			let th = self.theory()?;
			self.consume(TokenKind::RightParen, ")")?;
			Ok(theory_node!(token => Grouping @t b!(th)))
		} else {
			let expr = self.expression()?;
			Ok(theory_node!(token => Expression @t expr))
		}
	}
}

// expression stuff
impl Parser {
	fn expression(&mut self) -> PResult<ExprNode> {
		self.equality()
	}

	fn equality(&mut self) -> PResult<ExprNode> {
		let mut expr = self.term()?;

		while self.matches(&[Equals, NotEquals]) {
			let tok = self.current();
			let rhs = self.term()?;
			expr = expr_node!(tok => Equality @s lhs: b!(expr), rhs: b!(rhs) );
		}

		Ok(expr)
	}

	fn term(&mut self) -> PResult<ExprNode> {
		let mut expr = self.factor()?;

		while self.matches(&[Plus, Minus]) {
			let tok = self.current();
			let rhs = self.factor()?;
			expr = expr_node!(tok => Term @s lhs: b!(expr), rhs: b!(rhs) );
		}

		Ok(expr)
	}

	fn factor(&mut self) -> PResult<ExprNode> {
		let mut expr = self.power()?;

		while self.matches(&[Multiply, Divide]) {
			let tok = self.current();
			let rhs = self.power()?;
			expr = expr_node!(tok => Factor @s lhs: b!(expr), rhs: b!(rhs) );
		}

		Ok(expr)
	}

	fn power(&mut self) -> PResult<ExprNode> {
		let mut expr = self.unary()?;

		while self.matches(&[Power]) {
			let tok = self.current();
			let rhs = self.power()?;
			expr = expr_node!(tok => Power @s base: b!(expr), power: b!(rhs) );
		}

		Ok(expr)
	}

	fn unary(&mut self) -> PResult<ExprNode> {
		if self.matches(&[Minus]) {
			let tok = self.current();
			let expr = self.unary()?;
			Ok(expr_node!(tok => Unary @t b!(expr)))
		} else {
			self.primary()	
		}
	}

	fn primary(&mut self) -> PResult<ExprNode> {
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
			Ok(expr_node!(token => Literal @t Literal::Integer(intval.unwrap_or(0))))
		}
		else if self.matches(&[Float]) {
			let floatval = token.span.get_part().unwrap_or("0.0")
				.parse::<f64>().unwrap();
			Ok(expr_node!(token => Literal @t Literal::Float(floatval)))
		}
		else if self.matches(&[LeftParen]) {
			let expr = self.expression()?;
			self.consume(TokenKind::RightParen, ")")?;
			Ok(expr_node!(token => Grouping @t b!(expr)))
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