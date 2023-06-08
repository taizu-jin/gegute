use std::fmt::Display;

use clap::{Args, Parser, Subcommand, ValueEnum};

/// Gets and (aspirationally) sets the time.
#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// [DEFAULT] Prints the current time in specified format.
    Get(GetArgs),
    /// (Aspirationally) sets the time.
    Set(SetArgs),
    /// Resolve clock differences with Network Time Protocol (NTP)
    Ntp,
}

#[derive(Args)]
pub struct GetArgs {
    #[arg(short = 's', long = "use-standard", default_value = "rfc3339")]
    pub standard: GetFormat,
}

#[derive(Args)]
pub struct SetArgs {
    #[arg(short = 's', long = "use-standard", default_value = "rfc3339")]
    pub standard: SetFormat,
    pub datetime: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum GetFormat {
    Rfc2822,
    Rfc3339,
    Timestamp,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SetFormat {
    Rfc2822,
    Rfc3339,
}

impl Display for SetFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetFormat::Rfc3339 => write!(f, "rfc3339"),
            SetFormat::Rfc2822 => write!(f, "rfc2822"),
        }
    }
}
