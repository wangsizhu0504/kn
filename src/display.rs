use console::{style, Term};
use indicatif::{ProgressBar, ProgressStyle};

// ── ASCII Art Logo (lightweight line characters) ──
const LOGO: &[&str] = &[
    "╦╔═ ╔╗╔",
    "╠╩╗ ║║║",
    "╩ ╩ ╝╚╝",
];

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

fn term_width() -> usize {
    let term = Term::stdout();
    (term.size().1 as usize).min(60)
}

impl StyledOutput {
    // ════════════════════════════════════════════════
    //  Status messages
    // ════════════════════════════════════════════════

    pub fn error(text: &str) {
        eprintln!(
            "  {} {}",
            style("✖").red().bold(),
            style(text).red(),
        );
    }

    #[allow(dead_code)]
    pub fn warning(text: &str) {
        eprintln!(
            "  {} {}",
            style("⚠").yellow().bold(),
            style(text).yellow(),
        );
    }

    pub fn info(text: &str) {
        println!("  {} {}", style("ℹ").cyan(), text);
    }

    pub fn success(text: &str) {
        println!("  {} {}", style("✔").green(), text);
    }

    pub fn dim(text: &str) {
        println!("  {}", style(text).dim());
    }

    pub fn hint(text: &str) {
        println!(
            "  {} {}",
            style("›").dim(),
            style(text).dim(),
        );
    }

    // ════════════════════════════════════════════════
    //  Titled section (▸ prefix header + indented body)
    //
    //  ▸ Title text
    //
    //    Body line 1
    //    Body line 2
    //
    // ════════════════════════════════════════════════

    /// Section header with ▸ arrow prefix
    pub fn titled(title: &str) {
        println!(
            "  {} {}",
            style("▸").cyan().bold(),
            style(title).bold(),
        );
    }

    /// Indented body line (4 spaces)
    pub fn body(text: &str) {
        if text.is_empty() {
            println!();
        } else {
            println!("    {}", text);
        }
    }

    // ════════════════════════════════════════════════
    //  Section / Separator
    // ════════════════════════════════════════════════

    pub fn header(text: &str) {
        println!();
        println!("  {}", style(text).bold());
    }

    #[allow(dead_code)]
    pub fn section(text: &str) {
        println!();
        println!("  {}", style(text).bold().dim());
    }

    pub fn separator() {
        println!("  {}", style("─".repeat(term_width())).dim());
    }

    pub fn separator_with_label(label: &str) {
        let width = term_width();
        let label_str = format!(" {} ", label);
        let remaining = width.saturating_sub(label_str.len() + 2);
        println!(
            "  {}{}{}",
            style("─".repeat(2)).dim(),
            style(&label_str).dim(),
            style("─".repeat(remaining)).dim(),
        );
    }

    // ════════════════════════════════════════════════
    //  Key-value pairs
    // ════════════════════════════════════════════════

    pub fn kv(key: &str, value: &str) {
        Self::kv_width(key, value, 18);
    }

    pub fn kv_width(key: &str, value: &str, width: usize) {
        let label = format!("{}:", key);
        println!(
            "  {:<width$} {}",
            style(&label).bold().dim(),
            value,
            width = width + 1,
        );
    }

    /// Format as a string for embedding in body lines
    pub fn kv_line(key: &str, value: &str, width: usize) -> String {
        let label = format!("{}:", key);
        format!(
            "{:<width$} {}",
            style(&label).dim(),
            value,
            width = width + 1,
        )
    }

    // ════════════════════════════════════════════════
    //  Tree rendering
    // ════════════════════════════════════════════════

    pub fn tree_item(text: &str, is_last: bool) {
        let connector = if is_last { "└" } else { "├" };
        println!("  {} {}", style(connector).dim(), text);
    }

    // ════════════════════════════════════════════════
    //  Spinner
    // ════════════════════════════════════════════════

