use crate::parse::parse_ni;
use crate::runner::{run_cli, get_cli_command_direct, DetectOptions};
use crate::display::StyledOutput;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::time::Instant;

pub fn handle(packages: Vec<String>, dev: bool, global: bool, exact: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut args = packages.clone();

    // Handle flags
    if global {
        args.push("-g".to_string());
    } else {
        if dev {
            args.push("-D".to_string());
        }
        if exact {
            args.push("-E".to_string());
        }
    }

    // If no packages specified, just run the install command
    if packages.is_empty() {
        return run_cli(parse_ni, None, args);
    }

    // Enhanced install with progress for specific packages
    let options = DetectOptions::new();
    let command = get_cli_command_direct(parse_ni, args.clone(), options)?;

    if let Some((agent, cmd_args)) = command {
        install_with_progress(&agent, &cmd_args, &packages, dev, global)?;
        Ok(())
    } else {
        run_cli(parse_ni, None, args)
    }
}

fn install_with_progress(
    agent: &str,
    args: &[String],
    packages: &[String],
    dev: bool,
    global: bool
) -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    // Show header
    StyledOutput::header("Installing Packages");

    // Show what we're installing
    let install_type = if global {
        "globally"
    } else if dev {
        "as dev dependencies"
    } else {
        "as dependencies"
    };

    println!();
    StyledOutput::info(&format!("Installing {} package(s) {}", packages.len(), install_type));

    for (i, pkg) in packages.iter().enumerate() {
        println!("  {}. {}", i + 1, pkg);
    }
    println!();

    // Show package manager
    StyledOutput::section_title(&format!("Using {}", agent));
    println!();

    // Execute command
    let mut child = Command::new(agent)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Capture and display output with filtering
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines().flatten() {
            // Filter out some verbose lines, show important ones
            if should_display_line(&line) {
                println!("  {}", line);
            }
        }
    }

    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines().flatten() {
            if should_display_line(&line) {
                StyledOutput::warning(&line);
            }
        }
    }

    let status = child.wait()?;
    let duration = start_time.elapsed();

    println!();

    if status.success() {
        StyledOutput::success(&format!(
            "✓ Successfully installed {} package(s) in {:.2}s",
            packages.len(),
            duration.as_secs_f64()
        ));
    } else {
        StyledOutput::error(&format!(
            "✗ Installation failed after {:.2}s",
            duration.as_secs_f64()
        ));
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}

fn should_display_line(line: &str) -> bool {
    let line_lower = line.to_lowercase();

    // Skip progress bars and very verbose output
    if line.contains("█") || line.contains("▓") || line.contains("░") {
        return false;
    }

    // Skip fetch progress
    if line_lower.contains("fetch") && (line.contains("%") || line.contains("/")) {
        return false;
    }

    // Skip download progress percentages
    if line.contains("%") && (line_lower.contains("download") || line_lower.contains("progress")) {
        return false;
    }

    // Skip empty lines
    if line.trim().is_empty() {
        return false;
    }

    // Show warnings and errors
    if line_lower.contains("warn") || line_lower.contains("error") || line_lower.contains("deprecated") {
        return true;
    }

    // Show package additions
    if line_lower.contains("added") || line_lower.contains("installed") || line_lower.contains("✓") {
        return true;
    }

    // Show important summary lines
    if line_lower.contains("packages") || line_lower.contains("dependencies") {
        return true;
    }

    // Skip most other verbose output
    false
}
