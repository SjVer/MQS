pub mod lex;
pub mod object;
pub mod parse;
pub mod report;
pub mod runtime;
pub mod cli;
pub mod info;

use lex::Lexer;
use parse::Parser; 
use object::{obj_filename, dis::Disassembler};
use report::code::ErrorCode;
use cli::{CLI_ARGS, LintMode, claperr};

use std::{io::{Write, stderr}, fs::read_to_string, path::PathBuf};
use regex::Regex;

pub fn exit(code: i32) {
    finish_lint();
    std::process::exit(code);
}

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


fn do_review(objfile: PathBuf, error: report::Report) {
    // let filename = get_cli_arg!(infile).unwrap();
    // let objfile = obj_filename(filename.clone());

    if !objfile.exists() {
        error.dispatch();
        exit(1);
    }

    let mut dis = Disassembler::new(objfile);
    dis.dis();

    // check and parse --at option
    if let Some(at) = get_cli_arg!(at) {
        let re = Regex::new("^([a-zA-Z_][a-zA-Z0-9_]*'*)(?::([0-9]+))?$").unwrap();
        
        if !re.is_match(&at) {
            // invalid value
            clap::Error::raw(clap::ErrorKind::InvalidValue, "value does not match 'QUESTION[:STEP]'")
                .print().unwrap();
            writeln!(stderr(), "\n\nFor more information try --help").unwrap();
            exit(1);
        }
        let c = re.captures(&at).unwrap();
        
        // find question
        let mut question: i32 = -1;
        for i in 0..dis.questions.len() {
            if dis.strings[dis.questions[i].name] == c[1].to_string() {
                question = i as i32;
                break;
            }
        }
        if question == -1 {
            new_formatted_error!(CannotReview "uninterpreted question", &c[1]).dispatch();
            exit(1);
        }
        
        let question = dis.questions[question as usize].stringify(&dis.strings);
        if c.get(2).is_some() {
            question.print_at(c[2].parse().unwrap())
        } else {
            question.print();
        }
        
    } else {
        let mut true_count = 0;

        for q in &dis.questions {
            q.stringify(&dis.strings).print();
            true_count += q.is_true as i32;
            println!("");
        }

        println!("{}/{} answers are true", true_count, dis.questions.len());
    }
}

fn do_file() {
    let filename = get_cli_arg!(infile).unwrap();
    let src = match read_to_string(&filename) {
        Ok(text) => text,
        Err(e) => {
            new_formatted_error!(CouldNotOpen &filename, e.kind())
                .dispatch();
            std::process::exit(e.raw_os_error().unwrap());
        }
    };

    let src = SOURCES!().new_source(filename.clone(), src);
    let tokens = Lexer::new(filename.clone(), src).lex();
    // Parser::new().parse(tokens);
    for t in &tokens {
        println!("{} => {:?}", t.span.start.to_string(), t.kind);
    }

    // new_formatted_error!(CouldNotCompile &filename).dispatch();
}


fn main() {
    // parse cli args
    cli::setup();
    prepare_lint();

    if let Some(code) = get_cli_arg!(explain) {
        // explain error
        match ErrorCode::try_from(code as i16) {
            Ok(e) => {
                let t = match e.get_type() {
                    Some(t) => format!(" ({} error)", t),
                    None => String::new()
                };

                println!("error code E{}: {}{}", code, e.get_name(), t);
            },
            Err(_) => {
                new_formatted_error!(CannotExplainCode code).dispatch();
                exit(1);
            },
        }
    }
    else if let Some(path) = get_cli_arg!(dis) {
        let error = new_formatted_error!(CannotReview "nonexistent object file", path);
        do_review(PathBuf::from(&path), error);
    }
    else {
        if let None = get_cli_arg!(infile) {
            claperr::Error::raw(claperr::ErrorKind::MissingRequiredArgument, "missing required argument INFILE")
                .print().unwrap();
            writeln!(stderr(), "\n\nFor more information try --help").unwrap();
            exit(1);
        }

        if get_cli_arg!(review) {
            let filename = get_cli_arg!(infile).unwrap();
            let objpath = obj_filename(filename.clone());
            do_review(objpath, new_formatted_error!(CannotReview "uninterpreted file", filename));
        }
        else { do_file(); }
    }

    finish_lint();
}