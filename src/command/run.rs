use crate::command::stats;
use crate::command_utils::{parse_package_json, run_script_fast};
use crate::display::StyledOutput;
use inquire::Select;

pub fn handle(
    script_name: Option<String>,
    args: Vec<String>,
    _if_present: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match script_name {
        Some(script) => {
            // Try fuzzy match if script not found
            let final_script = if script_exists(&script) {
                script
            } else {
                match fuzzy_find_script(&script)? {
                    Some(found) => {
                        println!(
                            "\x1b[90mDid you mean '\x1b[36m{}\x1b[90m'? Running it...\x1b[0m\n",
                            found
                        );
                        found
                    }
                    None => script,
                }
            };

            // Measure execution time
            let start = std::time::Instant::now();
            let result = run_script_fast(&final_script, &args);
            let duration = start.elapsed();

            // Record stats
            if result.is_ok() {
                stats::record_execution(&final_script, duration.as_millis() as u64);
                println!(
                    "\n\x1b[90m✓ Completed in {:.2}s\x1b[0m",
                    duration.as_secs_f64()
                );
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
                std::cmp::min(matrix[i][j + 1] + 1, matrix[i + 1][j] + 1),
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

    // Get execution stats for sorting
    let all_stats = stats::get_all_stats();

    // Create script items with stats
    let mut script_items: Vec<ScriptItem> = scripts
        .iter()
        .map(|(name, cmd)| {
            let stat = all_stats.iter().find(|s| &s.script == name);
            ScriptItem {
                name: name.clone(),
                command: cmd.clone(),
                runs: stat.map(|s| s.count).unwrap_or(0),
                last_run: stat.and_then(|s| s.last_run),
                avg_time: stat.map(|s| s.average_time).unwrap_or(0),
            }
        })
        .collect();

    // Sort by run count (most used first)
    script_items.sort_by(|a, b| b.runs.cmp(&a.runs));

    StyledOutput::header("Available Scripts");
    println!();

    // Interactive selection
    let options: Vec<String> = script_items
        .iter()
        .map(|item| format_script_option(item))
        .collect();

    match Select::new("Select a script to run:", options).prompt() {
        Ok(selection) => {
            // Extract script name from selection
            if let Some(item) = script_items
                .iter()
                .find(|item| selection.starts_with(&format!("{} ", item.name)))
            {
                // Measure execution time
                let start = std::time::Instant::now();
                println!();
                let result = run_script_fast(&item.name, &[]);
                let duration = start.elapsed();

                // Record stats
                if result.is_ok() {
                    stats::record_execution(&item.name, duration.as_millis() as u64);
                    println!(
                        "\n\x1b[90m✓ Completed in {:.2}s\x1b[0m",
                        duration.as_secs_f64()
                    );
                }

                result?;
            }
        }
        Err(_) => {
            println!("\nCancelled");
        }
    }

    Ok(())
}

#[derive(Clone)]
struct ScriptItem {
    name: String,
    command: String,
    runs: u32,
    last_run: Option<u64>,
    avg_time: u64,
}

fn format_script_option(item: &ScriptItem) -> String {
    let mut parts = vec![item.name.clone()];

    // Show command preview
    let cmd_preview = if item.command.len() > 50 {
        format!("{}...", &item.command[..47])
    } else {
        item.command.clone()
    };
    parts.push(format!("\x1b[90m{}\x1b[0m", cmd_preview));

    // Show stats if available
    if item.runs > 0 {
        let mut stats_parts = Vec::new();

        // Run count
        if item.runs > 0 {
            stats_parts.push(format!("⚡ {}", item.runs));
        }

        // Average time
        if item.avg_time > 0 {
            let time_str = if item.avg_time < 1000 {
                format!("{}ms", item.avg_time)
            } else {
                format!("{:.1}s", item.avg_time as f64 / 1000.0)
            };
            stats_parts.push(format!("⏱️ {}", time_str));
        }

        // Last run
        if let Some(last_run) = item.last_run {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let diff = now - last_run;
            let time_ago = if diff < 60 {
                "just now".to_string()
            } else if diff < 3600 {
                format!("{}m ago", diff / 60)
            } else if diff < 86400 {
                format!("{}h ago", diff / 3600)
            } else {
                format!("{}d ago", diff / 86400)
            };
            stats_parts.push(format!("🕒 {}", time_ago));
        }

        if !stats_parts.is_empty() {
            parts.push(format!("\x1b[36m[{}]\x1b[0m", stats_parts.join(" ")));
        }
    }

    parts.join(" ")
}
