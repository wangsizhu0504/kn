use anyhow::Result;
use console::style;
use inquire::Select;

use crate::command_utils::run_script_fast;
use crate::display::StyledOutput;
use crate::utils::{find_and_parse_package_json, levenshtein_distance};

pub fn handle(script_name: Option<String>, args: Vec<String>, _if_present: bool) -> Result<()> {
    match script_name {
        Some(script) => {
            // Try fuzzy match if script not found
            let final_script = if script_exists(&script) {
                script
            } else {
                match fuzzy_find_script(&script)? {
                    Some(found) => {
                        println!(
                            "  {} Did you mean {}? Running it...\n",
                            style("›").dim(),
                            style(&found).cyan(),
                        );
                        found
                    }
                    None => script,
                }
            };

            let start = std::time::Instant::now();
            let result = run_script_fast(&final_script, &args);
            let duration = start.elapsed();

            if result.is_ok() {
                StyledOutput::completion(duration.as_secs_f64());
            }

            result?;
        }
        None => {
            show_available_scripts()?;
        }
    }
    Ok(())
}

fn script_exists(script_name: &str) -> bool {
    let cwd = match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => return false,
    };
    if let Ok((_path, package)) = find_and_parse_package_json(&cwd) {
        if let Some(scripts) = package.scripts {
            return scripts.contains_key(script_name);
        }
    }
    false
}

fn fuzzy_find_script(input: &str) -> Result<Option<String>> {
    let cwd = std::env::current_dir()?;
    let (_path, package) = find_and_parse_package_json(&cwd)?;
    let scripts = package.scripts.unwrap_or_default();

    let mut matches: Vec<(String, usize)> = scripts
        .keys()
        .map(|name| (name.clone(), levenshtein_distance(input, name)))
        .filter(|(_, dist)| *dist <= 2)
        .collect();

    matches.sort_by_key(|(_, dist)| *dist);

    Ok(matches.first().map(|(name, _)| name.clone()))
}

fn show_available_scripts() -> Result<()> {
    let cwd = std::env::current_dir()?;
    let (_path, package) = match find_and_parse_package_json(&cwd) {
        Ok(result) => result,
        Err(_) => {
            StyledOutput::error("No package.json found");
            std::process::exit(1);
        }
    };

    let scripts = package.scripts.unwrap_or_default();

    if scripts.is_empty() {
        StyledOutput::info("No scripts found in this package");
        StyledOutput::hint("Add scripts to your package.json to get started");
        return Ok(());
    }

    // Build interactive selection
    let max_name = scripts.keys().map(|k| k.len()).max().unwrap_or(10).min(24);

    let options: Vec<String> = scripts
        .iter()
        .map(|(name, cmd)| {
            let cmd_preview = if cmd.len() > 40 {
                format!("{}…", &cmd[..39])
            } else {
                cmd.clone()
            };
            format!(
                "{:<width$}  {}",
                name,
                style(&cmd_preview).dim(),
                width = max_name
            )
        })
        .collect();

    let script_names: Vec<String> = scripts.keys().cloned().collect();

    println!();
    match Select::new("Select a script to run:", options).prompt() {
        Ok(selection) => {
            if let Some(script_name) = script_names
                .iter()
                .find(|name| selection.starts_with(name.as_str()))
            {
                println!();

                let start = std::time::Instant::now();
                let result = run_script_fast(script_name, &[]);
                let duration = start.elapsed();

                if result.is_ok() {
                    StyledOutput::completion(duration.as_secs_f64());
                }

                result?;
            }
        }
        Err(_) => {
            println!();
            StyledOutput::dim("Cancelled");
        }
    }

    Ok(())
}
