use inquire::Select;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};
use std::{env, io};

use crate::agents::Agent;
use crate::config::{get_default_agent, get_global_agent, DefaultAgent};
use crate::detect::{detect};
use crate::agents::AGENT_MAP;
use crate::display::StyledOutput;

#[derive(Clone)]
pub struct DetectOptions {
    pub cwd: PathBuf,
    pub auto_install: bool,
    pub programmatic: bool,
}
impl Default for DetectOptions {
    fn default() -> Self {
        DetectOptions {
            cwd: env::current_dir().unwrap(),
            auto_install: false,
            programmatic: false,
        }
    }
}
impl DetectOptions {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_auto_install(mut self, auto_install: bool) -> Self {
        self.auto_install = auto_install;
        self
    }
}

#[allow(dead_code)]
pub struct RunnerContext {
    pub programmatic: bool,
    pub has_lock: bool,
    pub cwd: PathBuf,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct RunnerContextClone {
    pub programmatic: bool,
    pub has_lock: bool,
    pub cwd: PathBuf,
}

pub type Runner =
    fn(agent: Agent, args: Vec<String>, ctx: Option<RunnerContext>) -> (String, Vec<String>);

pub fn run_cli(func: Runner, options: Option<DetectOptions>, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut options = options.unwrap_or_default();

    run(func, args, &mut options)?;
    Ok(())
}

pub fn run(func: Runner, args: Vec<String>, options: &mut DetectOptions) -> Result<(), Box<dyn std::error::Error>> {
    let version = env!("CARGO_PKG_VERSION");

    let mut args = args;
    if args.len() > 2 && args[0] == "-C" {
        let path = Path::new(args[1].as_str());
        options.cwd = if path.is_absolute() {
            path.to_path_buf()
        } else {
            options.cwd.join(path)
        };
        args = args[0..2].to_vec();
    }

    if args.len() == 1 && (args[0].to_lowercase() == "-v" || args[0] == "--version") {
        StyledOutput::package_info("kn", &format!("v{}", version), "blazing fast");
        return Ok(());
    }
    if args.len() == 1 && (args[0] == "-h" || args[0] == "--help") {
        StyledOutput::opencode_header();
        println!("v{}\n", version);
        println!("USAGE:");
        println!("    kn <command> [args...]  # All commands");
        println!();
        StyledOutput::section_title("Commands");
        StyledOutput::command_example("install, i", "Install packages");
        StyledOutput::command_example("run, r", "Run scripts");
        StyledOutput::command_example("uninstall", "Uninstall packages");
        StyledOutput::command_example("exec, x", "Execute packages");
        StyledOutput::command_example("upgrade, update", "Upgrade dependencies");
        StyledOutput::command_example("clean-install, ci", "Clean install");
        StyledOutput::command_example("agent", "Run package manager");
        StyledOutput::command_example("list", "List scripts");
        StyledOutput::command_example("info", "Show package manager info");
        println!();
        StyledOutput::section_title("Examples");
        StyledOutput::command_example("kn i react", "Install react");
        StyledOutput::command_example("kn r dev", "Run dev script");
        StyledOutput::command_example("kn uninstall webpack", "Uninstall webpack");
        StyledOutput::command_example("kn exec tsc", "Execute typescript");
        StyledOutput::command_example("kn", "List all scripts");
        println!();
        StyledOutput::info("For more documentation, visit: https://github.com/wangsizhu0504/kn");
        return Ok(());
    }

    let command = get_cli_command(func, args.clone(), options.clone());

    if let Ok(Some((agent, args))) = command {
        execa_command(&agent, Some(args))?;
        Ok(())
    } else {
        Ok(())
    }
}

type CommandResult = Result<Option<(String, Vec<String>)>, Box<dyn std::error::Error>>;

// Public function to get command without executing
pub fn get_cli_command_direct(
    func: Runner,
    args: Vec<String>,
    options: DetectOptions,
) -> CommandResult {
    get_cli_command(func, args, options)
}

fn get_cli_command(
    func: Runner,
    args: Vec<String>,
    options: DetectOptions,
) -> CommandResult {
    let global = "-g".to_string();
    if args.contains(&global) {
        return Ok(Some(func(get_global_agent(), args, None)));
    }
    let mut agent = if let Some(v) = detect(options.clone()) {
        DefaultAgent::Agent(v)
    } else {
        get_default_agent(options.programmatic)
    };

    if agent == DefaultAgent::Prompt {
        let items: Vec<&str> = AGENT_MAP.iter().filter(|(name, _)| !name.contains("@")).map(|(name, _)| *name).collect();
        let selection = Select::new("script to run:", items).prompt();
        if let Ok(selection) = selection {
            if let Some(agent_value) = AGENT_MAP.iter().find(|(name, _)| *name == selection).map(|(_, agent)| *agent) {
                agent = DefaultAgent::Agent(agent_value);
            } else {
                return Ok(None);
            }
        } else {
            process::exit(1)
        }
    }
    let runner_ctx = RunnerContext {
        programmatic: options.programmatic,
        has_lock: true,
        cwd: options.cwd,
    };
    match agent {
        DefaultAgent::Agent(agent) => Ok(Some(func(agent, args, Some(runner_ctx)))),
        DefaultAgent::Prompt => Ok(Some(func(Agent::Npm, args, Some(runner_ctx)))),
    }
}

pub fn execa_command(agent: &str, args: Option<Vec<String>>) -> Result<(), io::Error> {
    let mut command = Command::new(agent)
        .args(args.unwrap_or_default())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute command");

    if let Some(stdout) = command.stdout.take() {
        let reader = io::BufReader::new(stdout);
        for line in reader.lines().flatten() {
            println!("{}", line);
        }
    }

    command.wait()?;

    Ok(())
}
