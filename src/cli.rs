use crate::info::cli;
pub use clap::{Parser, ArgEnum, AppSettings::DeriveDisplayOrder};
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

    #[clap(short, default_value_t = 2)]
    #[clap(value_name = "VERBOSITY", help = cli::ARG_VERBOSE)]
    pub verbosity: usize,

    #[clap(long)]
    pub lint: Option<LintMode>,
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
    if get_cli_arg!(verbosity) > 2 {
        writeln!(stderr(), concat!("error: Invalid value \"{}\" for '-v <VERBOSITY>': verbosity not in range 0-2\n\n",
                                   "For more information try --help"), get_cli_arg!(verbosity)).unwrap();
        std::process::exit(1);
    }
}
