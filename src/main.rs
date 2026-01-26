mod agents;
mod cli_parser;
mod command;
mod command_utils;
mod config;
mod detect;
mod display;
mod parse;
mod runner;
mod storage;
mod update_checker;
mod utils;

use cli_parser::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check for updates in the background (non-blocking)
    update_checker::check_for_updates();

    let cli = match Cli::parse() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    cli.execute()
}

// Include tests at the end of main.rs to ensure they can access all modules
#[cfg(test)]
mod tests;
