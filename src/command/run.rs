use crate::command_utils::{run_script_fast, parse_package_json};
use crate::display::StyledOutput;
use crate::command::alias;
use crate::command::stats;
use crate::command::history;

pub fn handle(script_name: Option<String>, args: Vec<String>, _if_present: bool) -> Result<(), Box<dyn std::error::Error>> {
    match script_name {
        Some(script) => {
            // Check if it's an alias first
            let resolved_script = alias::resolve_alias(&script).unwrap_or(script.clone());

            // Try fuzzy match if script not found
            let final_script = if script_exists(&resolved_script) {
                resolved_script
            } else {
                match fuzzy_find_script(&resolved_script)? {
                    Some(found) => {
                        println!("\x1b[90mDid you mean '\x1b[36m{}\x1b[90m'? Running it...\x1b[0m\n", found);
                        found
                    }
                    None => resolved_script
                }
            };

            // Record in history
            history::add_to_history(&format!("run {}", final_script));

            // Measure execution time
            let start = std::time::Instant::now();
            let result = run_script_fast(&final_script, &args);
            let duration = start.elapsed();

            // Record stats
            if result.is_ok() {
                stats::record_execution(&final_script, duration.as_millis() as u64);
                println!("\n\x1b[90m✓ Completed in {:.2}s\x1b[0m", duration.as_secs_f64());
            }

            result?;
        }
        None => {
            // Show available scripts with usage hint
            show_available_scripts()?;
        }
    }
    Ok(())
}

fn script_exists(script_name: &str) -> bool {
    if let Ok(mut current_dir) = std::env::current_dir() {
        loop {
            let package_json_path = current_dir.join("package.json");
            if package_json_path.is_file() {
                if let Ok(package) = parse_package_json(&package_json_path.to_string_lossy()) {
                    if let Some(scripts) = package.scripts {
                        return scripts.contains_key(script_name);
                    }
                }
                break;
            }
            if !current_dir.pop() {
                break;
            }
        }
    }
    false
}

fn fuzzy_find_script(input: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let mut current_dir = std::env::current_dir()?;
    let package_json_path = loop {
        let package_json_path = current_dir.join("package.json");
        if package_json_path.is_file() {
            break package_json_path.to_string_lossy().to_string();
        }
        if !current_dir.pop() {
            return Ok(None);
        }
    };

    let package = parse_package_json(&package_json_path)?;
    let scripts = package.scripts.unwrap_or_default();

    // Calculate Levenshtein distance for fuzzy matching
    let mut matches: Vec<(String, usize)> = scripts
        .keys()
        .map(|name| (name.clone(), levenshtein_distance(input, name)))
        .filter(|(_, dist)| *dist <= 2)
        .collect();

    matches.sort_by_key(|(_, dist)| *dist);

    Ok(matches.first().map(|(name, _)| name.clone()))
}

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(
                    matrix[i][j + 1] + 1,
                    matrix[i + 1][j] + 1,
                ),
                matrix[i][j] + cost,
            );
        }
    }

    matrix[len1][len2]
}

fn show_available_scripts() -> Result<(), Box<dyn std::error::Error>> {
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

    let package = parse_package_json(&package_json_path)?;
    let scripts = package.scripts.unwrap_or_default();

    if scripts.is_empty() {
        StyledOutput::info("No scripts found in this package");
        return Ok(());
    }

    println!("");
    println!("\x1b[33m⚠\x1b[0m  Missing script name");
    println!("");
    println!("\x1b[1mAvailable scripts:\x1b[0m");
    println!("");

    // Display scripts in a clean list
    for (name, cmd) in scripts.iter() {
        let display_cmd = if cmd.len() > 60 {
            format!("{}...", &cmd[..57])
        } else {
            cmd.to_string()
        };
        println!("  \x1b[36m{:<15}\x1b[0m \x1b[90m{}\x1b[0m", name, display_cmd);
    }

    println!("");
    println!("\x1b[90mUsage:\x1b[0m kn run \x1b[36m<script>\x1b[0m");
    println!("");

    Ok(())
}
