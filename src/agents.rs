use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Agent {
    Npm,
    Yarn,
    YarnBerry,
    Pnpm,
    Pnpm6,
    Bun,
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Agent {
    /// Resolve an Agent from a package manager name string
    pub fn from_name(name: &str) -> Option<Self> {
        AGENT_MAP
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, agent)| *agent)
    }

    /// The binary name used to invoke this package manager
    pub fn name(&self) -> &'static str {
        match self {
            Agent::Npm => "npm",
            Agent::Yarn | Agent::YarnBerry => "yarn",
            Agent::Pnpm | Agent::Pnpm6 => "pnpm",
            Agent::Bun => "bun",
        }
    }

    /// Emoji icon for display
    #[allow(dead_code)]
    pub fn icon(&self) -> &'static str {
        match self {
            Agent::Npm => "npm",
            Agent::Yarn | Agent::YarnBerry => "yarn",
            Agent::Pnpm | Agent::Pnpm6 => "pnpm",
            Agent::Bun => "bun",
        }
    }

    /// The subcommand for adding packages (e.g., "install" for npm, "add" for others)
    pub fn add_cmd(&self) -> &'static str {
        match self {
            Agent::Npm => "install",
            _ => "add",
        }
    }

    /// The subcommand for removing packages
    pub fn remove_cmd(&self) -> &'static str {
        match self {
            Agent::Npm => "uninstall",
            _ => "remove",
        }
    }

    /// The subcommand for upgrading packages
    pub fn upgrade_cmd(&self) -> &'static str {
        match self {
            Agent::Npm | Agent::Pnpm | Agent::Pnpm6 | Agent::Bun => "update",
            Agent::Yarn => "upgrade",
            Agent::YarnBerry => "up",
        }
    }

    /// The binary used for executing package binaries (npx, bunx, etc.)
    pub fn exec_binary(&self) -> &'static str {
        match self {
            Agent::Npm => "npx",
            Agent::Bun => "bunx",
            _ => self.name(),
        }
    }

    /// Whether the execute command uses `dlx` subcommand
    pub fn exec_uses_dlx(&self) -> bool {
        matches!(
            self,
            Agent::Yarn | Agent::YarnBerry | Agent::Pnpm | Agent::Pnpm6
        )
    }

    /// Arguments for frozen/clean install
    pub fn frozen_args(&self) -> Vec<&'static str> {
        match self {
            Agent::Npm => vec!["ci"],
            Agent::Yarn => vec!["install", "--frozen-lockfile"],
            Agent::YarnBerry => vec!["install", "--immutable"],
            Agent::Pnpm | Agent::Pnpm6 => vec!["install", "--frozen-lockfile"],
            Agent::Bun => vec!["install", "--no-save"],
        }
    }

    /// Whether global commands use a prefix like `yarn global add` vs `-g` flag
    pub fn uses_global_prefix(&self) -> bool {
        matches!(self, Agent::Yarn | Agent::YarnBerry)
    }

    /// Whether interactive upgrade is supported
    pub fn supports_interactive_upgrade(&self) -> bool {
        matches!(
            self,
            Agent::Yarn | Agent::YarnBerry | Agent::Pnpm | Agent::Pnpm6
        )
    }

    /// Arguments for interactive upgrade
    pub fn interactive_upgrade_args(&self) -> Vec<&'static str> {
        match self {
            Agent::Yarn => vec!["upgrade-interactive"],
            Agent::YarnBerry => vec!["up", "-i"],
            Agent::Pnpm | Agent::Pnpm6 => vec!["update", "-i"],
            _ => vec![],
        }
    }

    /// Cache clean command and args
    pub fn cache_clean_args(&self) -> Option<(&'static str, Vec<&'static str>)> {
        match self {
            Agent::Npm => Some(("npm", vec!["cache", "clean", "--force"])),
            Agent::Yarn | Agent::YarnBerry => Some(("yarn", vec!["cache", "clean"])),
            Agent::Pnpm | Agent::Pnpm6 => Some(("pnpm", vec!["store", "prune"])),
            Agent::Bun => None, // Bun doesn't support cache clean yet
        }
    }

    /// Global list command and args
    pub fn global_list_args(&self) -> (&'static str, Vec<&'static str>) {
        match self {
            Agent::Npm => ("npm", vec!["list", "-g", "--depth=0"]),
            Agent::Yarn | Agent::YarnBerry => ("yarn", vec!["global", "list"]),
            Agent::Pnpm | Agent::Pnpm6 => ("pnpm", vec!["list", "-g"]),
            Agent::Bun => ("bun", vec!["pm", "ls", "-g"]),
        }
    }
}

pub const AGENT_MAP: &[(&str, Agent)] = &[
    ("npm", Agent::Npm),
    ("yarn", Agent::Yarn),
    ("pnpm", Agent::Pnpm),
    ("bun", Agent::Bun),
    ("yarn@berry", Agent::YarnBerry),
    ("pnpm@6", Agent::Pnpm6),
];
