mod report;

use report::Report;
use crate::info::report::{ERROR_LABEL, CODE_PREFIX};
// use crate::lex::span::Span;
use yansi::Color::Red;

pub fn error(message: impl ToString, code: Option<u8>) -> Report {
	let mut label = String::from(ERROR_LABEL);

	if let Some(code) = code {
		label.push_str(std::format!("[{}{}]", CODE_PREFIX, code).as_str());
	}
	
	Report{
		label,
		color: Red,
		message: message.to_string(),
		labels: vec![],
		notes: vec![],
	}
}

// pub fn dispatch_simple(code: u8, message: &String) -> () {
// 	d::dispatch_header(Color::Red, ERROR_LABEL, code, message);
// }

// pub fn dispatch_snippet(code: u8, message: &String, span: &Span) -> () {
// 	d::dispatch_header(Color::Red, ERROR_LABEL, code, message);
// 	d::dispatch_snippet(Some(Color::Red), span);
// }