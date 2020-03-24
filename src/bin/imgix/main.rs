use std::process::exit;

use imgix::Result;

// Module Declarations.
mod cli;
mod commands;

fn main() -> Result<()> {
    Ok(run(cli::app())?)
}

/// Executes a cli app. This function parses the command line arguments and
/// maps a given command to _its_ executor.
fn run(app: clap::App<'static, 'static>) -> Result<()> {
    match app.get_matches().subcommand() {
        _ => {
            exit(1);
        }
    }
}
