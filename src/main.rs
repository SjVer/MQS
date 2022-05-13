pub mod lex;
pub mod parse;
pub mod report;
pub mod runtime;
pub mod cli;
pub mod info;

use cli::*;
use lex::Lexer;
use lex::token::TokenKind;
use std::fs::read_to_string;


fn prepare_lint() {
    if lint_mode_is!(Diag) {
        println!("[");
    }
}

fn finish_lint() {
    if lint_mode_is!(Diag) {
        println!("]");
    }
}

fn main() {
    // parse cli args
    cli::setup();


    prepare_lint();
    
    let filename = get_cli_arg!(infile);
    let src = match read_to_string(&filename) {
        Ok(text) => text,
        Err(e) => {
            new_formatted_error!(CouldNotOpen &filename, e.kind())
                .dispatch();
            std::process::exit(e.raw_os_error().unwrap());
        }
    };

    let src = SOURCES!().new_source(filename.clone(), src);
    
    let mut lex = Lexer::new(filename.clone(), src);
    let mut tok = lex.next();

    loop {
        println!("{} => {:?}", tok.span.start.to_string(), tok.kind);

        if tok.kind == TokenKind::EOF { break; }

        else if let TokenKind::Error(code, msg, _) = tok.kind {
            report::error(msg, Some(code))
                .with_quote(tok.span, None::<String>)
                .dispatch();
        }

        tok = lex.next();
    }

    


    new_formatted_error!(CouldNotCompile &filename).dispatch();


    finish_lint();
}