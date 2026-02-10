use crate::agents::Agent;
use crate::detect::detect;
use crate::runner::DetectOptions;
use dirs::home_dir;
use ini::Ini;
use std::{
    env,
    path::{Path, PathBuf},
};

#[derive(PartialEq, Clone, Debug, Eq, Hash)]
pub enum DefaultAgent {
    Prompt,
    Agent(Agent),
}

pub struct Config {
    default_agent: DefaultAgent,
    global_agent: Agent,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            default_agent: DefaultAgent::Prompt,
            global_agent: Agent::Npm,
        }
    }
}

impl Config {
    pub fn assign(&self) -> Self {
        let home = home_dir().unwrap_or_else(|| PathBuf::from("~/"));
        let custom_rc_path = env::var("KN_CONFIG_FILE");
        let default_rc_path = home.join(".knrc");
        let rc_path = if let Ok(custom_rc_path) = custom_rc_path {
            custom_rc_path
        } else {
            default_rc_path.to_string_lossy().to_string()
        };

        let mut config = Self::default();

        if Path::new(&rc_path).exists() {
            if let Ok(conf) = Ini::load_from_file(&rc_path) {
                if let Some(section) = conf.section(None::<String>) {
                    if let Some(default_agent) = section.get("default_agent") {
                        if let Some(agent) = Agent::from_name(default_agent) {
                            config.default_agent = DefaultAgent::Agent(agent);
                        }
                    }
                    if let Some(global_agent) = section.get("global_agent") {
                        if let Some(agent) = Agent::from_name(global_agent) {
                            config.global_agent = agent;
                        }
                    }
                }
            }
        }

        config
    }
}

pub fn get_config() -> Config {
    let mut config = Config::default().assign();
    let options = DetectOptions {
        programmatic: true,
        ..DetectOptions::default()
    };
    let agent = detect(options);
    if let Some(agent) = agent {
        config.default_agent = DefaultAgent::Agent(agent);
    }
    config
}

pub fn get_default_agent(programmatic: bool) -> DefaultAgent {
    let Config { default_agent, .. } = get_config();
    let ci = env::var("CI");

    if default_agent == DefaultAgent::Prompt && (programmatic || ci.is_ok()) {
        return DefaultAgent::Agent(Agent::Npm);
    }
    default_agent
}

pub fn get_global_agent() -> Agent {
    let Config { global_agent, .. } = get_config();
    global_agent
}
