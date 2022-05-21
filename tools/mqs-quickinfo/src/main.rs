use mqs::{self, lex, SOURCES};

use clap::{Parser, Subcommand};
use json;
use std::process::exit;
use self::Command::*;

#[derive(Parser, Debug)]
struct CliArgs {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// json format: { name: line, ... } 
    GetQuestions { f: String },

    /// yes: exit code 0, no: exit code 1
    CanReviewQuestion { f: String, q: String },

    /// yes: exit code 0, no: exit code 1
    QuestionIsTrue { f: String, q: String },
}

fn quick_dis(f: impl ToString) -> mqs::object::dis::Disassembler {
    let objfile = mqs::object::obj_filename(f.to_string());
    if !objfile.exists() { exit(1); }
    
    let mut dis = mqs::object::dis::Disassembler::new(objfile, f.to_string());
    dis.dis();
    dis
}

fn main() {
    let args = CliArgs::parse();
    mqs::cli::set_cli_args_empty();

    // can review file?
    match &args.cmd {
        GetQuestions { f } => {
            let r = || -> Result<mqs::parse::Context, mqs::report::Report> {
        
                // read source and parse
                let src = SOURCES!().new_source(f.clone())?;
                let tokens = mqs::lex::Lexer::new(f.clone(), src).lex();
                let context = mqs::parse::Parser::new().parse(f.clone(), tokens)?;
        
                Ok(context)
            };
        
            if let Ok(c) = r() {
                let mut qs = json::object!{};

                for q in &c.questions {
                    if let Some(ln) = q.token.span.start.line {
                        qs.insert(&q.name, ln).unwrap();
                    }
                }

                println!("{}", json::stringify_pretty(qs, 2));
                exit(0);
            } else {
                exit(1);
            }
        },

        CanReviewQuestion { f, q } => {
            let dis = quick_dis(f);
            
            for qe in &dis.questions {
                if &dis.strings[qe.name] == q {
                    exit(0);
                }
                exit(1);
            }
        },
        QuestionIsTrue { f, q} => {
            let dis = quick_dis(f);
            
            for qe in &dis.questions {
                if &dis.strings[qe.name] == q {
                    exit((!qe.is_true) as i32);
                }
                exit(1);
            }
        },
    }
}
