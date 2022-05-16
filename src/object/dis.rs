use super::question::{SQuestion, StringCollection};

use std::{io::{BufReader, Read}, fs::File};

macro_rules! dis_error {
	($code:ident $($arg:tt)*) => {
		{
			crate::new_formatted_error!($code $($arg)*).dispatch();
			std::process::exit(1);
		}
	};
}

pub struct Disassembler {
	data: Vec<u8>,
	bi: usize,

	questions: Vec<SQuestion>,
	strings: StringCollection,
}

impl Disassembler {

	pub fn new(file: String) -> Self {
		let mut data = Vec::<u8>::new();

		match File::open(&file) {
			Ok(f) => {
				match BufReader::new(f).read_to_end(&mut data) {
					Ok(_) => (),
					Err(e) => dis_error!(CouldNotOpen file, e.kind())
				}
			},
			Err(e) => { dis_error!(CouldNotOpen file, e.kind()); }
		};

		Self {
			data,
			bi: 0,

			questions: Vec::new(),
			strings: StringCollection::new()
		}
	}

	/// returns `true` if successful
	pub fn dis(&mut self) {
		macro_rules! read_as {
			// (read size, type size => type)
			($size:expr, $buffsize:expr => $type:ty) => (
				{
					self.bi += $size;
					if self.bi >= self.data.len() { dis_error!(MissingData); }

					let mut buff = [0u8; $buffsize];
					buff[$buffsize - $size..].copy_from_slice(
						&self.data[self.bi - $size..self.bi]
					);

					<$type>::from_be_bytes(buff)
				}
			);
		}
		macro_rules! test_or_error {
			($cond:expr => $code:ident $($arg:tt)*) => {
				if !($cond) { dis_error!($code $($arg)*); }
			};
		}

		// header
		test_or_error!(&self.data[0..super::HEADER.len()] == super::HEADER => InvalidHeader);
		self.bi += super::HEADER.len();

		// checksum
		let css = read_as!(1, 1 => u8) as usize;
		test_or_error!(
			read_as!(css, 8 => u64) == self.data[self.bi..].iter().map(|x| *x as u64).sum::<u64>()
			=> InvalidChecksum
		);

		// 
	}
}