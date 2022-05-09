mod common; use common::info;
mod report;
mod_and_use!(lex::span::*);
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

    report::error::dispatch_snippet(104, &String::from("duplicate parameter `firstparam`"), 
        &Span{
            start: Position{
                file: String::from("test.mqs"),
                line: 32,
                column: 8},
            length: 5});

    report::error::dispatch_simple(0, &String::from("could not compile 'test.mqs' due to previous error"));
}