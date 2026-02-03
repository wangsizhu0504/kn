use crate::agents::Agent;
use crate::runner::RunnerContext;

/// Parse function for install commands (ni)
/// Maps to: npm install, yarn add, pnpm add, bun add
pub fn parse_ni(
    agent: Agent,
    args: Vec<String>,
    _ctx: Option<RunnerContext>,
) -> (String, Vec<String>) {
    let mut cmd_args = args;

    // Handle frozen install flag
    if let Some(index) = cmd_args.iter().position(|arg| arg == "--frozen") {
        cmd_args.remove(index);
        return match agent {
            Agent::Npm => ("npm".to_string(), vec!["ci".to_string()]),
            Agent::Yarn => (
                "yarn".to_string(),
                vec!["install".to_string(), "--frozen-lockfile".to_string()],
            ),
            Agent::YarnBerry => (
                "yarn".to_string(),
                vec!["install".to_string(), "--immutable".to_string()],
            ),
            Agent::Pnpm => (
                "pnpm".to_string(),
                vec!["install".to_string(), "--frozen-lockfile".to_string()],
            ),
            Agent::Pnpm6 => (
                "pnpm".to_string(),
                vec!["install".to_string(), "--frozen-lockfile".to_string()],
            ),
            Agent::Bun => (
                "bun".to_string(),
                vec!["install".to_string(), "--no-save".to_string()],
            ),
        };
    }

    // Check if global install
    if cmd_args.contains(&"-g".to_string()) {
        return match agent {
            Agent::Npm => {
                let mut args = vec!["install".to_string()];
                args.extend(cmd_args);
                ("npm".to_string(), args)
            }
            Agent::Yarn => {
                let mut args = vec!["global".to_string(), "add".to_string()];
                args.extend_from_slice(&cmd_args[1..]);
                ("yarn".to_string(), args)
            }
            Agent::YarnBerry => {
                let mut args = vec!["global".to_string(), "add".to_string()];
                args.extend_from_slice(&cmd_args[1..]);
                ("yarn".to_string(), args)
            }
            Agent::Pnpm => {
                let mut args = vec!["add".to_string()];
                args.extend(cmd_args);
                ("pnpm".to_string(), args)
            }
            Agent::Pnpm6 => {
                let mut args = vec!["add".to_string()];
                args.extend(cmd_args);
                ("pnpm".to_string(), args)
            }
            Agent::Bun => {
                let mut args = vec!["add".to_string()];
                args.extend(cmd_args);
                ("bun".to_string(), args)
            }
        };
    }

    // Regular install (add packages)
    if cmd_args.is_empty() {
        match agent {
            Agent::Npm => ("npm".to_string(), vec!["install".to_string()]),
            Agent::Yarn => ("yarn".to_string(), vec!["install".to_string()]),
            Agent::YarnBerry => ("yarn".to_string(), vec!["install".to_string()]),
            Agent::Pnpm => ("pnpm".to_string(), vec!["install".to_string()]),
            Agent::Pnpm6 => ("pnpm".to_string(), vec!["install".to_string()]),
            Agent::Bun => ("bun".to_string(), vec!["install".to_string()]),
        }
    } else {
        match agent {
            Agent::Npm => {
                let mut args = vec!["install".to_string()];
                args.extend(cmd_args);
                ("npm".to_string(), args)
            }
            Agent::Yarn => {
                let mut args = vec!["add".to_string()];
                args.extend(cmd_args);
                ("yarn".to_string(), args)
            }
            Agent::YarnBerry => {
                let mut args = vec!["add".to_string()];
                args.extend(cmd_args);
                ("yarn".to_string(), args)
            }
            Agent::Pnpm => {
                let mut args = vec!["add".to_string()];
                args.extend(cmd_args);
                ("pnpm".to_string(), args)
            }
            Agent::Pnpm6 => {
                let mut args = vec!["add".to_string()];
                args.extend(cmd_args);
                ("pnpm".to_string(), args)
            }
            Agent::Bun => {
                let mut args = vec!["add".to_string()];
                args.extend(cmd_args);
                ("bun".to_string(), args)
            }
        }
    }
}

