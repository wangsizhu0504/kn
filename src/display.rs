use console::{style, Emoji, Term};
use indicatif::{ProgressBar, ProgressStyle};
use comfy_table::{Table, Cell, Color as TableColor, Attribute, ContentArrangement, presets::UTF8_FULL};

// Emoji constants
static PACKAGE: Emoji<'_, '_> = Emoji("📦  ", "[PKG] ");
static INFO: Emoji<'_, '_> = Emoji("ℹ️  ", "[INFO] ");
static SUCCESS: Emoji<'_, '_> = Emoji("✅  ", "[OK] ");
static WARNING: Emoji<'_, '_> = Emoji("⚠️  ", "[WARN] ");
static ERROR: Emoji<'_, '_> = Emoji("❌  ", "[ERR] ");
static CHART: Emoji<'_, '_> = Emoji("📊  ", "[STAT] ");

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

pub struct Spinner {
    pb: Option<ProgressBar>,
}

impl Drop for Spinner {
    fn drop(&mut self) {
        if let Some(pb) = &self.pb {
            pb.finish_and_clear();
        }
    }
}

impl StyledOutput {
    pub fn header(text: &str) {
        let term = Term::stdout();
        let width = term.size().1 as usize;
        println!("\n{}", style(text).bold().cyan());
        println!("{}", style("─".repeat(width.min(80))).dim());
    }

    pub fn success(text: &str) {
        println!("{}{}", SUCCESS, style(text).green());
    }

    pub fn error(text: &str) {
        eprintln!("{}{}", ERROR, style(text).red().bold());
    }

    pub fn warning(text: &str) {
        println!("{}{}", WARNING, style(text).yellow());
    }

    pub fn info(text: &str) {
        println!("{}{}", INFO, style(text).cyan());
    }

    pub fn package_info(name: &str, version: &str, manager: &str) {
        println!(
            "{}{} {} {}",
            PACKAGE,
            style(name).bold(),
            style(version).dim(),
            style(format!("({})", manager)).dim()
        );
    }

    pub fn working(text: &str) -> Spinner {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        pb.set_message(text.to_string());
        pb.enable_steady_tick(std::time::Duration::from_millis(80));

        Spinner { pb: Some(pb) }
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
            ("stats", "", "Show script performance statistics", "\x1b[93m"),
            ("parallel", "p", "Run multiple scripts in parallel", "\x1b[95m"),
            ("clean", "", "Clean node_modules, cache, etc.", "\x1b[31m"),
            ("analyze", "", "Analyze project dependencies", "\x1b[96m"),
            ("doctor", "", "Check project health and configuration", "\x1b[92m"),
            ("size", "", "Analyze package sizes", "\x1b[94m"),
            ("completion", "", "Generate shell completion scripts", "\x1b[90m"),
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
        println!();
        println!("  {}", style(title).bold().underlined().cyan());
        println!();
    }

    pub fn check_item(passed: bool, message: &str) {
        if passed {
            println!("  {} {}", style("●").green().bold(), style(message).dim());
        } else {
            println!("  {} {}", style("●").red().bold(), style(message).dim());
        }
    }

    pub fn detail_item(icon: &str, message: &str) {
        println!("    {} {}", style(icon).dim(), style(message).dim());
    }

    pub fn summary_box(title: &str, good: usize, warnings: usize, errors: usize) {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic);

        println!("\n{}{}", CHART, style(title).bold().cyan());

        if good > 0 {
            table.add_row(vec![
                Cell::new("✓ Passed").fg(TableColor::Green).add_attribute(Attribute::Bold),
                Cell::new(good.to_string()).fg(TableColor::Green),
            ]);
        }

        if warnings > 0 {
            table.add_row(vec![
                Cell::new("⚠ Warnings").fg(TableColor::Yellow).add_attribute(Attribute::Bold),
                Cell::new(warnings.to_string()).fg(TableColor::Yellow),
            ]);
        }

        if errors > 0 {
            table.add_row(vec![
                Cell::new("✗ Errors").fg(TableColor::Red).add_attribute(Attribute::Bold),
                Cell::new(errors.to_string()).fg(TableColor::Red),
            ]);
        }

        if good == 0 && warnings == 0 && errors == 0 {
            table.add_row(vec![
                Cell::new("No checks performed").fg(TableColor::Grey),
                Cell::new("-"),
            ]);
        }

        println!("{}", table);
    }

    pub fn key_value(key: &str, value: &str) {
        println!("  {} {}", key, value);
    }

    pub fn command_example(command: &str, description: &str) {
        println!("  - {} {}", command, description);
    }

}
