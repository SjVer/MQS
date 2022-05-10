pub mod info;
pub mod report;
pub mod lex;

use clap::Parser;
use lex::span::{Span, Position};

/// struct containing arguments from cli
#[derive(Parser, Debug)]
#[clap(version, about, long_about = info::cli::DESCRIPTION)]
struct CliArgs {

    #[clap(help = info::cli::ARG_INFILE)]
    infile: String,

    #[clap(short, default_value_t = 1,
      value_name = "VERBOSITY",
      help = info::cli::ARG_VERBOSE)]
    verbosity: usize,
}

fn main() {
    // parse and set cli args
    let cli_args = CliArgs::parse();
    report::set_verbosity(cli_args.verbosity);


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