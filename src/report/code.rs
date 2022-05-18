use num_enum::TryFromPrimitive;
use convert_case::{Case, Casing};

// pub trait Code {
// 	fn to_string(&self) -> String;
// }

#[repr(i16)]
#[derive(Debug, Clone, PartialEq, TryFromPrimitive)]
pub enum ErrorCode {
	_C = -100, // codes without code
	CouldNotOpen,
	CouldNotCompile,
	CannotExplainCode,
	CannotReview,
	InvalidStepNumber,

	NoError = 0,

	_L = 100, // lexical-error codes
	UnexpectedChar,
	InvalidDigit,

	_S = 200, // syntax-error codes
	ExpectedToken,
	UnexpectedToken,
	ExpectedTopLevel,
	ExpectedDeclaration,
	ExpectedExpression,
	AlreadyDefined,
	UseOfUndefined,
	DuplicateParameter,
	FailedToResModule,

	_D = 300, // disassembly-error codes
	MissingData,
	InvalidData,
	InvalidHeader,
	InvalidIndex,
	InvalidChecksum,
	ExpectedNullByte,
	NonexistentString,
}

impl ErrorCode {
	pub fn is_useful(&self) -> bool {
		self.clone() as i16 >= 0
	}

	pub fn get_name(&self) -> String {
		format!("{:?}", self).to_case(Case::Lower)
	}

	pub fn get_type(&self) -> Option<&str> {
		let rounded = (self.clone() as i16 as f32 / 100_f32).floor() * 100_f32;
		match ErrorCode::try_from(rounded as i16) {
			Err(_) => None,
			Ok(c) => match c {
				Self::_C => None,
				Self::_L => Some("lexical"),
				Self::_S => Some("syntax"),
				Self::_D => Some("disassembly"),
				_ => None
			}
		}
	}
}

#[macro_export]
macro_rules! fmt_error_msg {
	(CouldNotOpen $file:expr, $why:expr) => (format!("could not open file '{}': {}", $file, std::io::Error::from($why)));
	(CouldNotCompile $file:expr) => (format!("could not compile '{}' due to previous error", $file));
	(CannotExplainCode $code:expr) => (format!("cannot explain invalid error code {:?}", $code));
	(CannotReview $what:expr, $name:expr) => (format!("cannot review {} '{}'", $what, $name));
	(InvalidStepNumber $step:expr, $len:expr) => (format!("invalid step number '{}' not in range [1-{}]", $step, $len));
	
	(NoError) => ("there is no error, why did this appear?");

	(UnexpectedChar $chr:expr) => (format!("unexpected character {:?}", $chr));
	(InvalidDigit $chr:expr, $base:expr, $t:expr) => (format!("invalid digit {:?} in {} {}", $chr, $base, $t));

	(ExpectedToken $tok:expr) => (format!("expected token `{}`", $tok));
	(UnexpectedToken $tok:expr) => (format!("unexpected token `{}`", $tok));
	(ExpectedTopLevel) => ("expected a top-level statement");
	(ExpectedDeclaration) => ("expected a declaration");
	(ExpectedExpression) => ("expected an expression");
	(AlreadyDefined $type:tt $name:expr) => (format!("{} `{}` already defined", $type, $name));
	(UseOfUndefined $type:tt $name:expr) => (format!("use of undefined {} `{}`", $type, $name));
	(DuplicateParameter $param:expr) => (format!("duplicate parameter `{}`", $param));
	(FailedToResModule $mod:expr) => (format!("failed to resolve module `{}`", $mod));

	(MissingData) => ("missing data");
	(InvalidData) => ("invalid data");
	(InvalidHeader) => ("invalid header");
	(InvalidChecksum) => ("invalid checksum");
	(InvalidIndex $of:expr) => (format!("invalid {} index", $of));
	(ExpectedNullByte) => ("expected null-byte");
	(NonexistentString) => ("use of nonexistent string");
}