    pub fn working(text: &str) -> Spinner {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("  {spinner:.cyan} {msg}")
                .unwrap(),
        );
        pb.set_message(text.to_string());
        pb.enable_steady_tick(std::time::Duration::from_millis(80));
        Spinner { pb: Some(pb) }
    }

    // ════════════════════════════════════════════════
    //  Completion
    // ════════════════════════════════════════════════

    pub fn completion(duration_secs: f64) {
        println!(
            "\n  {} {}",
            style("✔").green(),
            style(format!("Done in {:.2}s", duration_secs)).dim(),
        );
    }

    // ════════════════════════════════════════════════
    //  Brand / Version (ASCII Art Logo)
    // ════════════════════════════════════════════════

    pub fn brand() {
        let version = env!("CARGO_PKG_VERSION");
        println!();
        for (i, line) in LOGO.iter().enumerate() {
            if i == 1 {
                // version on the middle line
                println!(
                    "  {}   {}",
                    style(line).cyan().bold(),
                    style(format!("v{}", version)).dim(),
                );
            } else {
                println!("  {}", style(line).cyan().bold());
            }
        }
        println!();
    }

    // ════════════════════════════════════════════════
    //  Help page
    // ════════════════════════════════════════════════

    pub fn opencode_header() {
        Self::print_help();
    }

    pub fn print_help() {
        let version = env!("CARGO_PKG_VERSION");

        println!();
        for (i, line) in LOGO.iter().enumerate() {
            if i == 1 {
                println!(
                    "  {}   {}",
                    style(line).cyan().bold(),
                    style(format!("v{}", version)).dim(),
                );
            } else {
                println!("  {}", style(line).cyan().bold());
            }
        }
        println!(
            "  {}",
            style("Minimal, blazing fast Node.js package manager runner").dim(),
        );
        println!();

        // ── Package Management ──
        println!("  {}", style("Package Management").bold());
        println!();
        Self::help_cmd("install", "i, add", "Install packages");
        Self::help_cmd("uninstall", "rm, remove", "Remove packages");
        Self::help_cmd("upgrade", "up, update", "Upgrade dependencies");
        Self::help_cmd("clean-install", "ci", "Clean install (frozen lockfile)");
        println!();

        // ── Scripts ──
        println!("  {}", style("Scripts").bold());
        println!();
        Self::help_cmd("run", "r", "Run scripts from package.json");
        Self::help_cmd("list", "ls", "List available scripts");
        Self::help_cmd("watch", "w", "Watch files and re-run script");
        Self::help_cmd("execute", "x, exec", "Execute package binaries");
        println!();

        // ── Project ──
        println!("  {}", style("Project").bold());
        println!();
        Self::help_cmd("info", "env", "Show environment information");
        Self::help_cmd("view", "", "View package info from registry");
        Self::help_cmd("clean", "", "Clean node_modules, cache, etc.");
        println!();

        // ── Other ──
        println!("  {}", style("Other").bold());
        println!();
        Self::help_cmd("upgrade-self", "", "Upgrade kn to latest version");
        Self::help_cmd("help", "-h", "Show this help");
        Self::help_cmd("--version", "-v", "Show version number");
        println!();

        // ── Examples ──
        Self::separator_with_label("Examples");
        println!();
        Self::help_example("kn i react", "Install a package");
        Self::help_example("kn i -D typescript", "Install as devDependency");
        Self::help_example("kn r dev", "Run dev script");
        Self::help_example("kn ls", "List all scripts");
        Self::help_example("kn up -i", "Interactive upgrade");
        Self::help_example("kn view react", "View package details");
        println!();

        Self::dim("Run kn <command> --help for more information.");
        println!();
    }

    fn help_cmd(name: &str, aliases: &str, desc: &str) {
        if aliases.is_empty() {
            println!(
                "    {:<26} {}",
                style(name).cyan(),
                style(desc).dim(),
            );
        } else {
            let combined = format!("{}, {}", name, aliases);
            println!(
                "    {:<26} {}",
                style(&combined).cyan(),
                style(desc).dim(),
            );
        }
    }

    fn help_example(cmd: &str, desc: &str) {
        println!(
            "    {}  {}",
            style(format!("{:<24}", cmd)).green(),
            style(desc).dim(),
        );
    }

    // ════════════════════════════════════════════════
    //  Script list
    // ════════════════════════════════════════════════

    pub fn list_scripts(
        package_name: &str,
        package_version: &str,
        scripts: &indexmap::IndexMap<String, String>,
    ) {
        let max_key = scripts.keys().map(|k| k.len()).max().unwrap_or(10).min(20);

        println!();
        Self::titled(&format!(
            "{} {}",
            package_name,
            style(format!("v{}", package_version)).dim(),
        ));
        println!();

        if scripts.is_empty() {
            Self::body(&format!("{}", style("No scripts defined").dim()));
        } else {
            for (name, cmd) in scripts {
                let cmd_display = if cmd.len() > 50 {
                    format!("{}…", &cmd[..49])
                } else {
                    cmd.clone()
                };
                Self::body(&format!(
                    "{:<width$}  {}",
                    style(name).cyan(),
                    style(&cmd_display).dim(),
                    width = max_key,
                ));
            }
        }

        println!();
        Self::hint("kn r <name>");
        println!();
    }
}
