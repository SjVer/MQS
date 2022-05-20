use super::question::{StringCollection, StringIndex};
use crate::parse::context::Context;

use std::{io::{BufReader, Read}, fs::File, path::PathBuf};

pub struct Assembler {
	data: Vec<u8>,
	pub strings: StringCollection,
}

impl Assembler {
	pub fn new() -> Self {
		Self {
			data: Vec::new(),
			strings: StringCollection::new(),
		}
	}

	pub fn asm(&mut self, context: &Context, path: PathBuf) {
		// macro_rules! write {
		// 	()
		// }

		
	}
}