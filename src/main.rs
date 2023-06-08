use clap::Parser;
use gegute::{
    cli::{Cli, Commands, Timestamp},
    clock::Clock,
};
use std::println;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(ref command) => match command {
            Commands::Get(args) => {
                let now = Clock::get();
                let time = match args.standard {
                    Timestamp::Rfc2822 => now.to_rfc2822(),
                    Timestamp::Rfc3339 => now.to_rfc3339(),
                    Timestamp::Timestamp => now.timestamp().to_string(),
                };
                println!("{}", time)
            }
            Commands::Set(_) => todo!(),
        },
        None => println!("{}", Clock::get().to_rfc3339()),
    }
}
