// Custom CLI parser without external dependencies
use std::env;

// Calculate Levenshtein distance for string similarity
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(
                    matrix[i][j + 1] + 1,      // deletion
                    matrix[i + 1][j] + 1,      // insertion
                ),
                matrix[i][j] + cost,           // substitution
            );
        }
    }

    matrix[len1][len2]
}

// Find similar commands based on user input
fn find_similar_commands(input: &str) -> Vec<(String, usize)> {
    let all_commands = vec![
        "install", "i", "add",
        "run", "r",
        "uninstall", "remove", "rm",
        "execute", "exec", "x",
        "upgrade", "update", "up",
        "clean-install", "ci",
        "agent", "npm", "yarn", "pnpm", "bun",
        "list", "ls",
        "info", "env",
        "watch", "w",
        "history", "hist",
        "alias",
        "stats",
        "parallel", "p",
        "clean",
        "analyze",
        "help",
    ];

    let mut similarities: Vec<(String, usize)> = all_commands
        .iter()
        .map(|&cmd| (cmd.to_string(), levenshtein_distance(input, cmd)))
        .collect();

    // Sort by distance (smaller is more similar)
    similarities.sort_by_key(|(_, dist)| *dist);

    // Return top 3 most similar commands with distance <= 3
    similarities
        .into_iter()
        .filter(|(_, dist)| *dist <= 3)
        .take(3)
        .collect()
}

// Format error message with suggestions
fn format_unknown_command_error(input: &str) -> String {
    let similar = find_similar_commands(input);

    let mut error_msg = format!("\x1b[31m✗ Error:\x1b[0m Unknown command: \x1b[33m{}\x1b[0m\n", input);

    if !similar.is_empty() {
        error_msg.push_str("\n\x1b[36m💡 Did you mean:\x1b[0m\n");
        for (cmd, _) in similar {
            error_msg.push_str(&format!("  \x1b[32m→\x1b[0m kn \x1b[36m{}\x1b[0m\n", cmd));
        }
    }

    error_msg.push_str("\n\x1b[90mRun \x1b[36mkn help\x1b[90m to see all available commands.\x1b[0m\n");

    error_msg
}

// Format general error message
fn format_error(message: &str) -> String {
    format!("\x1b[31m✗ Error:\x1b[0m {}", message)
}

