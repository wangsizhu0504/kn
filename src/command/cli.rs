use crate::cli_parser::{Cli, Commands};
use crate::command::{
    install, run, uninstall, execute, upgrade,
    clean_install, agent, list, info, watch,
    stats, parallel, clean, analyze,
    doctor, size, completion
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
            Commands::Doctor => {
                doctor::handle()
            }
            Commands::Size => {
                size::handle()
            }
            Commands::Completion { shell } => {
                completion::handle(shell)
            }
            Commands::Help => {
                StyledOutput::opencode_header();
                Ok(())
            }
        }
    }
}
