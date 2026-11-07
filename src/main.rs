use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Flash the latest build of the current sketch to the connected board
    Flash,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Command::Flash => {
            eprintln!("flash: not implemented yet");
            ExitCode::FAILURE
        }
    }
}
