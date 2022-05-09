mod report;
pub mod code;

use report::Report;
use crate::info::report::*;
use yansi::Color;

use self::code::ErrorCode;

pub fn error(message: impl ToString, code: Option<code::ErrorCode>) -> Report {
	let mut label = String::from(ERROR_LABEL);

	if let Some(code) = code {
		if code >= ErrorCode::None {
			label.push_str(std::format!("[{}{:#03}]", CODE_PREFIX, code as u32).as_str());
		}
	}
	
	Report{
		label,
		color: Color::Red,
		message: message.to_string(),
		labels: vec![],
		notes: vec![],
	}
}

#[macro_export]
macro_rules! new_formatted_error {
	($code:ident $(, $arg:tt)*) => {
		report::error(fmt_error_msg!($code $($arg)*), Some(report::code::ErrorCode::$code))
	};
}

pub fn warning(message: impl ToString) -> Report {
	Report{
		label: String::from(WARNING_LABEL),
		color: Color::Yellow,
		message: message.to_string(),
		labels: vec![],
		notes: vec![],
	}
}