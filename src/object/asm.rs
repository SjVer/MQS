use super::question::{StringCollection, StringIndex};
use crate::parse::context::Context;

use std::{io::Write, fs::File, path::PathBuf};

pub struct Assembler {
	data: Vec<u8>,
	pub strings: StringCollection,
}

fn int_to_bytes(val: u64, len: u8) -> Vec<u8> {
	let strip_max = len == 0;
	let len = (if len > 0 {len} else {1}) as usize;

	let mut bytes = val.to_be_bytes().to_vec();

	while bytes.len() > len && bytes[0] == 0 {
		bytes.remove(0);
	}

	while bytes.len() > 1 && strip_max {
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

	pub fn asm(&mut self, context: &Context, path: PathBuf) {
		self.strings.push(String::from("a"));
		self.strings.push(String::from("b"));
		self.strings.push(String::from("c"));

		// info
		let stris = size_of_int(self.strings.len() as u64);

		// strings
		for (i, s) in self.strings.clone().iter().enumerate() {
			self.write_byte(0);
			self.write_bytes(&int_to_bytes(i as u64, stris));
			self.write_bytes(&s.to_utf8_bytes());
		}


		// insert header and checksum info at start
		let checksum = self.data[..].iter().map(|x| *x as u64).sum::<u64>();
		self.data = [
			super::HEADER.to_vec(),
			vec![size_of_int(checksum)], int_to_bytes(checksum, 0),
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