use num_enum::TryFromPrimitive;
use convert_case::{Case, Casing};

#[repr(i16)]
#[derive(Debug, Clone, PartialEq, TryFromPrimitive)]
pub enum ErrorCode {
	_C = -100, // codes without code
	CouldNotOpen,
	CouldNotCompile,

	NoError = 0,

	_L = 100, // lexical-error codes
	UnExpectedChar,
	InvalidDigit,

	_S = 200, // syntax-error codes
	ExpectedToken,
	UnExpectedToken,
	AlreadyDefined,
	UseOfUndefined,
	DuplicateParameter,
	FailedToResModule,

	_D = 300, // disassembly-error codes
	MissingData,
	InvalidData,
	InvalidIndex,
	InvalidChecksum,
	NonExistentString,
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
				Self::_C => Some("codeless"),
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
	
	(NoError) => ("there is no error, why did this appear?");

	(UnExpectedChar $chr:expr) => (format!("unexpected character {:?}", $chr));
	(InvalidDigit $chr:expr, $base:expr, $t:expr) => (format!("invalid digit {:?} in {} {}", $chr, $base, $t));

	(ExpectedToken $tok:expr) => (format!("expected token `{}`", $tok));
	(UnExpectedToken $tok:expr) => (format!("unexpected token `{}`", $tok));
	(AlreadyDefined $type:tt $name:expr) => (format!("{} `{}` already defined", $type, $name));
	(UseOfUndefined $type:tt $name:expr) => (format!("use of undefined {} `{}`", $type, $name));
	(DuplicateParameter $param:expr) => (format!("duplicate parameter `{}`", $param));
	(FailedToResModule $mod:expr) => (format!("failed to resolve module `{}`", $mod));

	(MissingData) => ("missing data");
	(InvalidData) => ("invalid data");
	(InvalidIndex $of:expr) => ("invalid {} index", $of);
	(InvalidChecksum) => ("invalid checksum");
	(NonExistentString) => ("use of nonexistent string");
}