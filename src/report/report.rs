use crate::{get_cli_arg, get_lint_mode, lint_mode_is};
use crate::lex::span::Span;
use crate::info::{report::{NOTE_LABEL, CODE_PREFIX}, app::NAME};
use super::code::ErrorCode;

use std::io::{Write, stderr};
use yansi::{Color, Paint};
use json::{object, stringify_pretty, JsonValue};


pub struct Quote {
	pub span: Span,
	pub color: Color,
	pub message: Option<String>,
}

impl Quote {
	pub fn to_string(&self, tail: bool) -> String {
		//! newline: trialing <br>
		//! if tail then '│' else '╵'

		let cyan = |s| Color::Cyan.paint(s).bold().to_string();
		let color = |s| self.color.paint(s).bold().to_string();

		let mut text = " ".to_string();

		// "file:line:col"
		text.push_str(&cyan(self.span.start.to_string()));
		text.push('\n');
		
		// no line? return
		if let None = self.span.start.line { return text; }
		let lineno = self.span.start.line.unwrap();

		// make sure that we know the amount of digits in the lineno
		// for correct padding and whatnot
		let digits = lineno.to_string().len();



		// before: " lineno │ ..."
		if let Some(line) = self.span.get_line_before() {
			text.push_str(&cyan(format!(" {:1$} │ ", lineno - 1, digits)));
			text.push_str(line);
			text.push('\n');
		}

		
		
		// line: " lineno │ ..."
		text.push_str(&cyan(format!(" {:1$} │ ", lineno, digits)));
		let part_before = self.span.get_part_before().unwrap();
		text.push_str(part_before);
		text.push_str(&color(self.span.get_part().unwrap()));
		text.push_str(self.span.get_part_after().unwrap());
		text.push('\n');


		// token marking
		text.push_str(&cyan(format!(" {:2$} {} ", "", if tail {'│'} else {'╵'}, digits)));
		for _ in 0..part_before.len() { text.push(' '); }
		let mut mark = String::from("^");
		for _ in 1..self.span.length { mark.push('~'); }
		text.push_str(&color(&mark));



		// token marking message
		if let Some(msg) = &self.message { text.push_str(&color(msg)); }
		text.push('\n');
		
		// optional tail
		if tail && matches!(self.message, Some(_)) {
			text.push_str(&cyan(format!(" {:1$} │\n", "", digits)));
		}

		return text;
	}
}


pub enum Severity {
	Note,
	Warning,
	Error
}

impl Severity {
	fn to_string(&self) -> String {
		String::from(match self {
			Severity::Note => "note",
			Severity::Warning => "warning",
			Severity::Error => "error",
		})
	}
}


pub struct Report {

	pub label: String,
	pub message: String,
	
	pub color: Color,
	pub severity: Severity,
	pub code: Option<ErrorCode>,
	
	pub quote: Option<Quote>,
	pub sub_quotes: Vec<Quote>,
	pub notes: Vec<String>,
}

static mut HAS_DISPATCHED: bool = false;

impl Report {
	pub fn with_quote(mut self, span: Span, message: Option<impl ToString>) -> Self {
		let color = self.color.clone();
		
		if let Some(msg) = message {
			self.quote = Some(Quote{span, color, message: Some(msg.to_string())});
		} else {
			self.quote = Some(Quote{span, color, message: None});
		}
		self
	}
	
	pub fn with_sub_quote(mut self, span: Span, message: impl ToString) -> Self {
		self.sub_quotes.push(Quote{
			span,
			color: Color::White, 
			message: Some(message.to_string())
		});
		self
	}
	
	pub fn with_note(mut self, note: impl ToString) -> Self {
		self.notes.push(note.to_string());
		self
	}


	fn generate_heading(&self) -> String {
		// label
		let mut text = self.color.paint(&self.label).bold().to_string();

		// code if given
		if let Some(code) = &self.code {
			if code.is_useful() {
				text.push_str(&self.color.paint(
					format!("[{}{}]", CODE_PREFIX, code.clone() as u32))
					.bold().to_string());
			}
		}

		// message
		text.push_str(&Paint::new(format!(": {}\n", self.message)).bold().to_string());

		// add "mqs: " for clearification if debugging
		if cfg!(debug_assertions) {
			text.insert_str(0, &Color::Green.paint(": ").bold().to_string());
			text.insert_str(0, &Color::Green.paint(NAME).bold().to_string());
		}
		
		String::from(text)
	}

	fn generate_quote(&self) -> String {
		if let Some(quote) = &self.quote {
			quote.to_string(get_cli_arg!(verbosity) >= 2 && !self.notes.is_empty())
		} else {
			String::new()
		}
	}

	fn generate_sub_quotes(&self) -> String {
		let dotail = get_cli_arg!(verbosity) >= 2 && !self.notes.is_empty();
		let mut text = String::new();

		for (i, quote) in self.sub_quotes.iter().enumerate() {
			// if it isn't the last label, or if notes will follow: add tail 
			let tail = dotail || i + 1 < self.sub_quotes.len();
			text.push_str(&quote.to_string(tail));
		}

		text
	}

	fn generate_notes(&self) -> String {
		let mut text = String::new();

		for note in &self.notes {
			text.push_str(&Paint::new(format!(" {}: ", NOTE_LABEL)).bold().to_string());
			text.push_str(note);
			text.push('\n');
		}

		text
	}

	pub fn dispatch(&self) {
		let mut text = String::new();

		if lint_mode_is!(Diag) {
			// output as json object

			// location and length (default values)
			let mut location = JsonValue::Boolean(false);
			let mut length = JsonValue::Number(0_i8.into());
			
			// set actual location and length if applicable 
			if let Some(s) = &self.quote {
				location = JsonValue::String(s.span.start.to_string());
				length = JsonValue::Number(s.span.length.into());
			}

			// related quotes
			let mut related = json::array![];
			for s in &self.sub_quotes {
				related.push(object!{
					"message" => s.message.clone().unwrap_or_default().as_str(),
					"location" => JsonValue::String(s.span.start.to_string()),
					"length" => JsonValue::Number(s.span.length.into()),
				}).unwrap();
			}

			// code if given
			let code = if let Some(code) = &self.code {
				if code.is_useful() {
					JsonValue::String(format!("{}{}", CODE_PREFIX, code.clone() as u32))
				} else {
					JsonValue::Boolean(false)
				}
			} else {
				JsonValue::Boolean(false)
			};

			// create the json object and "print" it
			text = stringify_pretty(object!{
				"message" => self.message.as_str(),
				"location" => location,
				"length" => length,
				"severity" => self.severity.to_string(),
				"code" => code,
				"related" => related
			}, 4) + ",\n";

		} else {
			// check if we actually want output
			let verbosity = get_cli_arg!(verbosity);
			if verbosity == 0 { return; }
	
			// label
			if verbosity >= 1 { text.push_str(&self.generate_heading()); }
	
			// quotes
			if verbosity >= 2 { text.push_str(&self.generate_quote()); }
			if verbosity >= 2 { text.push_str(&self.generate_sub_quotes()); }
	
			// notes
			if verbosity >= 2 { text.push_str(&self.generate_notes()); }
			
			// write it (with leading newline if this isn't the first error)
			unsafe{
				if HAS_DISPATCHED { text.insert(0, '\n'); }
				else { HAS_DISPATCHED = true; }
			}
		}

		write!(stderr(), "{}", text).unwrap();
	}
}