// Format error with suggestion
fn format_error_with_suggestion(message: &str, suggestion: &str) -> String {
    format!("\x1b[31m✗ Error:\x1b[0m {}\n\x1b[36m💡 Suggestion:\x1b[0m {}", message, suggestion)
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
    CleanInstall {
        force: bool,
        no_optional: bool,
    },
    Agent {
        manager: Option<String>,
        args: Vec<String>,
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
    History {
        count: Option<usize>,
    },
    HistoryRun {
        index: usize,
    },
    HistoryLast,
    Alias {
        action: Option<String>,
        key: Option<String>,
        value: Option<String>,
    },
    Stats,
    Parallel {
        scripts: Vec<String>,
    },
    Clean {
        cache: bool,
        all: bool,
        global: bool,
    },
    Analyze,
    Help,
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
                    "Usage: kn -C <directory> <command>"
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
            "clean-install" | "ci" => {
                i += 1;
                parse_clean_install_command(&args, &mut i)?
            }
            "agent" | "npm" | "yarn" | "pnpm" | "bun" => {
                let agent_name = args[i].clone();
                i += 1;
                parse_agent_command(&args, &mut i, &agent_name)?
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
            "history" | "hist" => {
                i += 1;
                parse_history_command(&args, &mut i)?
            }
            "!!" => Commands::HistoryLast,
            cmd if cmd.starts_with('!') && cmd.len() > 1 => {
                // Parse !N format
                if let Ok(index) = cmd[1..].parse::<usize>() {
                    Commands::HistoryRun { index }
                } else {
                    return Err(format_error("Invalid history index"));
                }
            }
            "alias" => {
                i += 1;
                parse_alias_command(&args, &mut i)?
            }
            "stats" => Commands::Stats,
            "parallel" | "p" => {
                i += 1;
                parse_parallel_command(&args, &mut i)?
            }
            "clean" => {
                i += 1;
                parse_clean_command(&args, &mut i)?
            }
            "analyze" => Commands::Analyze,
            "help" | "--help" | "-h" => Commands::Help,
            "--version" | "-v" => {
                println!("kn version {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
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

    while *i < args.len() {
        match args[*i].as_str() {
            "--save-dev" | "-D" => dev = true,
            "-g" | "--global" => global = true,
            "--save-exact" | "-E" => exact = true,
            arg if arg.starts_with('-') => {
                return Err(format_error_with_suggestion(
                    &format!("Unknown flag for install: {}", arg),
                    "Valid flags: -D (--save-dev), -g (--global), -E (--save-exact)"
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
    })
}

fn parse_run_command(args: &[String], i: &mut usize) -> Result<Commands, String> {
    let mut script_name = None;
    let mut script_args = Vec::new();
    let mut if_present = false;

    while *i < args.len() {
        match args[*i].as_str() {
            "--if-present" => if_present = true,
            arg if arg.starts_with('-') => {
                return Err(format_error_with_suggestion(
                    &format!("Unknown flag for run: {}", arg),
                    "Valid flag: --if-present"
                ));
            }
            _ => {
                if script_name.is_none() {
                    script_name = Some(args[*i].clone());
                } else {
                    script_args.push(args[*i].clone());
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
                    "Valid flag: -g (--global)"
                ));
            }
            _ => packages.push(args[*i].clone()),
        }
        *i += 1;
    }

    if packages.is_empty() {
        return Err(format_error_with_suggestion(
            "Uninstall command requires at least one package name",
            "Usage: kn uninstall <package> [options]"
        ));
    }

    Ok(Commands::Uninstall { packages, global })
}

fn parse_execute_command(args: &[String], i: &mut usize) -> Result<Commands, String> {
    if *i >= args.len() {
        return Err(format_error_with_suggestion(
            "Execute command requires a command to execute",
            "Usage: kn execute <command> [args...]"
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
                    "Valid flags: -i (--interactive), --latest"
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
                    "Valid flags: --force, --no-optional"
                ));
            }
            _ => return Err(format_error(&format!("Unexpected argument for clean-install: {}", args[*i]))),
        }
        *i += 1;
    }

    Ok(Commands::CleanInstall { force, no_optional })
}

fn parse_agent_command(args: &[String], i: &mut usize, agent_name: &str) -> Result<Commands, String> {
    let manager = match agent_name {
        "npm" | "yarn" | "pnpm" | "bun" => Some(agent_name.to_string()),
        _ => None,
    };

    let mut agent_args = Vec::new();
    while *i < args.len() {
        agent_args.push(args[*i].clone());
        *i += 1;
    }

    if agent_args.is_empty() {
        return Err(format_error_with_suggestion(
            "Agent command requires arguments",
            "Usage: kn agent [npm|yarn|pnpm|bun] <args...>"
        ));
    }

    Ok(Commands::Agent {
        manager,
        args: agent_args,
    })
}

fn parse_list_command(args: &[String], i: &mut usize) -> Result<Commands, String> {
    let mut json = false;

    while *i < args.len() {
        match args[*i].as_str() {
            "--json" => json = true,
            arg if arg.starts_with('-') => {
                return Err(format_error_with_suggestion(
                    &format!("Unknown flag for list: {}", arg),
                    "Valid flag: --json"
                ));
            }
            _ => return Err(format_error(&format!("Unexpected argument for list: {}", args[*i]))),
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
                    "Valid flag: -v (--verbose)"
                ));
            }
            _ => return Err(format_error(&format!("Unexpected argument for info: {}", args[*i]))),
        }
        *i += 1;
    }

    Ok(Commands::Info { verbose })
}

fn parse_watch_command(args: &[String], i: &mut usize) -> Result<Commands, String> {
    if *i >= args.len() {
        return Err(format_error_with_suggestion(
            "Watch command requires a script name",
            "Usage: kn watch <script-name> [patterns...]"
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

fn parse_history_command(args: &[String], i: &mut usize) -> Result<Commands, String> {
    let count = if *i < args.len() {
        match args[*i].parse::<usize>() {
            Ok(n) => {
                *i += 1;
                Some(n)
            }
            Err(_) => None,
        }
    } else {
        None
    };

    Ok(Commands::History { count })
}

fn parse_alias_command(args: &[String], i: &mut usize) -> Result<Commands, String> {
    let action = if *i < args.len() {
        Some(args[*i].clone())
    } else {
        None
    };

    if action.as_deref() == Some("set") || action.as_deref() == Some("add") {
        *i += 1;
        let key = if *i < args.len() {
            let k = args[*i].clone();
            *i += 1;
            Some(k)
        } else {
            None
        };

        let value = if *i < args.len() {
            let v = args[*i].clone();
            *i += 1;
            Some(v)
        } else {
            None
        };

        Ok(Commands::Alias { action, key, value })
    } else if action.as_deref() == Some("remove") || action.as_deref() == Some("rm") || action.as_deref() == Some("delete") {
        *i += 1;
        let key = if *i < args.len() {
            let k = args[*i].clone();
            *i += 1;
            Some(k)
        } else {
            None
        };

        Ok(Commands::Alias { action, key, value: None })
    } else {
        if action.is_some() {
            *i += 1;
        }
        Ok(Commands::Alias { action, key: None, value: None })
    }
}

fn parse_parallel_command(args: &[String], i: &mut usize) -> Result<Commands, String> {
    let mut scripts = Vec::new();

    while *i < args.len() && !args[*i].starts_with('-') {
        scripts.push(args[*i].clone());
        *i += 1;
    }

    Ok(Commands::Parallel { scripts })
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
            _ => return Err(format_error(&format!("Unexpected argument for clean: {}", args[*i]))),
        }
        *i += 1;
    }

    Ok(Commands::Clean { cache, all, global })
}
