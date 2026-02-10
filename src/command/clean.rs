use anyhow::Result;
use console::style;

use crate::agents::Agent;
use crate::detect::detect;
use crate::display::StyledOutput;
use crate::runner::DetectOptions;
use crate::utils::{dir_size, format_size};
use std::fs;
use std::process::Command;

pub fn handle(cache: bool, all: bool, global: bool) -> Result<()> {
    if all {
        clean_all()?;
    } else if cache {
        clean_cache(false)?;
    } else if global {
        clean_global()?;
    } else {
        clean_local(false)?;
    }

    Ok(())
}

fn detect_current_agent() -> Agent {
    let options = DetectOptions {
        cwd: std::env::current_dir().unwrap_or_default(),
        ..Default::default()
    };
    detect(options).unwrap_or(Agent::Npm)
}

/// Clean local build artifacts. When `quiet`, skip output (used in clean_all).
fn clean_local(quiet: bool) -> Result<(u32, u64)> {
    let spinner = if !quiet {
        Some(StyledOutput::working("Scanning local artifacts..."))
    } else {
        None
    };

    let paths = [
        "node_modules", ".turbo", ".next", "dist", "build", ".vite", ".nuxt",
    ];

    let mut removed = 0u32;
    let mut size_freed = 0u64;
    let mut removed_names = Vec::new();

    for path in paths {
        if let Ok(metadata) = fs::metadata(path) {
            if metadata.is_dir() {
                if let Ok(size) = dir_size(std::path::Path::new(path)) {
                    size_freed += size;
                }
                if fs::remove_dir_all(path).is_ok() {
                    removed += 1;
                    removed_names.push(path);
                }
            }
        }
    }

    drop(spinner);

    if !quiet {
        if removed > 0 {
            println!();
            println!("  {}", style("Cleaned").bold());
            for (i, name) in removed_names.iter().enumerate() {
                let is_last = i == removed_names.len() - 1;
                StyledOutput::tree_item(
                    &format!(
                        "{} {}",
                        style(name).cyan(),
                        style("removed").green(),
                    ),
                    is_last,
                );
            }
            println!();
            StyledOutput::success(&format!(
                "Freed ~{}",
                format_size(size_freed),
            ));
        } else {
            StyledOutput::info("Nothing to clean");
        }
        println!();
    }

    Ok((removed, size_freed))
}

/// Clean package manager cache. When `quiet`, skip output.
fn clean_cache(quiet: bool) -> Result<bool> {
    let spinner = if !quiet {
        Some(StyledOutput::working("Cleaning cache..."))
    } else {
        None
    };

    let agent = detect_current_agent();

    drop(spinner);

    if let Some((cmd, args)) = agent.cache_clean_args() {
        let result = Command::new(cmd).args(&args).status();

        match result {
            Ok(status) if status.success() => {
                if !quiet {
                    StyledOutput::success(&format!("{} cache cleaned", agent));
                    println!();
                }
                return Ok(true);
            }
            _ => {
                if !quiet {
                    StyledOutput::error(&format!("Failed to clean {} cache", agent));
                    println!();
                }
            }
        }
    } else if !quiet {
        StyledOutput::info(&format!("{} cache cleaning not supported", agent));
        println!();
    }

    Ok(false)
}

fn clean_global() -> Result<()> {
    StyledOutput::header("Global packages");
    StyledOutput::dim("Listing global packages. Remove them manually if needed.");
    println!();

    let agent = detect_current_agent();
    let (cmd, args) = agent.global_list_args();
    let _ = Command::new(cmd).args(&args).status();

    println!();
    Ok(())
}

fn clean_all() -> Result<()> {
    let spinner = StyledOutput::working("Running deep clean...");

    let (removed, size_freed) = clean_local(true)?;
    let cache_ok = clean_cache(true)?;

    drop(spinner);

    // ── Summary card ──
    let mut lines = Vec::new();

    if removed > 0 {
        lines.push(format!(
            "{} {} directories removed, ~{} freed",
            style("✔").green(),
            removed,
            format_size(size_freed),
        ));
    }

    if cache_ok {
        lines.push(format!(
            "{} Cache cleaned",
            style("✔").green(),
        ));
    }

    if removed == 0 && !cache_ok {
        lines.push(format!("{}", style("Nothing to clean").dim()));
    }

    println!();
    StyledOutput::titled("Deep Clean");
    println!();
    for line in &lines {
        StyledOutput::body(line);
    }
    println!();

    Ok(())
}
