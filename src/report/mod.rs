mod report;
pub mod code;

use report::{Report, Severity};
use crate::info::report::*;
use yansi::Color;

pub fn error(message: impl ToString, code: Option<code::ErrorCode>) -> Report {
	Report{
		label: String::from(ERROR_LABEL),
		message: message.to_string(),

		color: Color::Red,
		severity: Severity::Error,
		code,

		snippet: None,
		sub_snippet: vec![],
		notes: vec![],
	}
}

#[macro_export]
macro_rules! new_formatted_error {
	($code:ident $($arg:tt)*) => {
		report::error(fmt_error_msg!($code $($arg)*), Some(report::code::ErrorCode::$code))
	};
}

pub fn warning(message: impl ToString) -> Report {
	Report{
		label: String::from(WARNING_LABEL),
		message: message.to_string(),

		color: Color::Yellow,
		severity: Severity::Warning,
		code: None,

		snippet: None,
		sub_snippet: vec![],
		notes: vec![],
	}
}