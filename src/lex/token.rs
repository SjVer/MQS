use super::span::Span;
use crate::report::code::ErrorCode;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
	// definition
	Conclusion,
	Function,
	Variable,
	Question,
	Theorem,
	Apply,

	// punctuation
	LeftBrace,
	RightBrace,
	LeftParen,
	RightParen,
	Define,
	Colon,
	Dot,
	DoubleDot,

	// theory operators
	Or,
	XOr,
	And,
	Matches,
	NotMatches,
	DefEquals,
	DefNotEquals,
	RoughlyEquals,
	Greater,
	GreaterEqual,
	Lesser,
	LesserEqual,
	Divisible,
	Exists,

	// expression operators
	Equals,
	NotEquals,
	Plus,
	Minus,
	Multiply,
	Divide,
	Power,

	// literals
	Identifier,
	Type,
	Integer,
	Float,
	
	// misc.
	// Newline,
	EOF,
	Error(ErrorCode, String, Option<Box<Token>>),
}

macro_rules! __somestr {
	($s:expr) => { Some(String::from($s)) };
}

macro_rules! __somekind {
	($kind:ident) => {
		Some(Self::$kind)
	};
}

impl TokenKind {
	pub fn from_chars(c1: char, c2: char) -> Option<Self> {
		match (c1, c2) {

			(':', '=') => __somekind!(Define),

			('.', '.') => __somekind!(DoubleDot),

			('!', '|') => __somekind!(XOr),
			('=', '=') => __somekind!(Matches),
			('<', '>') => __somekind!(NotEquals),
			('<', '!') => __somekind!(DefEquals),
			('!', '=') => __somekind!(DefNotEquals),
			('>', '=') => __somekind!(GreaterEqual),
			('<', '=') => __somekind!(LesserEqual),
			('?', '?') => __somekind!(Exists),

			('/', '=') => __somekind!(NotEquals),
			
			_ => None
		}
	}

	pub fn from_char(c: char) -> Option<Self> {
		match c {
			
			'&' => __somekind!(Conclusion),
			'@' => __somekind!(Function),
			'$' => __somekind!(Variable),
			'?' => __somekind!(Question),
			'!' => __somekind!(Theorem),
			'#' => __somekind!(Apply),

			'{' => __somekind!(LeftBrace),
			'}' => __somekind!(RightBrace),
			'(' => __somekind!(LeftParen),
			')' => __somekind!(RightParen),
			':' => __somekind!(Colon),
			'.' => __somekind!(Dot),

			'|' => __somekind!(Or),
			'&' => __somekind!(And),
			'~' => __somekind!(RoughlyEquals),
			'>' => __somekind!(Greater),
			'<' => __somekind!(Lesser),
			'%' => __somekind!(Divisible),

			'=' => __somekind!(Equals),
			'+' => __somekind!(Plus),
			'-' => __somekind!(Minus),
			'*' => __somekind!(Multiply),
			'/' => __somekind!(Divide),
			'^' => __somekind!(Power),

			_ => None
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
	pub kind: TokenKind,
	pub span: Span
}