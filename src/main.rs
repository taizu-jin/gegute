use chrono::DateTime;
use clap::Parser;
use gegute::{
    cli::{Cli, Commands, GetFormat, SetFormat},
    clock::Clock,
};
use std::{eprintln, println};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(ref command) => match command {
            Commands::Get(args) => {
                let now = Clock::get();
                let time = match args.standard {
                    GetFormat::Rfc2822 => now.to_rfc2822(),
                    GetFormat::Rfc3339 => now.to_rfc3339(),
                    GetFormat::Timestamp => now.timestamp().to_string(),
                };
                println!("{}", time)
            }
            Commands::Set(args) => {
                let parser = match args.standard {
                    SetFormat::Rfc2822 => DateTime::parse_from_rfc2822,
                    SetFormat::Rfc3339 => DateTime::parse_from_rfc3339,
                };

                let err_msg = format!(
                    "Unable to parse {} according to {}",
                    args.datetime, args.standard,
                );
                let t = parser(&args.datetime).expect(&err_msg);

                Clock::set(t);

                let os_error = std::io::Error::last_os_error();
                let os_error_code = &os_error.raw_os_error();

                match os_error_code {
                    Some(0) => (),
                    Some(_) => eprintln!("Unable to set the time: {:?}", os_error),
                    None => (),
                }
            }
        },
        None => println!("{}", Clock::get().to_rfc3339()),
    }
}
