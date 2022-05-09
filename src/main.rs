pub mod info;
pub mod report;
pub mod lex;

use lex::span::{Span, Position};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = info::cli::DESCRIPTION)]
struct Args {

    #[clap(help = info::cli::ARG_INFILE)]
    infile: String,

    #[clap(short, parse(from_occurrences), help = info::cli::ARG_VERBOSE)]
    verbose: usize,
}

fn main() {
    // let args = Args::parse();

    let span = Span {
        start: Position {
            file: "test.txt".to_string(),
            line: Some(83),
            column: Some(15),
        },
        length: 10,
    };
    let span2 = Span {
        start: Position {
            file: "test.txt".to_string(),
            line: Some(83),
            column: Some(15),
        },
        length: 10,
    };

    new_formatted_error!(DuplicateParameter, "firstparam")
        .with_label(span, Some("duplicate parameter here"))
        .with_colored_label(span2, yansi::Color::White, Some("first occurance here"))
        .with_note("consider removing or renaming the second `firstparam`")
        .dispatch();

    new_formatted_error!(CouldNotCompile, "test.txt").dispatch();
}