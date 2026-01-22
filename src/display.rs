// Minimal, dependency-free UI outputs for compilation stability.
static PACKAGE: &str = "📦";
static INFO: &str = "ℹ️";
static SUCCESS: &str = "✅";
static WARNING: &str = "⚠️";
static ERROR: &str = "❌";

// ASCII art for KN
const KN_ASCII: &str = r#"
 ██╗  ██╗███╗   ██╗
 ██║ ██╔╝████╗  ██║
 █████╔╝ ██╔██╗ ██║
 ██╔═██╗ ██║╚██╗██║
 ██║  ██╗██║ ╚████║
 ╚═╝  ╚═╝╚═╝  ╚═══╝
"#;

pub struct StyledOutput;

pub struct Spinner;
impl Drop for Spinner {
    fn drop(&mut self) {
        // no-op
    }
}

impl StyledOutput {
    pub fn header(text: &str) {
        println!("");
        println!("{}", text);
        println!("{}", "-".repeat(text.len() + 4));
        println!("{}", "");
        println!("{}", "");
    }

    pub fn success(text: &str) {
        println!("{} {}", SUCCESS, text);
    }

    pub fn error(text: &str) {
        eprintln!("{} {}", ERROR, text);
    }

    pub fn warning(text: &str) {
        println!("{} {}", WARNING, text);
    }

    pub fn info(text: &str) {
        println!("{} {}", INFO, text);
    }

    pub fn package_info(name: &str, version: &str, manager: &str) {
        println!("{} {} {} ({})", PACKAGE, name, version, manager);
    }

    pub fn working(_text: &str) -> Spinner {
        Spinner
    }

    pub fn opencode_header() {
        Self::kn_help_header();
    }

    pub fn kn_help_header() {
        println!("{}", KN_ASCII);
        println!("");
        println!("Minimal, blazing fast Node.js package manager and scripts runner");
        println!("");
        println!("{}  {}", "📚", "Available Commands:");
        println!("");
        Self::print_command_list();
        println!("");
        println!("{}  {}", "💡", "Examples:");
        println!("  kn i react           # Install react");
        println!("  kn i -D typescript   # Install typescript as dev dependency");
        println!("  kn r dev             # Run dev script");
        println!("  kn ls                # List available scripts");
        println!("  kn up                # Upgrade dependencies");
        println!("");
        println!("For more info: kn <command> --help");
        println!("");
    }

    fn print_command_list() {
        let commands = vec![
            ("install", "i, add", "Install packages (auto-detects package manager)", "\x1b[32m"),
            ("run", "r", "Run npm scripts from package.json", "\x1b[36m"),
            ("uninstall", "remove, rm", "Uninstall packages", "\x1b[31m"),
            ("execute", "exec, x", "Execute package binaries", "\x1b[33m"),
            ("upgrade", "update, up", "Upgrade dependencies", "\x1b[35m"),
            ("clean-install", "ci", "Clean install dependencies (frozen lockfile)", "\x1b[34m"),
            ("agent", "npm, yarn, pnpm, bun", "Run package manager directly", "\x1b[95m"),
            ("list", "ls", "Show available package scripts", "\x1b[96m"),
            ("info", "env", "Show package manager and environment information", "\x1b[93m"),
            ("watch", "w", "Watch files and re-run script on changes", "\x1b[35m"),
            ("history", "hist", "Show command history", "\x1b[94m"),
            ("!!", "", "Re-run last command", "\x1b[90m"),
            ("!N", "", "Re-run command N from history", "\x1b[90m"),
            ("alias", "", "Manage script aliases", "\x1b[92m"),
            ("stats", "", "Show script performance statistics", "\x1b[93m"),
            ("parallel", "p", "Run multiple scripts in parallel", "\x1b[95m"),
            ("clean", "", "Clean node_modules, cache, etc.", "\x1b[31m"),
            ("analyze", "", "Analyze project dependencies", "\x1b[96m"),
            ("help", "", "Show this help message", "\x1b[37m"),
        ];

        // 计算最大宽度以对齐
        let max_main_cmd_width = commands.iter().map(|(main, _, _, _)| main.len()).max().unwrap_or(0);
        let max_alias_width = commands.iter().map(|(_, alias, _, _)| alias.len()).max().unwrap_or(0);

        for (main_cmd, aliases, desc, color) in commands {
            if aliases.is_empty() {
                println!("  {}{:<width$}\x1b[0m  {}",
                    color,
                    main_cmd,
                    desc,
                    width = max_main_cmd_width + max_alias_width + 4
                );
            } else {
                println!("  {}{:<main_width$}\x1b[0m \x1b[90m{:<alias_width$}\x1b[0m  {}",
                    color,
                    main_cmd,
                    aliases,
                    desc,
                    main_width = max_main_cmd_width,
                    alias_width = max_alias_width + 2
                );
            }
        }
    }


    pub fn enhanced_list_scripts(
        package_name: &str,
        package_version: &str,
        scripts: &indexmap::IndexMap<String, String>,
    ) {

        println!("╭─────────────────────────────────────────────────────────────────────╮");
        println!("│  📦  \x1b[1m{}\x1b[0m \x1b[90mv{}\x1b[0m", package_name, package_version);
        println!("├─────────────────────────────────────────────────────────────────────┤");

        if scripts.is_empty() {
            println!("│  ℹ️   No scripts found in this package");
        } else {
            println!("│  📋  \x1b[1mAvailable Scripts\x1b[0m");
            println!("├─────────────────────────────────────────────────────────────────────┤");

            let list: Vec<(String, String)> = scripts.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();

            let max_name_width = list.iter()
                .map(|(n, _)| n.len())
                .max()
                .unwrap_or(0)
                .max(12);

            for (i, (name, cmd)) in list.iter().enumerate() {
                let is_last = i == list.len() - 1;
                let prefix = if is_last { "└─" } else { "├─" };

                println!("│  {} \x1b[36m{:<width$}\x1b[0m  \x1b[90m{}\x1b[0m",
                    prefix,
                    name,
                    cmd,
                    width = max_name_width
                );
            }
        }

        println!("╰─────────────────────────────────────────────────────────────────────╯");
        println!("");
        println!("  \x1b[35m💡 Tip:\x1b[0m Run scripts with: \x1b[36mkn run <script-name>\x1b[0m");
        println!("");
    }

    pub fn section_title(title: &str) {
        println!("\n");
        println!("{}", title);
        println!("{}", "-".repeat(title.len()));
        println!("");
    }

    pub fn key_value(key: &str, value: &str) {
        println!("  {} {}", key, value);
    }

    pub fn command_example(command: &str, description: &str) {
        println!("  - {} {}", command, description);
    }

}
