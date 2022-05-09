use crate::report::dispatch as d;
use crate::info::report::ERROR_LABEL;
use crate::lex::span::Span;

use yansi::Color;

pub fn dispatch_simple(code: u8, message: &String) -> () {
	d::dispatch_header(Color::Red, ERROR_LABEL, code, message);
}

pub fn dispatch_snippet(code: u8, message: &String, span: &Span) -> () {
	d::dispatch_header(Color::Red, ERROR_LABEL, code, message);
	d::dispatch_snippet(Some(Color::Red), span);
}