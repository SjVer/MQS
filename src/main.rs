pub mod info;
pub mod cli;
pub mod report;
pub mod lex;

use cli::*;
use lex::Lexer;
use lex::token::TokenKind;

use std::io::{Write, stderr};
use std::fs::read_to_string;


fn prepare_lint() {
    if lint_mode_is!(Diag) {
        writeln!(stderr(), "[").unwrap();
    }
}

fn finish_lint() {
    if lint_mode_is!(Diag) {
        writeln!(stderr(), "]").unwrap();
    }
}

fn main() {
    // parse cli args
    cli::setup();


    prepare_lint();
    

    let src = match read_to_string(get_cli_arg!(infile)) {
        Ok(text) => text,
        Err(e) => {
            new_formatted_error!(CouldNotOpen get_cli_arg!(infile), e.kind())
                .dispatch();
            std::process::exit(e.raw_os_error().unwrap());
        }
    };

    let src = SOURCES!().new_source(get_cli_arg!(infile), src);
    
    let mut lex = Lexer::new(get_cli_arg!(infile), src);
    let mut tok = lex.next();

    loop {
        println!("{} => {:?}", tok.span.start.to_string(), tok.kind);
     
        if tok.kind == TokenKind::EOF { break; }

        else if let TokenKind::Error(code, msg, _) = tok.kind {
            report::error(msg, Some(code)).dispatch();
        }

        tok = lex.next();
    }

    

    new_formatted_error!(CouldNotCompile &get_cli_arg!(infile)).dispatch();



    finish_lint();
}