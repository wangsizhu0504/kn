mod agents;
mod cli_parser;
mod command;
mod command_utils;
mod config;
mod detect;
mod display;
mod parse;
mod runner;
mod update_checker;
mod utils;
mod version;

use cli_parser::Cli;
use tracing_subscriber::{fmt, EnvFilter};

fn init_logging() {
    // 初始化日志系统
    // 可以通过 KN_LOG 环境变量控制日志级别
    // 例如: KN_LOG=debug kn install package
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .init();
}

fn main() {
    // 初始化日志系统
    init_logging();

    // Check for updates in the background (non-blocking)
    let update_handle = update_checker::check_for_updates();

    let cli = match Cli::parse() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let exit_code = if let Err(e) = cli.execute() {
        eprintln!("  {} {}", console::style("✖").red().bold(), e);
        1
    } else {
        0
    };

    // Print update notification after command output is done
    if let Ok(Some(msg)) = update_handle.join() {
        eprint!("{}", msg);
    }

    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}
