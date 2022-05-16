use crate::runtime::question::Question;

#[derive(Clone)]
pub struct Context {
	questions: Vec<Question>,
}

impl Context {
	pub fn new() -> Self {
		Self {
			questions: Vec::<Question>::new(),
		}
	}
}