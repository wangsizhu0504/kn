use anyhow::Result;
use console::style;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use crate::command_utils::run_script_fast;
use crate::display::StyledOutput;

pub fn handle(script_name: String, patterns: Vec<String>) -> Result<()> {
    let watch_patterns = if patterns.is_empty() {
        vec![
            "src/**/*".to_string(),
            "*.js".to_string(),
            "*.ts".to_string(),
        ]
    } else {
        patterns
    };

    // ── Watch header card ──
    let title = format!("Watch  {}", style(&script_name).cyan(),);

    println!();
    StyledOutput::titled(&title);
    for (i, pattern) in watch_patterns.iter().enumerate() {
        let is_last = i == watch_patterns.len() - 1;
        StyledOutput::tree_item(&format!("{}", style(pattern).dim()), is_last);
    }
    println!();

    // Initial run
    StyledOutput::separator();
    println!();

    if let Err(e) = run_script_fast(&script_name, &[]) {
        StyledOutput::error(&format!("Script failed: {}", e));
    }

    // Watch for changes
    let mut file_times: HashMap<PathBuf, SystemTime> = HashMap::new();
    let current_dir = std::env::current_dir()?;

    scan_directory_recursive(&current_dir, &watch_patterns, &mut file_times);

    println!();
    println!(
        "  {} {}  {}",
        style("●").cyan().bold(),
        style("Watching for changes...").cyan(),
        style("(Ctrl+C to stop)").dim(),
    );
    println!();

    loop {
        std::thread::sleep(Duration::from_millis(500));

        let mut changed_files = Vec::new();
        check_changes_recursive(
            &current_dir,
            &watch_patterns,
            &mut file_times,
            &mut changed_files,
        );

        if !changed_files.is_empty() {
            // Show changed files
            for (i, path) in changed_files.iter().enumerate() {
                let relative = path.strip_prefix(&current_dir).unwrap_or(path);
                let is_last = i == changed_files.len() - 1;
                let connector = if is_last { "└" } else { "├" };
                println!(
                    "  {} {} {}",
                    style(connector).dim(),
                    style("~").yellow(),
                    style(relative.display()).dim(),
                );
            }

            println!();
            StyledOutput::separator();
            println!();

            if let Err(e) = run_script_fast(&script_name, &[]) {
                StyledOutput::error(&format!("Script failed: {}", e));
            }

            println!();
            println!(
                "  {} {}",
                style("●").cyan().bold(),
                style("Watching for changes...").cyan(),
            );
            println!();
        }
    }
}

fn scan_directory_recursive(
    dir: &Path,
    patterns: &[String],
    file_times: &mut HashMap<PathBuf, SystemTime>,
) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if name.starts_with('.') || name == "node_modules" || name == "target" {
            continue;
        }

        if path.is_dir() {
            scan_directory_recursive(&path, patterns, file_times);
        } else if path.is_file() && should_watch(&path, patterns) {
            if let Ok(modified) = entry.metadata().and_then(|m| m.modified()) {
                file_times.insert(path, modified);
            }
        }
    }
}

fn check_changes_recursive(
    dir: &Path,
    patterns: &[String],
    file_times: &mut HashMap<PathBuf, SystemTime>,
    changed: &mut Vec<PathBuf>,
) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if name.starts_with('.') || name == "node_modules" || name == "target" {
            continue;
        }

        if path.is_dir() {
            check_changes_recursive(&path, patterns, file_times, changed);
        } else if path.is_file() && should_watch(&path, patterns) {
            if let Ok(modified) = entry.metadata().and_then(|m| m.modified()) {
                if let Some(&last_modified) = file_times.get(&path) {
                    if modified > last_modified {
                        changed.push(path.clone());
                        file_times.insert(path, modified);
                    }
                } else {
                    file_times.insert(path.clone(), modified);
                    changed.push(path);
                }
            }
        }
    }
}

fn should_watch(path: &Path, patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();

    for pattern in patterns {
        if pattern.contains("**") {
            let parts: Vec<&str> = pattern.split("**").collect();
            if parts.len() == 2 {
                let prefix = parts[0].trim_end_matches('/');
                let suffix = parts[1].trim_start_matches('/');
                if (prefix.is_empty() || path_str.contains(prefix))
                    && (suffix == "*" || suffix.is_empty() || path_str.ends_with(suffix))
                {
                    return true;
                }
            }
        } else if pattern.starts_with("*.") {
            let ext = pattern.trim_start_matches("*.");
            if path_str.ends_with(&format!(".{}", ext)) {
                return true;
            }
        } else if path_str.contains(pattern) {
            return true;
        }
    }

    false
}
