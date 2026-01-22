use crate::command_utils::run_script_fast;
use crate::display::StyledOutput;
use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn handle(script_name: String, patterns: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let watch_patterns = if patterns.is_empty() {
        vec!["src/**/*".to_string(), "*.js".to_string(), "*.ts".to_string()]
    } else {
        patterns
    };

    println!("");
    println!("\x1b[36m👀 Watch Mode\x1b[0m");
    println!("");
    println!("\x1b[90mWatching for changes in:\x1b[0m");
    for pattern in &watch_patterns {
        println!("  \x1b[90m•\x1b[0m {}", pattern);
    }
    println!("");
    println!("\x1b[32m▶\x1b[0m Running script: \x1b[36m{}\x1b[0m", script_name);
    println!("{}", "\x1b[90m─\x1b[0m".repeat(70));
    println!("");

    // Initial run
    if let Err(e) = run_script_fast(&script_name, &[]) {
        StyledOutput::error(&format!("Script failed: {}", e));
    }

    // Watch for changes
    let mut file_times: HashMap<PathBuf, SystemTime> = HashMap::new();
    let current_dir = std::env::current_dir()?;

    println!("");
    println!("\x1b[33m👁  Watching for changes...\x1b[0m \x1b[90m(Press Ctrl+C to stop)\x1b[0m");
    println!("");

    loop {
        std::thread::sleep(Duration::from_millis(500));

        let mut changed = false;

        // Check for file changes
        if let Ok(entries) = std::fs::read_dir(&current_dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        let path = entry.path();

                        // Check if file matches patterns
                        if should_watch(&path, &watch_patterns) {
                            if let Ok(modified) = metadata.modified() {
                                if let Some(&last_modified) = file_times.get(&path) {
                                    if modified > last_modified {
                                        changed = true;
                                        file_times.insert(path.clone(), modified);
                                        println!("\x1b[33m📝 Changed:\x1b[0m {}", path.display());
                                    }
                                } else {
                                    file_times.insert(path, modified);
                                }
                            }
                        }
                    }
                }
            }
        }

        if changed {
            println!("");
            println!("\x1b[32m▶\x1b[0m Re-running script: \x1b[36m{}\x1b[0m", script_name);
            println!("{}", "\x1b[90m─\x1b[0m".repeat(70));
            println!("");

            if let Err(e) = run_script_fast(&script_name, &[]) {
                StyledOutput::error(&format!("Script failed: {}", e));
            }

            println!("");
            println!("\x1b[33m👁  Watching for changes...\x1b[0m");
            println!("");
        }
    }
}

fn should_watch(path: &Path, patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy();

    for pattern in patterns {
        if pattern.contains("**") {
            // Simple glob matching for **
            let parts: Vec<&str> = pattern.split("**").collect();
            if parts.len() == 2 {
                let prefix = parts[0];
                let suffix = parts[1].trim_start_matches('/');
                if path_str.starts_with(prefix) && path_str.ends_with(suffix) {
                    return true;
                }
            }
        } else if pattern.starts_with("*.") {
            // Extension matching
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
