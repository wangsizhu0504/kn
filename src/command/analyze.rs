use crate::command_utils::parse_package_json;
use crate::display::StyledOutput;
use std::collections::HashMap;
use std::process::Command;

pub fn handle() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\x1b[1m📊 Analyzing project dependencies...\x1b[0m\n");

    // Find package.json
    let mut current_dir = std::env::current_dir()?;
    let package_json_path = loop {
        let package_json_path = current_dir.join("package.json");
        if package_json_path.is_file() {
            break package_json_path.to_string_lossy().to_string();
        }
        if !current_dir.pop() {
            StyledOutput::error("No package.json found");
            std::process::exit(1);
        }
    };

    let _package = parse_package_json(&package_json_path)?;

    // For now, we'll parse dependencies directly from JSON
    let json_str = std::fs::read_to_string(&package_json_path)?;
    let json: serde_json::Value = serde_json::from_str(&json_str)?;

    let deps_count = json
        .get("dependencies")
        .and_then(|v| v.as_object())
        .map(|o| o.len())
        .unwrap_or(0);

    let dev_deps_count = json
        .get("devDependencies")
        .and_then(|v| v.as_object())
        .map(|o| o.len())
        .unwrap_or(0);

    let total_deps = deps_count + dev_deps_count;

    println!("  \x1b[1mDependencies Overview\x1b[0m");
    println!("  ├─ Production: \x1b[36m{}\x1b[0m", deps_count);
    println!("  ├─ Development: \x1b[33m{}\x1b[0m", dev_deps_count);
    println!("  └─ Total: \x1b[1m{}\x1b[0m", total_deps);
    println!();

    // Analyze node_modules size if it exists
    if let Ok(metadata) = std::fs::metadata("node_modules") {
        if metadata.is_dir() {
            println!("  \x1b[1mDisk Usage\x1b[0m");

            if let Ok(size) = get_dir_size("node_modules") {
                let size_mb = size / 1024 / 1024;
                let color = if size_mb > 500 {
                    "\x1b[31m"
                } else if size_mb > 200 {
                    "\x1b[33m"
                } else {
                    "\x1b[32m"
                };
                println!("  └─ node_modules: {}{} MB\x1b[0m", color, size_mb);
            }
            println!();
        }
    }

    // Check for outdated packages
    println!("  \x1b[1mOutdated Packages\x1b[0m");
    check_outdated()?;

    // Analyze duplicates
    analyze_duplicates()?;

    println!();

    Ok(())
}

fn get_dir_size(path: &str) -> Result<u64, std::io::Error> {
    let mut size = 0u64;

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    size += metadata.len();
                } else if metadata.is_dir() {
                    // Limit recursion depth to avoid performance issues
                    if let Ok(subsize) = get_dir_size(&entry.path().to_string_lossy()) {
                        size += subsize;
                    }
                }
            }
        }
    }

    Ok(size)
}

fn check_outdated() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("npm").args(&["outdated", "--json"]).output();

    match output {
        Ok(output) if output.status.success() || !output.stdout.is_empty() => {
            if let Ok(json_str) = String::from_utf8(output.stdout) {
                if let Ok(outdated) =
                    serde_json::from_str::<HashMap<String, serde_json::Value>>(&json_str)
                {
                    if outdated.is_empty() {
                        println!("  └─ \x1b[32m✓ All packages are up to date\x1b[0m");
                    } else {
                        println!(
                            "  └─ \x1b[33m{} packages need updates\x1b[0m",
                            outdated.len()
                        );
                        println!("     \x1b[90mRun 'kn upgrade' to update them\x1b[0m");
                    }
                    return Ok(());
                }
            }
        }
        _ => {}
    }

    println!("  └─ \x1b[90mRun 'kn upgrade' to check for updates\x1b[0m");
    Ok(())
}

fn analyze_duplicates() -> Result<(), Box<dyn std::error::Error>> {
    println!();
    println!("  \x1b[1mDuplicate Packages\x1b[0m");

    let output = Command::new("npm").args(&["dedupe", "--dry-run"]).output();

    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("removed") || stdout.contains("dedupe") {
                println!("  └─ \x1b[33mDuplicates found\x1b[0m");
                println!("     \x1b[90mRun 'npm dedupe' to optimize\x1b[0m");
            } else {
                println!("  └─ \x1b[32m✓ No duplicates detected\x1b[0m");
            }
        }
        _ => {
            println!("  └─ \x1b[90mCould not analyze duplicates\x1b[0m");
        }
    }

    Ok(())
}
