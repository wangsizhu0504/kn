use crate::cli_parser::{Cli, Commands};
use crate::command::{
    install, run, uninstall, execute, upgrade,
    clean_install, agent, list, info, watch,
    history, alias, stats, parallel, clean, analyze
};
use crate::display::StyledOutput;

impl Cli {
    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        // Handle working directory change
        if let Some(cwd) = self.cwd {
            std::env::set_current_dir(&cwd)?;
        }

        match self.command {
            Commands::Install { packages, dev, global, exact } => {
                install::handle(packages, dev, global, exact)
            }
            Commands::Run { script_name, args, if_present } => {
                run::handle(script_name, args, if_present)
            }
            Commands::Uninstall { packages, global } => {
                uninstall::handle(packages, global)
            }
            Commands::Execute { command, args } => {
                execute::handle(command, args)
            }
            Commands::Upgrade { packages, interactive, latest } => {
                upgrade::handle(packages, interactive, latest)
            }
            Commands::CleanInstall { force, no_optional } => {
                clean_install::handle(force, no_optional)
            }
            Commands::Agent { manager, args } => {
                agent::handle(manager, args)
            }
            Commands::List { json } => {
                list::handle(json)
            }
            Commands::Info { verbose } => {
                info::handle(verbose)
            }
            Commands::Watch { script_name, patterns } => {
                watch::handle(script_name, patterns)
            }
            Commands::History { count } => {
                history::handle(count)
            }
            Commands::HistoryRun { index } => {
                let cmd = history::run_from_history(index)?;
                // Re-parse and execute the command from history
                execute_history_command(&cmd)
            }
            Commands::HistoryLast => {
                let cmd = history::run_last()?;
                // Re-parse and execute the command from history
                execute_history_command(&cmd)
            }
            Commands::Alias { action, key, value } => {
                alias::handle(action, key, value)
            }
            Commands::Stats => {
                stats::handle()
            }
            Commands::Parallel { scripts } => {
                parallel::handle(scripts)
            }
            Commands::Clean { cache, all, global } => {
                clean::handle(cache, all, global)
            }
            Commands::Analyze => {
                analyze::handle()
            }
            Commands::Help => {
                StyledOutput::opencode_header();
                Ok(())
            }
        }
    }
}

fn execute_history_command(cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Parse command from history
    let parts: Vec<&str> = cmd.split_whitespace().collect();

    if parts.is_empty() {
        return Ok(());
    }

    match parts[0] {
        "run" => {
            let script_name = parts.get(1).map(|s| s.to_string());
            let args: Vec<String> = parts.iter().skip(2).map(|s| s.to_string()).collect();
            run::handle(script_name, args, false)
        }
        "install" => {
            let packages: Vec<String> = parts.iter().skip(1)
                .filter(|s| !s.starts_with('-'))
                .map(|s| s.to_string())
                .collect();
            install::handle(packages, false, false, false)
        }
        _ => {
            StyledOutput::error(&format!("Cannot re-execute command: {}", cmd));
            Ok(())
        }
    }
}
