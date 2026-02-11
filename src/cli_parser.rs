use console::style;
use std::env;

use crate::utils::levenshtein_distance;

// Find similar commands based on user input
fn find_similar_commands(input: &str) -> Vec<(String, usize)> {
    let all_commands = [
        "install",
        "i",
        "add",
        "run",
        "r",
        "uninstall",
        "remove",
        "rm",
        "execute",
        "exec",
        "x",
        "upgrade",
        "update",
        "up",
        "upgrade-self",
        "clean-install",
        "ci",
        "list",
        "ls",
        "info",
        "env",
        "watch",
        "w",
        "clean",
        "view",
        "help",
    ];

    let mut similarities: Vec<(String, usize)> = all_commands
        .iter()
        .map(|&cmd| (cmd.to_string(), levenshtein_distance(input, cmd)))
        .collect();

    similarities.sort_by_key(|(_, dist)| *dist);

    similarities
        .into_iter()
        .filter(|(_, dist)| *dist <= 3)
        .take(3)
        .collect()
}

fn format_unknown_command_error(input: &str) -> String {
    let similar = find_similar_commands(input);

    let mut error_msg = format!(
        "\n  {} {}\n",
        style("✖").red().bold(),
        style(format!("Unknown command: {}", input)).red(),
    );

    if !similar.is_empty() {
        error_msg.push_str("\n  Did you mean:\n");
        for (i, (cmd, _)) in similar.iter().enumerate() {
            let is_last = i == similar.len() - 1;
            let connector = if is_last { "└" } else { "├" };
            error_msg.push_str(&format!(
                "  {} kn {}\n",
                style(connector).dim(),
                style(cmd).cyan(),
            ));
        }
    }

    error_msg.push_str(&format!(
        "\n  {} {}\n",
        style("›").dim(),
        style("Run kn help to see all available commands").dim(),
    ));

    error_msg
}

fn format_error(message: &str) -> String {
    format!("  {} {}", style("✖").red().bold(), style(message).red())
}

fn format_error_with_suggestion(message: &str, suggestion: &str) -> String {
    format!(
        "  {} {}\n  {} {}",
        style("✖").red().bold(),
        style(message).red(),
        style("└").dim(),
        style(suggestion).dim(),
    )
}

#[derive(Debug)]
pub struct Cli {
    pub cwd: Option<String>,
    pub command: Commands,
}

#[derive(Debug)]
pub enum Commands {
    Install {
        packages: Vec<String>,
        dev: bool,
        global: bool,
        exact: bool,
        ignore_scripts: bool,
    },
    Run {
        script_name: Option<String>,
        args: Vec<String>,
        if_present: bool,
    },
    Uninstall {
        packages: Vec<String>,
        global: bool,
    },
    Execute {
        command: String,
        args: Vec<String>,
    },
    Upgrade {
        packages: Vec<String>,
        interactive: bool,
        latest: bool,
    },
    UpgradeSelf,
    CleanInstall {
        force: bool,
        no_optional: bool,
    },
    List {
        json: bool,
    },
    Info {
        verbose: bool,
    },
    Watch {
        script_name: String,
        patterns: Vec<String>,
    },
    Clean {
        cache: bool,
        all: bool,
        global: bool,
    },
    View {
        package: String,
        version: Option<String>,
    },
    Help,
    Version,
}

impl Cli {
    pub fn parse() -> Result<Self, String> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            return Ok(Cli {
                cwd: None,
                command: Commands::Help,
            });
        }

        let mut i = 1;
        let mut cwd: Option<String> = None;

        // Check for -C flag
        if args.len() > i && args[i] == "-C" {
            if args.len() <= i + 1 {
                return Err(format_error_with_suggestion(
                    "Expected directory after -C flag",
                    "Usage: kn -C <directory> <command>",
                ));
            }
            cwd = Some(args[i + 1].clone());
            i += 2;
        }

        if args.len() <= i {
            return Ok(Cli {
                cwd,
                command: Commands::Help,
            });
        }

        let command = match args[i].as_str() {
            "install" | "i" | "add" => {
                i += 1;
                parse_install_command(&args, &mut i)?
            }
            "run" | "r" => {
                i += 1;
                parse_run_command(&args, &mut i)?
            }
            "uninstall" | "remove" | "rm" => {
                i += 1;
                parse_uninstall_command(&args, &mut i)?
            }
            "execute" | "exec" | "x" => {
                i += 1;
                parse_execute_command(&args, &mut i)?
            }
            "upgrade" | "update" | "up" => {
                i += 1;
                parse_upgrade_command(&args, &mut i)?
            }
            "upgrade-self" => Commands::UpgradeSelf,
            "clean-install" | "ci" => {
                i += 1;
                parse_clean_install_command(&args, &mut i)?
            }
            "list" | "ls" => {
                i += 1;
                parse_list_command(&args, &mut i)?
            }
            "info" | "env" => {
                i += 1;
                parse_info_command(&args, &mut i)?
            }
            "watch" | "w" => {
                i += 1;
                parse_watch_command(&args, &mut i)?
            }
            "clean" => {
                i += 1;
                parse_clean_command(&args, &mut i)?
            }
            "view" => {
                i += 1;
                parse_view_command(&args, &mut i)?
            }
            "help" | "--help" | "-h" => Commands::Help,
            "--version" | "-v" | "-V" => Commands::Version,
            _ => return Err(format_unknown_command_error(&args[i])),
        };

        Ok(Cli { cwd, command })
    }
}

