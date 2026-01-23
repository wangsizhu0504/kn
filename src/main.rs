mod agents;
mod command;
mod command_utils;
mod config;
mod detect;
mod parse;
mod runner;
mod storage;
mod utils;
mod display;
mod cli_parser;

use cli_parser::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
