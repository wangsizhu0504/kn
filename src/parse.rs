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
        let frozen = agent.frozen_args();
        return (
            agent.name().to_string(),
            frozen.into_iter().map(|s| s.to_string()).collect(),
        );
    }

    // Handle global install
    if cmd_args.contains(&"-g".to_string()) {
        if agent.uses_global_prefix() {
            // yarn global add <packages> (remove -g from args)
            let filtered: Vec<String> = cmd_args
                .into_iter()
                .filter(|a| a != "-g" && a != "--global")
                .collect();
            let mut result = vec!["global".to_string(), agent.add_cmd().to_string()];
            result.extend(filtered);
            return (agent.name().to_string(), result);
        } else {
            // npm install -g <packages> / pnpm add -g / bun add -g
            let mut result = vec![agent.add_cmd().to_string()];
            result.extend(cmd_args);
            return (agent.name().to_string(), result);
        }
    }

    // Regular install
    if cmd_args.is_empty() {
        (agent.name().to_string(), vec!["install".to_string()])
    } else {
        let mut result = vec![agent.add_cmd().to_string()];
        result.extend(cmd_args);
        (agent.name().to_string(), result)
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

    // Handle global uninstall
    if cmd_args.contains(&"-g".to_string()) {
        if agent.uses_global_prefix() {
            let filtered: Vec<String> = cmd_args
                .into_iter()
                .filter(|a| a != "-g" && a != "--global")
                .collect();
            let mut result = vec!["global".to_string(), agent.remove_cmd().to_string()];
            result.extend(filtered);
            return (agent.name().to_string(), result);
        } else {
            let mut result = vec![agent.remove_cmd().to_string()];
            result.extend(cmd_args);
            return (agent.name().to_string(), result);
        }
    }

    let mut result = vec![agent.remove_cmd().to_string()];
    result.extend(cmd_args);
    (agent.name().to_string(), result)
}

/// Parse function for execute commands (nlx)
/// Maps to: npx, yarn dlx, pnpm dlx, bunx
pub fn parse_nlx(
    agent: Agent,
    args: Vec<String>,
    _ctx: Option<RunnerContext>,
) -> (String, Vec<String>) {
    if args.is_empty() {
        return (agent.exec_binary().to_string(), args);
    }

    if agent.exec_uses_dlx() {
        let mut dlx_args = vec!["dlx".to_string()];
        dlx_args.extend(args);
        (agent.name().to_string(), dlx_args)
    } else {
        (agent.exec_binary().to_string(), args)
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

        if !agent.supports_interactive_upgrade() {
            eprintln!(
                "{} {} does not support interactive upgrades",
                console::style("✗ Error:").red().bold(),
                agent
            );
            std::process::exit(1);
        }

        let base = agent.interactive_upgrade_args();
        let mut result: Vec<String> = base.into_iter().map(|s| s.to_string()).collect();
        result.extend(cmd_args);
        return (agent.name().to_string(), result);
    }

    // Regular upgrade
    let mut result = vec![agent.upgrade_cmd().to_string()];
    result.extend(cmd_args);
    (agent.name().to_string(), result)
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
