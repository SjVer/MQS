use super::question::{IQuestion, IStep, StringCollection, StringIndex};

use std::{io::{BufReader, Read}, fs::File, path::PathBuf};

macro_rules! dis_error {
	($f:expr => $code:ident $($arg:tt)*) => {
		{
			crate::new_formatted_error!($code $($arg)*).dispatch();
			crate::new_formatted_error!(CouldNotReview $f).dispatch();
			crate::exit(1);
		}
	};
}

pub struct Disassembler {
	filename: String,

	data: Vec<u8>,
	bi: usize,

	pub questions: Vec<IQuestion>,
	pub strings: StringCollection,
}

impl Disassembler {

	pub fn new(path: PathBuf, srcfile: String) -> Self {
		let mut data = Vec::<u8>::new();

		match File::open(&path) {
			Ok(f) => {
				match BufReader::new(f).read_to_end(&mut data) {
					Ok(_) => (),
					Err(e) => dis_error!(srcfile => CouldNotOpen path.display(), e.kind())
				}
			},
			Err(e) => { dis_error!(srcfile => CouldNotOpen path.display(), e.kind()); }
		};

		Self {
			filename: srcfile,

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
					if self.bi >= self.data.len() { dis_error!(self.filename => MissingData); }

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
				if !($cond) { dis_error!(self.filename => $code $($arg)*); }
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

		// info
		let stris = read_as!(1, 1 => u8) as usize;
		let qis = read_as!(1, 1 => u8) as usize;
		let qcount = read_as!(qis, 8 => usize);

		// questions
		for qi in 0..qcount {
			test_or_error!(read_as!(qis, 8 => usize) == qi => InvalidIndex "question");
			
			let name = read_as!(stris, 8 => StringIndex);
			let theory = read_as!(stris, 8 => StringIndex);
			
			let sis = read_as!(1, 1 => u8) as usize;
			let scount = read_as!(sis, 8 => usize);
			let mut steps = Vec::<IStep>::new();
			
			// get all steps
			for si in 0..scount {
				test_or_error!(read_as!(sis, 8 => usize) == si => InvalidIndex "step");

				steps.push(IStep{
					description: read_as!(stris, 8 => StringIndex),
					process: read_as!(stris, 8 => StringIndex),
					state_before: read_as!(stris, 8 => StringIndex),
					state_after: read_as!(stris, 8 => StringIndex),
				});
			}

			// conclusion, etc..
			let conclusion = read_as!(stris, 8 => StringIndex);
			let answer = read_as!(stris, 8 => StringIndex);
			let is_true = read_as!(1, 1 => u8) != 0;

			let sts = read_as!(1, 1 => u8) as usize;
			let steps_tried = read_as!(sts, 8 => u64);

			// append question
			self.questions.push(IQuestion{
				name,
				theory,
				steps,
				conclusion,
				answer,
				is_true,
				steps_tried,
			});
		}
	
		// strings
		while self.bi < self.data.len() {
			test_or_error!(read_as!(1, 1 => u8) == 0 => ExpectedNullByte);
			test_or_error!(read_as!(stris, 8 => StringIndex) == self.strings.len() => InvalidIndex "string");

			let mut len = 0;
			while self.bi + len < self.data.len() && self.data[self.bi + len] != 0 {
				len += 1;
			}

			self.strings.push(String::from_utf8(self.data[self.bi..self.bi + len]
				.to_vec()).unwrap_or(String::from("???")));
			self.bi += len;
		}
	}
}