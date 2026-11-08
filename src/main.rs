mod arduino;

use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use chrono::{DateTime, Local};
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
    Flash {
        /// Show the sketch and its build without touching the board
        #[arg(long)]
        dry_run: bool,

        /// Use this build folder instead of searching the Arduino build cache
        #[arg(long, value_name = "FOLDER")]
        build_path: Option<PathBuf>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Flash {
            dry_run,
            build_path,
        } => flash(dry_run, build_path.as_deref()),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn flash(dry_run: bool, build_path: Option<&Path>) -> Result<(), Box<dyn Error>> {
    // On Linux, the current directory comes with symbolic links already
    // resolved, which is the same path arduino-cli hashes.
    let start = env::current_dir()?;
    let build = arduino::find_build(&start, build_path, arduino::build_cache().as_deref())?;

    if !dry_run {
        return Err("flashing is not implemented yet".into());
    }

    let date: DateTime<Local> = build.date.into();
    println!("sketch: {} ({})", build.name, build.sketch.display());
    println!("build:  {}", build.folder.display());
    println!("fqbn:   {}", build.fqbn);
    println!("built:  {}", date.format("%Y-%m-%d %H:%M"));
    Ok(())
}
