pub fn exit(code: i32) {
    std::process::exit(code);
}

pub mod lex;
pub mod object;
pub mod parse;
pub mod report;
pub mod runtime;
pub mod cli;
pub mod info;

pub use cli::{CLI_ARGS, LintMode};