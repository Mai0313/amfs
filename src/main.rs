use std::process::ExitCode;

use amfs::cli::Cli;
use clap::Parser;

fn main() -> ExitCode {
    match Cli::parse().run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("amfs: {err}");
            ExitCode::FAILURE
        }
    }
}
