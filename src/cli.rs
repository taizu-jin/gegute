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
}

#[derive(Args)]
pub struct GetArgs {
    #[arg(short = 's', long = "use-standard", default_value = "rfc3339")]
    pub standard: Timestamp,
}

#[derive(Args)]
pub struct SetArgs {
    #[arg(short = 's', long)]
    standard: Timestamp,
    datetime: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Timestamp {
    Rfc2822,
    Rfc3339,
    Timestamp,
}
