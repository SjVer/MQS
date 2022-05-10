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
	static_string!(ARG_VERBOSE, "Increase output verbosity");
}

pub mod report {
	static_string!(CODE_PREFIX, "E");
	static_string!(ERROR_LABEL, "error");
	static_string!(WARNING_LABEL, "warning");
	static_string!(NOTE_LABEL, "note");
}

/*
#[macro_export]
macro_rules! mod_and_use {
	($module:ident $(::$path:tt)+) => {
		mod $module; use $module $(::$path)*;
	}
}
*/