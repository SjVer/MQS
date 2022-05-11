use crate::lex::span::Span;
use crate::info::{report::NOTE_LABEL, app::NAME};
use std::io::{Write, stderr};
use yansi::{Color, Paint};

pub struct Label {
	pub span: Span,
	pub color: Color,
	pub message: Option<String>,
}

impl Label {
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
		let line = self.span.start.line.unwrap();

		// make sure that we know the amount of digits in the lineno
		// for correct padding and whatnot
		let digits = line.to_string().len();

		// " lineno │ ..."
		text.push_str(&cyan(format!(" {:1$} │ ", line - 1, digits)));
		/* tmp */ text.push_str("var a = 123;\n");

		// " lineno │ ..."
		text.push_str(&cyan(format!(" {:1$} │ ", line, digits)));
		/* tmp */ text.push_str("func f(firstparam, firstparam) {\n");

		// token marking
		text.push_str(&cyan(format!(" {:2$} {} ", "", if tail {'│'} else {'╵'}, digits)));
		/* tmp */ text.push_str(&color("                   ^~~~~~~~~~ ".to_string()));

		// token marking message
		if let Some(msg) = &self.message { text.push_str(&color(msg.to_string())); }
		text.push('\n');
		
		// optional tail
		if tail && matches!(self.message, Some(_)) {
			text.push_str(&cyan(format!(" {:1$} │\n", "", digits)));
		}

		return text;
	}
}

pub struct Report {

	pub label: String,
	pub color: Color,
	pub message: String,

	pub labels: Vec<Label>,
	pub notes: Vec<String>,
}

static mut HAS_DISPATCHED: bool = false;

impl Report {
	pub fn with_colored_label(mut self, span: Span, color: Color, message: Option<impl ToString>) -> Self {
		if let Some(msg) = message {
			self.labels.push(Label{span, color, message: Some(msg.to_string())});
		} else {
			self.labels.push(Label{span, color, message: None});
		}
		self
	}
	pub fn with_label(self, span: Span, message: Option<impl ToString>) -> Self {
		let color = self.color.clone();
		self.with_colored_label(span, color, message)
	}

	pub fn with_note(mut self, note: impl ToString) -> Self {
		self.notes.push(note.to_string());
		self
	}


	fn generate_heading(&self) -> String {
		let mut text = self.color.paint(&self.label).bold().to_string();
		text.push_str(&Paint::new(format!(": {}\n", self.message)).bold().to_string());

		// add "mqs: " for clearification if debugging
		if cfg!(debug_assertions) {
			text.insert_str(0, &Color::Green.paint(": ").bold().to_string());
			text.insert_str(0, &Color::Green.paint(NAME).bold().to_string());
		}
		
		String::from(text)
	}

	fn generate_labels(&self) -> String {
		let verb = unsafe { crate::VERBOSITY };
		let mut text = String::from("");

		for (i, label) in self.labels.iter().enumerate() {

			// if it isn't the last label, or if notes will follow: add tail 
			let tail = (!self.notes.is_empty() && verb >= 2) || i + 1 < self.labels.len();

			text.push_str(&label.to_string(tail));
		}

		text
	}

	fn generate_notes(&self) -> String {
		let mut text = String::from("");

		for note in &self.notes {
			text.push_str(&Paint::new(format!(" {}: ", NOTE_LABEL)).bold().to_string());
			text.push_str(note);
			text.push('\n');
		}

		text
	}

	pub fn dispatch(&self) -> () {
		let verb = unsafe { crate::VERBOSITY };

		// if verbosity = 0 don't print shit
		if verb == 0 { return; }

		let mut text = String::from("");

		// label
		if verb >= 1 { text.push_str(&self.generate_heading()); }

		// labels
		if verb >= 2 { text.push_str(&self.generate_labels()); }

		// notes
		if verb >= 2 { text.push_str(&self.generate_notes()); }

		// write it (with leading newline if this isn't the first error)
		unsafe{
			if HAS_DISPATCHED { text.insert(0, '\n'); }
			else { HAS_DISPATCHED = true; }
		}
		write!(stderr(), "{}", text).unwrap();
	}
}