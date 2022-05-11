use crate::info::cli;
pub use clap::{Parser, ArgEnum};

/// struct containing arguments from cli
#[derive(Parser, Debug)]
#[clap(version, about, long_about = cli::DESCRIPTION)]
#[clap(propagate_version = true)]
pub struct CliArgs {

    #[clap(help = cli::ARG_INFILE)]
    pub infile: String,

    #[clap(short, default_value_t = 2)]
    #[clap(value_name = "VERBOSITY", help = cli::ARG_VERBOSE)]
    pub verbosity: usize,

    #[clap(default_value_t = LintMode::None)]
   pub lint: LintMode,
}

#[derive(Copy, Clone, Debug, ArgEnum)]
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
        write!(f, "{:?}", self)
    }
}