/// Parse function for uninstall commands (nun)
/// Maps to: npm uninstall, yarn remove, pnpm remove, bun remove
pub fn parse_nun(
    agent: Agent,
    args: Vec<String>,
    _ctx: Option<RunnerContext>,
) -> (String, Vec<String>) {
    let cmd_args = args;

    // Check if global uninstall
    if cmd_args.contains(&"-g".to_string()) {
        return match agent {
            Agent::Npm => {
                let mut args = vec!["uninstall".to_string()];
                args.extend(cmd_args);
                ("npm".to_string(), args)
            }
            Agent::Yarn => {
                let mut args = vec!["global".to_string(), "remove".to_string()];
                args.extend_from_slice(&cmd_args[1..]);
                ("yarn".to_string(), args)
            }
            Agent::YarnBerry => {
                let mut args = vec!["global".to_string(), "remove".to_string()];
                args.extend_from_slice(&cmd_args[1..]);
                ("yarn".to_string(), args)
            }
            Agent::Pnpm => {
                let mut args = vec!["remove".to_string()];
                args.extend(cmd_args);
                ("pnpm".to_string(), args)
            }
            Agent::Pnpm6 => {
                let mut args = vec!["remove".to_string()];
                args.extend(cmd_args);
                ("pnpm".to_string(), args)
            }
            Agent::Bun => {
                let mut args = vec!["remove".to_string()];
                args.extend(cmd_args);
                ("bun".to_string(), args)
            }
        };
    }

    match agent {
        Agent::Npm => {
            let mut args = vec!["uninstall".to_string()];
            args.extend(cmd_args);
            ("npm".to_string(), args)
        }
        Agent::Yarn => {
            let mut args = vec!["remove".to_string()];
            args.extend(cmd_args);
            ("yarn".to_string(), args)
        }
        Agent::YarnBerry => {
            let mut args = vec!["remove".to_string()];
            args.extend(cmd_args);
            ("yarn".to_string(), args)
        }
        Agent::Pnpm => {
            let mut args = vec!["remove".to_string()];
            args.extend(cmd_args);
            ("pnpm".to_string(), args)
        }
        Agent::Pnpm6 => {
            let mut args = vec!["remove".to_string()];
            args.extend(cmd_args);
            ("pnpm".to_string(), args)
        }
        Agent::Bun => {
            let mut args = vec!["remove".to_string()];
            args.extend(cmd_args);
            ("bun".to_string(), args)
        }
    }
}

/// Parse function for execute commands (nlx)
/// Maps to: npx, yarn dlx, pnpm dlx, bunx
pub fn parse_nlx(
    agent: Agent,
    args: Vec<String>,
    _ctx: Option<RunnerContext>,
) -> (String, Vec<String>) {
    if args.is_empty() {
        eprintln!("Error: No command provided for execution");
        std::process::exit(1);
    }

    match agent {
        Agent::Npm => ("npx".to_string(), args),
        Agent::Yarn => {
            let mut dlx_args = vec!["dlx".to_string()];
            dlx_args.extend(args);
            ("yarn".to_string(), dlx_args)
        }
        Agent::YarnBerry => {
            let mut dlx_args = vec!["dlx".to_string()];
            dlx_args.extend(args);
            ("yarn".to_string(), dlx_args)
        }
        Agent::Pnpm => {
            let mut dlx_args = vec!["dlx".to_string()];
            dlx_args.extend(args);
            ("pnpm".to_string(), dlx_args)
        }
        Agent::Pnpm6 => {
            let mut dlx_args = vec!["dlx".to_string()];
            dlx_args.extend(args);
            ("pnpm".to_string(), dlx_args)
        }
        Agent::Bun => ("bunx".to_string(), args),
    }
}

