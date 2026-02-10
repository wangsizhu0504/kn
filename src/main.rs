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
    update_checker::check_for_updates();

    let cli = match Cli::parse() {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = cli.execute() {
        eprintln!("  {} {}", console::style("✖").red().bold(), e);
        std::process::exit(1);
    }
}
