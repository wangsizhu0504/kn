use console::{style, Emoji, Term};
use indicatif::{ProgressBar, ProgressStyle};
use termimad::crossterm::style::Color;
use termimad::{Alignment, MadSkin};

// Emoji constants
static PACKAGE: Emoji<'_, '_> = Emoji("📦  ", "[PKG] ");
static INFO: Emoji<'_, '_> = Emoji("ℹ️  ", "[INFO] ");
static WARNING: Emoji<'_, '_> = Emoji("⚠️  ", "[WARN] ");
static ERROR: Emoji<'_, '_> = Emoji("❌  ", "[ERR] ");

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
        // 使用 Markdown 构建帮助信息
        let mut md = String::new();

        md.push_str("```\n");
        md.push_str(KN_ASCII);
        md.push_str("```\n\n");

        md.push_str("Minimal, blazing fast Node.js package manager and scripts runner\n\n");

        md.push_str("### 📚 Available Commands\n\n");

        md.push_str("```\n");
        md.push_str("  install (i, add)        Install packages (auto-detects package manager)\n");
        md.push_str("  run (r)                 Run npm scripts from package.json\n");
        md.push_str("  uninstall (remove, rm)  Uninstall packages\n");
        md.push_str("  execute (exec, x)       Execute package binaries\n");
        md.push_str("  upgrade (update, up)    Upgrade dependencies\n");
        md.push_str("  upgrade-self            Upgrade kn to the latest version\n");
        md.push_str("  clean-install (ci)      Clean install dependencies (frozen lockfile)\n");
        md.push_str("  list (ls)               Show available package scripts\n");
        md.push_str("  info (env)              Show package manager and environment information\n");
        md.push_str("  watch (w)               Watch files and re-run script on changes\n");
        md.push_str("  clean                   Clean node_modules, cache, etc.\n");
        md.push_str("  size                    Analyze package sizes\n");
        md.push_str("  help                    Show this help message\n");
        md.push_str("```\n");

        md.push_str("\n### 💡 Examples\n\n");
        md.push_str("* `kn i react` - Install react\n");
        md.push_str("* `kn i -D typescript` - Install typescript as dev dependency\n");
        md.push_str("* `kn r dev` - Run dev script\n");
        md.push_str("* `kn ls` - List available scripts\n");
        md.push_str("* `kn up` - Upgrade dependencies\n\n");

        md.push_str("> *For more info:* `kn <command> --help`\n");

        let mut skin = MadSkin::default();
        skin.set_headers_fg(Color::Cyan);
        skin.bold.set_fg(Color::Yellow);
        skin.paragraph.set_fg(Color::White);

        skin.print_text(&md);
    }

    pub fn enhanced_list_scripts(
        package_name: &str,
        package_version: &str,
        scripts: &indexmap::IndexMap<String, String>,
    ) {
        let mut md = String::new();

        md.push_str(&format!(
            "# 📦 {} **v{}**\n\n",
            package_name, package_version
        ));

        if scripts.is_empty() {
            md.push_str("> ℹ️  *No scripts found in this package*\n");
        } else {
            md.push_str("### 📋 Available Scripts\n\n");
            md.push_str("|-|-|\n");
            md.push_str("|**Script**|**Command**|\n");
            md.push_str("|-|-|\n");

            for (name, cmd) in scripts {
                md.push_str(&format!("| **{}** | `{}` |\n", name, cmd));
            }
            md.push_str("\n");
            md.push_str(" > 💡 **Tip:** Run with `kn run <script-name>`\n");
        }

        let mut skin = MadSkin::default();
        skin.set_headers_fg(Color::Cyan);
        skin.bold.set_fg(Color::Yellow);
        skin.table.compound_style.set_fg(Color::DarkGrey);
        skin.paragraph.set_fg(Color::White);
        skin.table_border_chars = termimad::ROUNDED_TABLE_BORDER_CHARS;
        skin.table.align = Alignment::Center;

        skin.print_text(&md);
        println!();
    }

    #[allow(dead_code)]
    pub fn key_value(key: &str, value: &str) {
        println!("  {} {}", key, value);
    }
}
