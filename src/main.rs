pub mod info;
pub mod cli;
pub mod report;
pub mod lex;

use cli::*;
use lex::span::{Span, Position};
use std::io::{Write, stderr};

pub static mut VERBOSITY: usize = 2;

fn main() {
    // parse and set cli args
    let cli_args = CliArgs::parse();
    if cli_args.verbosity > 2 {
        writeln!(stderr(), concat!("error: Invalid value \"{}\" for '-v <VERBOSITY>': verbosity not in range 0-2\n\n",
                                   "For more information try --help"), cli_args.verbosity).unwrap();
        std::process::exit(1);
    }
    unsafe { VERBOSITY = cli_args.verbosity; }

    println!("{:?}", cli_args);


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

    new_formatted_error!(AlreadyDefined "parameter" "firstparam")
        .with_label(span, Some("duplicate parameter here"))
        .with_colored_label(span2, yansi::Color::White, Some("first occurance here"))
        .with_note("consider removing or renaming the second `firstparam`")
        .dispatch();

    new_formatted_error!(CouldNotCompile &cli_args.infile).dispatch();
}