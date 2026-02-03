use crate::cli_parser::{Cli, Commands};
use crate::command::{
    clean, clean_install, execute, info, install, list, run, size, uninstall, upgrade,
    upgrade_self, watch,
};
use crate::display::StyledOutput;

impl Cli {
    pub fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
        // Handle working directory change
        if let Some(cwd) = self.cwd {
            std::env::set_current_dir(&cwd)?;
        }

        match self.command {
            Commands::Install {
                packages,
                dev,
                global,
                exact,
                ignore_scripts,
            } => install::handle(packages, dev, global, exact, ignore_scripts),
            Commands::Run {
                script_name,
                args,
                if_present,
            } => run::handle(script_name, args, if_present),
            Commands::Uninstall { packages, global } => uninstall::handle(packages, global),
            Commands::Execute { command, args } => execute::handle(command, args),
            Commands::Upgrade {
                packages,
                interactive,
                latest,
            } => upgrade::handle(packages, interactive, latest),
            Commands::UpgradeSelf => upgrade_self::handle(),
            Commands::CleanInstall { force, no_optional } => {
                clean_install::handle(force, no_optional)
            }
            Commands::List { json } => list::handle(json),
            Commands::Info { verbose } => info::handle(verbose),
            Commands::Watch {
                script_name,
                patterns,
            } => watch::handle(script_name, patterns),
            Commands::Clean { cache, all, global } => clean::handle(cache, all, global),
            Commands::Size => size::handle(),
            Commands::Help => {
                StyledOutput::opencode_header();
                Ok(())
            }
        }
    }
}
