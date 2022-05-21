use super::question::{StringCollection, StringIndex};
use crate::parse::{context::Context, astprinter::TheoryPrinter};

use std::{io::Write, fs::File, path::PathBuf};

pub struct Assembler {
	data: Vec<u8>,
	pub strings: StringCollection,
}

fn int_to_bytes(val: u64, len: u8) -> Vec<u8> {
	let len = (if len > 0 {len} else {1}) as usize;
	let mut bytes = val.to_be_bytes().to_vec();

	while bytes.len() > len && bytes[0] == 0 {
		bytes.remove(0);
	}

	bytes
}

fn size_of_int(val: u64) -> u8 {
	// 
	int_to_bytes(val, 0).len() as u8
}

impl Assembler {
	pub fn new() -> Self {
		Self {
			data: Vec::new(),
			strings: StringCollection::new(),
		}
	}

	fn write_byte(&mut self, byte: u8) {
		//
		self.data.push(byte);
	}

	fn write_bytes(&mut self, bytes: &[u8]) {
		//
		self.data.extend(bytes);
	}

	fn add_string(&mut self, string: impl ToString) -> StringIndex {
		// TODO: optimize? (interning)
		self.strings.push(string.to_string());
		self.strings.len() - 1
	}

	pub fn asm(&mut self, context: &Context, path: PathBuf) {
		let stris = size_of_int(self.strings.len() as u64);
		let qis = size_of_int(context.questions.len() as u64);

		// header and checksum info will be inserted in the end
		// to avoid checksum issues
		
		macro_rules! write_int {
			//
			($int:expr, $len:expr) => (self.write_bytes(&int_to_bytes($int as u64, $len)))
		}
		macro_rules! write_str {
			($string:expr) => {{
				let i = self.add_string(&$string);
				write_int!(i, stris);
			}}
		}

		// info (STRIS, QIS & question count)
		self.write_byte(stris);
		self.write_byte(qis);
		write_int!(context.questions.len(), qis);

		// questions
		for (i, q) in context.questions.iter().enumerate() {
			// index, name & theory
			write_int!(i, qis);
			write_str!(q.name);
			write_str!(TheoryPrinter::print(&q.theory));

			// SIS & step count
			let sis = size_of_int(/* q.steps.len() */ 0);
			self.write_byte(sis);
			write_int!(/* q.steps.len() */ 0, sis);

			// steps

			// conclusion, answer & answer type
			write_str!(/* q.conclusion */ "some conclusion");
			write_str!(/* q.answer */ "some answer");
			self.write_byte(/* q.answer_type as u64 */ 1);

			// STS & steps tried
			let sts = size_of_int(/* q.steps_tried */ 123);
			self.write_byte(sts);
			write_int!(/* q.steps_tried */ 123, sts);
		}
		// strings
		for (i, s) in self.strings.clone().iter().enumerate() {
			self.write_byte(0);
			self.write_bytes(&int_to_bytes(i as u64, stris));
			self.write_bytes(&s.as_bytes());
		}

		// insert header and checksum info at start
		let checksum = self.data[..].iter().map(|x| *x as u64).sum::<u64>();
		self.data = [
			super::HEADER.to_vec(),
			vec![size_of_int(checksum)], int_to_bytes(checksum, size_of_int(checksum)),
			self.data.clone()
		].concat();

		// write file
		if let Ok(mut f) = File::create(path.clone()) {
			f.write_all(&self.data).expect("failed to write mqso file");
		} else {
			unreachable!();
		}
	}
}