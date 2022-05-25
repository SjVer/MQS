use mqs::*;

use lex::Lexer;
use parse::Parser; 
use object::{obj_filename, {Disassembler, Assembler}};
use report::{ErrorCode, WarningCode, ReportableCode, lint};
use info::report::{WCODE_PREFIX, ECODE_PREFIX};
use cli::{CLI_ARGS, claperr};
use std::{io::{Write, stderr}, path::PathBuf};
use regex::Regex;

#[macro_export]
macro_rules! exp {
    ($what:expr) => { $what.map_err(|_| {()}) };
}

pub fn exit(code: i32) {
    lint::finish_lint();
    std::process::exit(code);
}

fn do_review(objfile: PathBuf, srcfile: String, error: report::Report) {
    // let filename = get_cli_arg!(infile).unwrap();
    // let objfile = obj_filename(filename.clone());

    if !objfile.exists() {
        error.dispatch();
        exit(1);
    }

    let mut dis = Disassembler::new(objfile, srcfile);
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

    let r = || -> Result<_, crate::report::Report> {
        
        // read source and parse
        let src = SOURCES!().new_source(filename.clone())?;
        let tokens = Lexer::new(filename.clone(), src).lex();
        let context = Parser::new().parse(filename.clone(), tokens)?;

        // assemble
        let objf = obj_filename(filename.clone());
        Assembler::new().asm(&context, objf);

        // temp
        do_review(obj_filename(filename.clone()), filename, new_formatted_error!(NoError));

        Ok(())
    };

    if let Err(r) = r() {
        r.dispatch();
        exit(1);
    }
}


fn main() {
    // parse cli args
    cli::setup();
    lint::prepare_lint();

    if let Some(code) = get_cli_arg!(explain) {
        // explain code
        let do_try = || -> Result<(), ()> {
            let prefix = code.chars().nth(0).unwrap_or('_');
            let rest = exp!(code.get(1..).unwrap_or("").parse::<i16>())?;

            match prefix {
                ECODE_PREFIX => {
                    let c = exp!(ErrorCode::try_from(rest))?;
                    let t = match c.get_type() {
                        Some(t) => format!(" ({})", t),
                        None => String::new()
                    };
    
                    println!("error code {}: {}{}", code.to_string(), c.get_name(), t);
                },
                WCODE_PREFIX => {
                    let c = exp!(WarningCode::try_from(rest))?;
                    println!("warning code {}: {}", code.to_string(), c.get_name());
                }
                _ => return Err(())
            }

            Ok(())
        };

        if let Err(_) = do_try() {
            new_formatted_error!(CannotExplainCode code).dispatch();
            exit(1);
        }
    }
    else if let Some(path) = get_cli_arg!(dis) {
        let error = new_formatted_error!(CannotReview "nonexistent object file", path);
        do_review(PathBuf::from(&path), path, error);
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
            let error = new_formatted_error!(CannotReview "uninterpreted file", filename);
            do_review(objpath, filename, error);
        }
        else { do_file(); }
    }

    lint::finish_lint();
}