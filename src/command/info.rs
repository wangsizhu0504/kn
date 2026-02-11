use crate::command_utils::detect_agent;
use crate::display::StyledOutput;
use anyhow::Result;
use console::style;
use std::process;

pub fn handle(verbose: bool) -> Result<()> {
    let spinner = StyledOutput::working("Analyzing environment...");
    let agent = detect_agent();
    drop(spinner);

    let manager = agent.name();
    let version = get_package_manager_version(manager).unwrap_or_else(|_| "unknown".to_string());
    let (lock_file, lock_status_text, is_valid_lock) = get_lock_file_info(manager);
    let node_version = get_node_version().unwrap_or_else(|_| "not found".to_string());
    let npm_version = get_npm_version().unwrap_or_else(|_| "not found".to_string());

    // ── Build card lines ──
    let kv_width = 16;

    // Lock status with color
    let lock_display = if lock_file == "None" {
        format!("{}", style("none").yellow())
    } else if is_valid_lock {
        format!("{} ({})", lock_file, style("valid").green())
    } else {
        format!("{} ({})", lock_file, style(&lock_status_text).red())
    };

    let mut lines = vec![
        StyledOutput::kv_line("Lock file", &lock_display, kv_width),
        StyledOutput::kv_line("Node.js", &node_version, kv_width),
        StyledOutput::kv_line("npm", &npm_version, kv_width),
    ];

    if verbose {
        lines.push(String::new());

        if let Ok(dir) = std::env::current_dir() {
            lines.push(StyledOutput::kv_line(
                "CWD",
                &dir.display().to_string(),
                kv_width,
            ));
        }

        if let Ok(output) = process::Command::new("npm")
            .args(["config", "get", "prefix"])
            .output()
        {
            let prefix = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !prefix.is_empty() {
                lines.push(StyledOutput::kv_line("Global prefix", &prefix, kv_width));
            }
        }

        if let Ok(cache) = std::env::var("npm_config_cache") {
            lines.push(StyledOutput::kv_line("Cache", &cache, kv_width));
        }
    }

    // Render as titled section
    let title = format!("{} {}", manager, style(format!("v{}", version)).dim(),);

    println!();
    StyledOutput::titled(&title);
    println!();
    for line in &lines {
        StyledOutput::body(line);
    }

    if !verbose {
        println!();
        StyledOutput::hint("kn info -v for more details");
    }

    println!();
    Ok(())
}

fn get_lock_file_info(manager: &str) -> (String, String, bool) {
    let lock_files = [
        ("pnpm-lock.yaml", "pnpm"),
        ("yarn.lock", "yarn"),
        ("package-lock.json", "npm"),
        ("bun.lockb", "bun"),
        ("bun.lock", "bun"),
    ];

    if let Ok(current_dir) = std::env::current_dir() {
        for (lock_file, lock_manager) in &lock_files {
            if current_dir.join(lock_file).exists() {
                if *lock_manager == manager {
                    return (lock_file.to_string(), "Valid".to_string(), true);
                } else {
                    return (
                        lock_file.to_string(),
                        format!("Expected {}", lock_manager),
                        false,
                    );
                }
            }
        }
    }

    ("None".to_string(), "Missing".to_string(), false)
}

fn get_node_version() -> Result<String> {
    let output = process::Command::new("node").arg("--version").output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn get_npm_version() -> Result<String> {
    let output = process::Command::new("npm").arg("--version").output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn get_package_manager_version(manager: &str) -> Result<String> {
    let output = process::Command::new(manager).arg("--version").output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
