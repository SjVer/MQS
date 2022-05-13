use crate::info::cli;
use clap::{Parser, ArgEnum, AppSettings::DeriveDisplayOrder};
use crate::report::code::ErrorCode;
use std::io::{Write, stderr};

pub static mut CLI_ARGS: Option<CliArgs> = None;


#[macro_export]
macro_rules! get_cli_arg {
    ($field:ident) => ( unsafe { crate::CLI_ARGS.clone().unwrap().$field } )
}

#[macro_export]
macro_rules! get_lint_mode {
    () => (get_cli_arg!(lint).unwrap_or(crate::cli::LintMode::None))
}

#[macro_export]
macro_rules! lint_mode_is {
    ($mode:ident) => (get_lint_mode!() == crate::LintMode::$mode)
}


/// struct containing arguments from cli
#[derive(Parser, Clone, Debug)]
#[clap(version, about, long_about = cli::DESCRIPTION)]
#[clap(propagate_version = true)]
#[clap(global_setting(DeriveDisplayOrder))]
pub struct CliArgs {

    #[clap(help = cli::ARG_INFILE)]
    pub infile: String,

    #[clap(short, long, help = cli::ARG_COMPACT)]
    pub compact: bool,

    #[clap(short, long, help = cli::ARG_QUIET)]
    pub quiet: bool,

    #[clap(long)]
    pub lint: Option<LintMode>,


    #[clap(long, help = cli::ARG_EXPLAIN)]
    pub explain: Option<u8>,
}


#[derive(Copy, Clone, Debug, ArgEnum, PartialEq)]
pub enum LintMode {
    #[clap(name = cli::LINT_NONE_NAME)] None,
    #[clap(name = cli::LINT_DIAG_NAME)] Diag,
}

impl std::str::FromStr for LintMode {
    type Err = clap::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            cli::LINT_NONE_NAME => Ok(LintMode::None),
            cli::LINT_DIAG_NAME => Ok(LintMode::Diag),
            _ => Err(
                clap::Error::raw(clap::ErrorKind::InvalidValue,
                "invalid lint mode")),
        }
    }
}

impl std::fmt::Display for LintMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LintMode::None => write!(f, "{}", cli::LINT_NONE_NAME),
            LintMode::Diag => write!(f, "{}", cli::LINT_DIAG_NAME),
        }
    }
}


pub fn setup() {
    unsafe { CLI_ARGS = Some(CliArgs::parse()); }
    
    if let Some(code) = get_cli_arg!(explain) {
        // explain error
        match ErrorCode::try_from(code as i16) {
            Ok(e) => {
                let t = match e.get_type() {
                    Some(t) => format!(" ({} error)", t),
                    None => String::new()
                };

                println!("error code E{}: {}{}", code, e.get_name(), t);
                std::process::exit(0);
            },
            Err(_) => {
                writeln!(stderr(), "cannot explain invalid error code {:?}", code)
                    .unwrap();
                std::process::exit(1);
            },
        }

    }
}
