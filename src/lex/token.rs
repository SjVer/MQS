use super::span::Span;
use crate::report::code::ErrorCode;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
	// definition
	Conclusion,
	Function,
	Variable,
	Question,
	Theory,
	Apply,

	// punctuation
	LeftBrace,
	RightBrace,
	LeftParen,
	RightParen,
	Define,
	Colon,
	Dot,

	// theory operators
	Divisible,

	// expression operators
	Equal,
	NotEqual,
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
	Newline,
	EOF,
	Error(ErrorCode, String, Option<Box<TokenKind>>),
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

			('/', '=') => __somekind!(NotEqual),
			
			_ => None
		}
	}

	pub fn from_char(c: char) -> Option<Self> {
		match c {
			
			'&' => __somekind!(Conclusion),
			'@' => __somekind!(Function),
			'$' => __somekind!(Variable),
			'?' => __somekind!(Question),
			'!' => __somekind!(Theory),
			'#' => __somekind!(Apply),

			'{' => __somekind!(LeftBrace),
			'}' => __somekind!(RightBrace),
			'(' => __somekind!(LeftParen),
			')' => __somekind!(RightParen),
			':' => __somekind!(Colon),
			'.' => __somekind!(Dot),

			'%' => __somekind!(Divisible),

			'=' => __somekind!(Equal),
			'+' => __somekind!(Plus),
			'-' => __somekind!(Minus),
			'*' => __somekind!(Multiply),
			'/' => __somekind!(Divide),
			'^' => __somekind!(Power),

			_ => None
		}
	}
}

pub struct Token {
	pub kind: TokenKind,
	pub span: Span
}