fn parse_install_command(args: &[String], i: &mut usize) -> Result<Commands, String> {
    let mut packages = Vec::new();
    let mut dev = false;
    let mut global = false;
    let mut exact = false;
    let mut ignore_scripts = false;

    while *i < args.len() {
        match args[*i].as_str() {
            "--save-dev" | "-D" => dev = true,
            "-g" | "--global" => global = true,
            "--save-exact" | "-E" => exact = true,
            "--ignore-scripts" => ignore_scripts = true,
            arg if arg.starts_with('-') => {
                return Err(format_error_with_suggestion(
                    &format!("Unknown flag for install: {}", arg),
                    "Valid flags: -D (--save-dev), -g (--global), -E (--save-exact), --ignore-scripts",
                ));
            }
            _ => packages.push(args[*i].clone()),
        }
        *i += 1;
    }

    // Allow install without packages (installs all dependencies from package.json)
    Ok(Commands::Install {
        packages,
        dev,
        global,
        exact,
        ignore_scripts,
    })
}

fn parse_run_command(args: &[String], i: &mut usize) -> Result<Commands, String> {
    let mut script_name = None;
    let mut script_args = Vec::new();
    let mut if_present = false;

    while *i < args.len() {
        let arg = args[*i].as_str();

        // Once we have a script name, all remaining args (including flags) go to the script
        if script_name.is_some() {
            script_args.push(args[*i].clone());
        } else {
            // Before script name, only parse kn's own flags
            match arg {
                "--if-present" => if_present = true,
                arg if arg.starts_with('-') => {
                    return Err(format_error_with_suggestion(
                        &format!("Unknown flag for run: {}", arg),
                        "Valid flag: --if-present (must come before script name)",
                    ));
                }
                _ => {
                    // This is the script name
                    script_name = Some(args[*i].clone());
                }
            }
        }
        *i += 1;
    }

    Ok(Commands::Run {
        script_name,
        args: script_args,
        if_present,
    })
}

fn parse_uninstall_command(args: &[String], i: &mut usize) -> Result<Commands, String> {
    let mut packages = Vec::new();
    let mut global = false;

    while *i < args.len() {
        match args[*i].as_str() {
            "-g" | "--global" => global = true,
            arg if arg.starts_with('-') => {
                return Err(format_error_with_suggestion(
                    &format!("Unknown flag for uninstall: {}", arg),
                    "Valid flag: -g (--global)",
                ));
            }
            _ => packages.push(args[*i].clone()),
        }
        *i += 1;
    }

    if packages.is_empty() {
        return Err(format_error_with_suggestion(
            "Uninstall command requires at least one package name",
            "Usage: kn uninstall <package> [options]",
        ));
    }

    Ok(Commands::Uninstall { packages, global })
}

fn parse_execute_command(args: &[String], i: &mut usize) -> Result<Commands, String> {
    if *i >= args.len() {
        return Err(format_error_with_suggestion(
            "Execute command requires a command to execute",
            "Usage: kn execute <command> [args...]",
        ));
    }

    let command = args[*i].clone();
    *i += 1;

    let mut exec_args = Vec::new();
    while *i < args.len() {
        exec_args.push(args[*i].clone());
        *i += 1;
    }

    Ok(Commands::Execute {
        command,
        args: exec_args,
    })
}

