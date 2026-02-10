use anyhow::{Context, Result};
use inquire::Select;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};
use std::env;
use tracing::{debug, info, warn};

use crate::agents::Agent;
use crate::agents::AGENT_MAP;
use crate::config::{get_default_agent, get_global_agent, DefaultAgent};
use crate::detect::detect;
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
            cwd: env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
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

pub type Runner =
    fn(agent: Agent, args: Vec<String>, ctx: Option<RunnerContext>) -> (String, Vec<String>);

pub fn run_cli(
    func: Runner,
    options: Option<DetectOptions>,
    args: Vec<String>,
) -> Result<()> {
    let mut options = options.unwrap_or_default();
    run(func, args, &mut options)
}

pub fn run(
    func: Runner,
    args: Vec<String>,
    options: &mut DetectOptions,
) -> Result<()> {
    debug!("Running command with args: {:?}", args);

    let mut args = args;

    // Handle -C <directory> flag: skip first two args and update cwd
    if args.len() > 2 && args[0] == "-C" {
        let path = Path::new(args[1].as_str());
        options.cwd = if path.is_absolute() {
            path.to_path_buf()
        } else {
            options.cwd.join(path)
        };
        args = args[2..].to_vec(); // Fixed: skip -C and path, keep remaining args
    }

    if args.len() == 1 && (args[0].to_lowercase() == "-v" || args[0] == "--version") {
        StyledOutput::brand();
        return Ok(());
    }
    if args.len() == 1 && (args[0] == "-h" || args[0] == "--help") {
        StyledOutput::opencode_header();
        return Ok(());
    }

    let command = get_cli_command(func, args, options.clone())?;

    if let Some((agent, cmd_args)) = command {
        execa_command(&agent, cmd_args)?;
    }

    Ok(())
}

fn get_cli_command(
    func: Runner,
    args: Vec<String>,
    options: DetectOptions,
) -> Result<Option<(String, Vec<String>)>> {
    debug!("Getting CLI command with args: {:?}", args);

    let global = "-g".to_string();
    if args.contains(&global) {
        info!("Using global agent for command");
        return Ok(Some(func(get_global_agent(), args, None)));
    }

    let mut agent = if let Some(v) = detect(options.clone()) {
        DefaultAgent::Agent(v)
    } else {
        get_default_agent(options.programmatic)
    };
    debug!("Selected agent: {:?}", agent);

    if agent == DefaultAgent::Prompt {
        let items: Vec<&str> = AGENT_MAP
            .iter()
            .filter(|(name, _)| !name.contains('@'))
            .map(|(name, _)| *name)
            .collect();
        let selection = Select::new("Choose a package manager:", items).prompt();
        match selection {
            Ok(selection) => {
                info!("User selected agent: {}", selection);
                if let Some(agent_value) = Agent::from_name(selection) {
                    agent = DefaultAgent::Agent(agent_value);
                } else {
                    warn!("Invalid agent selection");
                    return Ok(None);
                }
            }
            Err(_) => {
                process::exit(1);
            }
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

/// Execute a command and propagate its exit code
pub fn execa_command(agent: &str, args: Vec<String>) -> Result<()> {
    info!("Executing command: {} {:?}", agent, args);

    let status = Command::new(agent)
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .status()
        .with_context(|| format!("Failed to execute command: {} {:?}", agent, args))?;

    if !status.success() {
        let code = status.code().unwrap_or(1);
        process::exit(code);
    }

    Ok(())
}
