use super::span::Span;

#[derive(Debug, PartialEq)]
pub enum TokenKind {
	// definition
	LeftBrace,
	RightBrace,
	LeftParen,
	RightParen,

	// operators
	Plus,
	Minus,
	Multiply,
	Divide,
	Power,

	// literals
	Integer,
	Float,
	
	// misc.
	EOF,
	Error(String),
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
	pub fn source_literal(&self) -> Option<String> {
		match self {
			
			Self::LeftBrace		=> __somestr!("{"),
			Self::RightBrace	=> __somestr!("}"),
			Self::LeftParen		=> __somestr!("("),
			Self::RightParen	=> __somestr!(")"),

			Self::Plus			=> __somestr!("+"),
			Self::Minus			=> __somestr!("-"),
			Self::Multiply		=> __somestr!("*"),
			Self::Divide		=> __somestr!("/"),
			Self::Power			=> __somestr!("^"),

			Self::EOF			=> __somestr!("EOF"),
			_ => None
		}
	}

	pub fn from_char(c: char) -> Option<Self> {
		match c {
			
			'{' => __somekind!(LeftBrace),
			'}' => __somekind!(RightBrace),
			'(' => __somekind!(LeftParen),
			')' => __somekind!(RightParen),

			'+' => __somekind!(Plus),
			'-' => __somekind!(Minus),
			'*' => __somekind!(Multiply),
			'/' => __somekind!(Divide),
			'^' => __somekind!(Power),
			
			_ => None
		}
	}

	pub fn from_chars(c1: char, c2: char) -> Option<Self> {
		match c {
			
			'{' => __somekind!(LeftBrace),
			'}' => __somekind!(RightBrace),
			'(' => __somekind!(LeftParen),
			')' => __somekind!(RightParen),

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