fn parse_upgrade_command(args: &[String], i: &mut usize) -> Result<Commands, String> {
    let mut packages = Vec::new();
    let mut interactive = false;
    let mut latest = false;

    while *i < args.len() {
        match args[*i].as_str() {
            "-i" | "--interactive" => interactive = true,
            "--latest" => latest = true,
            arg if arg.starts_with('-') => {
                return Err(format_error_with_suggestion(
                    &format!("Unknown flag for upgrade: {}", arg),
                    "Valid flags: -i (--interactive), --latest",
                ));
            }
            _ => packages.push(args[*i].clone()),
        }
        *i += 1;
    }

    Ok(Commands::Upgrade {
        packages,
        interactive,
        latest,
    })
}

fn parse_clean_install_command(args: &[String], i: &mut usize) -> Result<Commands, String> {
    let mut force = false;
    let mut no_optional = false;

    while *i < args.len() {
        match args[*i].as_str() {
            "--force" => force = true,
            "--no-optional" => no_optional = true,
            arg if arg.starts_with('-') => {
                return Err(format_error_with_suggestion(
                    &format!("Unknown flag for clean-install: {}", arg),
                    "Valid flags: --force, --no-optional",
                ));
            }
            _ => {
                return Err(format_error(&format!(
                    "Unexpected argument for clean-install: {}",
                    args[*i]
                )))
            }
        }
        *i += 1;
    }

    Ok(Commands::CleanInstall { force, no_optional })
}

fn parse_list_command(args: &[String], i: &mut usize) -> Result<Commands, String> {
    let mut json = false;

    while *i < args.len() {
        match args[*i].as_str() {
            "--json" => json = true,
            arg if arg.starts_with('-') => {
                return Err(format_error_with_suggestion(
                    &format!("Unknown flag for list: {}", arg),
                    "Valid flag: --json",
                ));
            }
            _ => {
                return Err(format_error(&format!(
                    "Unexpected argument for list: {}",
                    args[*i]
                )))
            }
        }
        *i += 1;
    }

    Ok(Commands::List { json })
}

fn parse_info_command(args: &[String], i: &mut usize) -> Result<Commands, String> {
    let mut verbose = false;

    while *i < args.len() {
        match args[*i].as_str() {
            "--verbose" | "-v" => verbose = true,
            arg if arg.starts_with('-') => {
                return Err(format_error_with_suggestion(
                    &format!("Unknown flag for info: {}", arg),
                    "Valid flag: -v (--verbose)",
                ));
            }
            _ => {
                return Err(format_error(&format!(
                    "Unexpected argument for info: {}",
                    args[*i]
                )))
            }
        }
        *i += 1;
    }

    Ok(Commands::Info { verbose })
}

fn parse_watch_command(args: &[String], i: &mut usize) -> Result<Commands, String> {
    if *i >= args.len() {
        return Err(format_error_with_suggestion(
            "Watch command requires a script name",
            "Usage: kn watch <script-name> [patterns...]",
        ));
    }

    let script_name = args[*i].clone();
    *i += 1;

    let mut patterns = Vec::new();
    while *i < args.len() {
        patterns.push(args[*i].clone());
        *i += 1;
    }

    Ok(Commands::Watch {
        script_name,
        patterns,
    })
}

fn parse_clean_command(args: &[String], i: &mut usize) -> Result<Commands, String> {
    let mut cache = false;
    let mut all = false;
    let mut global = false;

    while *i < args.len() {
        match args[*i].as_str() {
            "--cache" => cache = true,
            "--all" => all = true,
            "--global" | "-g" => global = true,
            arg if arg.starts_with('-') => {
                return Err(format_error(&format!("Unknown flag for clean: {}", arg)));
            }
            _ => {
                return Err(format_error(&format!(
                    "Unexpected argument for clean: {}",
                    args[*i]
                )))
            }
        }
        *i += 1;
    }

    Ok(Commands::Clean { cache, all, global })
}

fn parse_view_command(args: &[String], i: &mut usize) -> Result<Commands, String> {
    if *i >= args.len() {
        return Err(format_error_with_suggestion(
            "View command requires a package name",
            "Usage: kn view <package> [version]",
        ));
    }

    let package = args[*i].clone();
    *i += 1;

    let version = if *i < args.len() && !args[*i].starts_with('-') {
        let v = Some(args[*i].clone());
        *i += 1;
        v
    } else {
        None
    };

    Ok(Commands::View { package, version })
}
