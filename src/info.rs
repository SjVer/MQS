//! This module contains information (such as text) to be used by the rest of the compiler.

#![allow(dead_code)]

macro_rules! static_string {
	($name:ident, $str:expr) => {pub static $name: &str = $str;}
}

pub mod app {
	static_string!(NAME, "mqs");
	static_string!(FULL_NAME, "MQS (official)");
	static_string!(VERSION, env!("CARGO_PKG_VERSION"));
}

pub mod cli {
	static_string!(DESCRIPTION, "The Official MQS Interpreter - written by Sjoerd Vermeulen");
	static_string!(ARG_INFILE, "The file to interpret");
	static_string!(ARG_REVIEW, "Review INFILE's last results");
	static_string!(ARG_DIS, "Disassemble MQS object file OBJFILE");
	static_string!(ARG_AT, "Review the given question (at a given step)");
	static_string!(ARG_MUTE, "Mute all warnings");
	static_string!(ARG_COMPACT, "Produce compact output");
	static_string!(ARG_QUIET, "Hide all output");
	static_string!(ARG_EXPLAIN, "Explain the given error code");

	pub const LINT_NONE_NAME: &str = "none";
	pub const LINT_DIAG_NAME: &str = "diag";
}

pub mod report {
	// static_string!(ECODE_PREFIX, "E");
	// static_string!(WCODE_PREFIX, "W");
	pub const ECODE_PREFIX: char = 'E';
	pub const WCODE_PREFIX: char = 'W';

	static_string!(ERROR_LABEL, "error");
	static_string!(WARNING_LABEL, "warning");
	static_string!(NOTE_LABEL, "note");
}