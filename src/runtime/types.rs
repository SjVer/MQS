pub trait Type {
	fn to_string(&self) -> &str;
}

impl dyn Type {
	pub fn from_string(string: String) -> Option<impl Type> {
		match string.as_str() {
			"int" => Some(Int),
			
			_ => None
		}
	}
}

macro_rules! __to_string {
	($ret:expr) => {
		fn to_string(&self) -> &str { $ret }
	};
}

struct Int;
impl Type for Int {
	__to_string!{"int"}
}