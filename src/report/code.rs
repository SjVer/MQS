#[derive(Debug, Clone, PartialEq)]
pub enum ErrorCode {
	_N = -100, // codes without code
	CouldNotOpen,
	CouldNotCompile,

	None = 0,

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
		self.clone() as i32 >= 0
	}
}

#[macro_export]
macro_rules! fmt_error_msg {
	(CouldNotOpen $file:expr, $why:expr) => (format!("could not open file '{}': {}", $file, std::io::Error::from($why)));
	(CouldNotCompile $file:expr) => (format!("could not compile '{}' due to previous error", $file));
	
	(None) => ("there is no error, why did this appear?");

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