mod report;
pub mod code;

pub use report::Report;
use report::Severity;
use crate::info::report::*;
use yansi::Color;

pub fn error(message: impl ToString, code: Option<code::ErrorCode>) -> Report {
	let label;
	
	if matches!(code, Some(_)) && matches!(code.as_ref().unwrap().get_type(), Some(_)) {
		label = format!("{} {}", code.as_ref().unwrap().get_type().unwrap(), ERROR_LABEL);
	} else {
		label = String::from(ERROR_LABEL);
	}

	Report{
		label,
		message: message.to_string(),

		color: Color::Red,
		severity: Severity::Error,
		code,

		quote: None,
		sub_quotes: vec![],
		notes: vec![],
	}
}

#[macro_export]
macro_rules! new_formatted_error {
	($code:ident $($arg:tt)*) => {
		crate::report::error(
			crate::fmt_error_msg!($code $($arg)*),
			Some(crate::report::code::ErrorCode::$code)
		)
	};
}

pub fn warning(message: impl ToString) -> Report {
	Report{
		label: String::from(WARNING_LABEL),
		message: message.to_string(),

		color: Color::Yellow,
		severity: Severity::Warning,
		code: None,

		quote: None,
		sub_quotes: vec![],
		notes: vec![],
	}
}