/// Parse function for upgrade commands (nu)
/// Maps to: npm upgrade, yarn upgrade, pnpm update, bun update
pub fn parse_nu(
    agent: Agent,
    args: Vec<String>,
    _ctx: Option<RunnerContext>,
) -> (String, Vec<String>) {
    let mut cmd_args = args;

    // Handle interactive upgrade
    if let Some(index) = cmd_args
        .iter()
        .position(|arg| arg == "-i" || arg == "--interactive")
    {
        cmd_args.remove(index);
        return match agent {
            Agent::Npm => {
                eprintln!("Error: npm does not support interactive upgrades");
                std::process::exit(1);
            }
            Agent::Yarn => {
                let mut args = vec!["upgrade-interactive".to_string()];
                args.extend(cmd_args);
                ("yarn".to_string(), args)
            }
            Agent::YarnBerry => {
                let mut args = vec!["up".to_string(), "-i".to_string()];
                args.extend(cmd_args);
                ("yarn".to_string(), args)
            }
            Agent::Pnpm => {
                let mut args = vec!["update".to_string(), "-i".to_string()];
                args.extend(cmd_args);
                ("pnpm".to_string(), args)
            }
            Agent::Pnpm6 => {
                let mut args = vec!["update".to_string(), "-i".to_string()];
                args.extend(cmd_args);
                ("pnpm".to_string(), args)
            }
            Agent::Bun => {
                eprintln!("Error: bun does not support interactive upgrades");
                std::process::exit(1);
            }
        };
    }

    // Regular upgrade
    if cmd_args.is_empty() {
        match agent {
            Agent::Npm => ("npm".to_string(), vec!["update".to_string()]),
            Agent::Yarn => ("yarn".to_string(), vec!["upgrade".to_string()]),
            Agent::YarnBerry => ("yarn".to_string(), vec!["up".to_string()]),
            Agent::Pnpm => ("pnpm".to_string(), vec!["update".to_string()]),
            Agent::Pnpm6 => ("pnpm".to_string(), vec!["update".to_string()]),
            Agent::Bun => ("bun".to_string(), vec!["update".to_string()]),
        }
    } else {
        match agent {
            Agent::Npm => {
                let mut args = vec!["update".to_string()];
                args.extend(cmd_args);
                ("npm".to_string(), args)
            }
            Agent::Yarn => {
                let mut args = vec!["upgrade".to_string()];
                args.extend(cmd_args);
                ("yarn".to_string(), args)
            }
            Agent::YarnBerry => {
                let mut args = vec!["up".to_string()];
                args.extend(cmd_args);
                ("yarn".to_string(), args)
            }
            Agent::Pnpm => {
                let mut args = vec!["update".to_string()];
                args.extend(cmd_args);
                ("pnpm".to_string(), args)
            }
            Agent::Pnpm6 => {
                let mut args = vec!["update".to_string()];
                args.extend(cmd_args);
                ("pnpm".to_string(), args)
            }
            Agent::Bun => {
                let mut args = vec!["update".to_string()];
                args.extend(cmd_args);
                ("bun".to_string(), args)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::Agent;

    #[test]
    fn test_parse_ni() {
        let (cmd, args) = parse_ni(Agent::Npm, vec!["lodash".to_string()], None);
        assert_eq!(cmd, "npm");
        assert_eq!(args, vec!["install", "lodash"]);

        let (cmd, args) = parse_ni(
            Agent::Npm,
            vec!["-g".to_string(), "typescript".to_string()],
            None,
        );
        assert_eq!(cmd, "npm");
        assert!(args.contains(&"-g".to_string()));
        assert!(args.contains(&"typescript".to_string()));

        let (cmd, args) = parse_ni(Agent::Npm, vec!["--frozen".to_string()], None);
        assert_eq!(cmd, "npm");
        assert_eq!(args, vec!["ci"]);

        let (cmd, args) = parse_ni(Agent::Yarn, vec!["react".to_string()], None);
        assert_eq!(cmd, "yarn");
        assert_eq!(args, vec!["add", "react"]);

        let (cmd, args) = parse_ni(Agent::Bun, vec!["express".to_string()], None);
        assert_eq!(cmd, "bun");
        assert_eq!(args, vec!["add", "express"]);
    }

    #[test]
    fn test_parse_nlx() {
        let (cmd, args) = parse_nlx(
            Agent::Npm,
            vec!["cowsay".to_string(), "Hello".to_string()],
            None,
        );
        assert_eq!(cmd, "npx");
        assert_eq!(args, vec!["cowsay", "Hello"]);

        let (cmd, args) = parse_nlx(
            Agent::Yarn,
            vec!["cowsay".to_string(), "Hello".to_string()],
            None,
        );
        assert_eq!(cmd, "yarn");
        assert_eq!(args, vec!["dlx", "cowsay", "Hello"]);

        let (cmd, args) = parse_nlx(
            Agent::Bun,
            vec!["cowsay".to_string(), "Hello".to_string()],
            None,
        );
        assert_eq!(cmd, "bunx");
        assert_eq!(args, vec!["cowsay", "Hello"]);
    }

    #[test]
    fn test_parse_nun() {
        let (cmd, args) = parse_nun(Agent::Npm, vec!["lodash".to_string()], None);
        assert_eq!(cmd, "npm");
        assert_eq!(args, vec!["uninstall", "lodash"]);

        let (cmd, args) = parse_nun(Agent::Yarn, vec!["react".to_string()], None);
        assert_eq!(cmd, "yarn");
        assert_eq!(args, vec!["remove", "react"]);

        let (cmd, args) = parse_nun(
            Agent::Pnpm,
            vec!["-g".to_string(), "typescript".to_string()],
            None,
        );
        assert_eq!(cmd, "pnpm");
        assert!(args.contains(&"-g".to_string()));
        assert!(args.contains(&"typescript".to_string()));
    }

    #[test]
    fn test_parse_nu() {
        let (cmd, args) = parse_nu(Agent::Npm, vec!["lodash".to_string()], None);
        assert_eq!(cmd, "npm");
        assert_eq!(args, vec!["update", "lodash"]);

        let (cmd, args) = parse_nu(Agent::Yarn, vec!["react".to_string()], None);
        assert_eq!(cmd, "yarn");
        assert_eq!(args, vec!["upgrade", "react"]);

        let (cmd, args) = parse_nu(Agent::YarnBerry, vec!["react".to_string()], None);
        assert_eq!(cmd, "yarn");
        assert_eq!(args, vec!["up", "react"]);
    }
}
