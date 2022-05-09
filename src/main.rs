pub mod info;
pub mod report;
pub mod lex;

// use lex::span::{Span, Position};
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

    // report::error::dispatch_snippet(104, &String::from("duplicate parameter `firstparam`"), 
    //     &Span{
    //         start: Position{
    //             file: String::from("test.mqs"),
    //             line: 32,
    //             column: 8},
    //         length: 5});
    report::error("duplicate parameter `firstparam`", Some(104))
        .with_note("this is not allowed")
        .dispatch();

    report::error("could not compile 'test.txt' due to previous error", None)
        .dispatch();
}