use crate::detect::detect;
use crate::runner::DetectOptions;
use std::fs;
use std::process::Command;

pub fn handle(cache: bool, all: bool, global: bool) -> Result<(), Box<dyn std::error::Error>> {
    if all {
        clean_all()?;
    } else if cache {
        clean_cache()?;
    } else if global {
        clean_global()?;
    } else {
        clean_local()?;
    }

    Ok(())
}

fn clean_local() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\x1b[33m🧹 Cleaning local project...\x1b[0m\n");

    let paths = vec![
        "node_modules",
        ".turbo",
        ".next",
        "dist",
        "build",
        ".vite",
        ".nuxt",
    ];

    let mut removed = 0;
    let mut size_freed = 0u64;

    for path in paths {
        if let Ok(metadata) = fs::metadata(path) {
            if metadata.is_dir() {
                if let Ok(size) = dir_size(path) {
                    size_freed += size;
                }

                match fs::remove_dir_all(path) {
                    Ok(_) => {
                        println!("  \x1b[32m✓\x1b[0m Removed \x1b[36m{}\x1b[0m", path);
                        removed += 1;
                    }
                    Err(e) => {
                        println!("  \x1b[31m✗\x1b[0m Failed to remove {}: {}", path, e);
                    }
                }
            }
        }
    }

    if removed > 0 {
        println!(
            "\n\x1b[32m✓\x1b[0m Cleaned {} directories, freed ~{} MB\n",
            removed,
            size_freed / 1024 / 1024
        );
    } else {
        println!("\n\x1b[90mNothing to clean\x1b[0m\n");
    }

    Ok(())
}

fn clean_cache() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\x1b[33m🧹 Cleaning package manager cache...\x1b[0m\n");

    let agent_option = detect(DetectOptions {
        cwd: std::env::current_dir()?,
        ..Default::default()
    });

    let agent = match agent_option {
        Some(agent) => agent,
        None => {
            println!("  \x1b[90mCould not detect package manager\x1b[0m");
            return Ok(());
        }
    };

    let agent_str = format!("{:?}", agent).to_lowercase();

    let result = match agent_str.as_str() {
        "npm" | "npmbun" => Command::new("npm")
            .args(&["cache", "clean", "--force"])
            .status(),
        "yarn" | "yarnberry" => Command::new("yarn").args(&["cache", "clean"]).status(),
        "pnpm" => Command::new("pnpm").args(&["store", "prune"]).status(),
        "bun" => {
            // Bun doesn't have a cache clean command yet
            println!("  \x1b[90mBun cache cleaning not supported yet\x1b[0m");
            return Ok(());
        }
        _ => {
            println!("  \x1b[90mUnknown package manager: {}\x1b[0m", agent_str);
            return Ok(());
        }
    };

    match result {
        Ok(status) if status.success() => {
            println!(
                "\n\x1b[32m✓\x1b[0m {} cache cleaned successfully\n",
                agent_str
            );
        }
        _ => {
            println!("\n\x1b[31m✗\x1b[0m Failed to clean {} cache\n", agent_str);
        }
    }

    Ok(())
}

fn clean_global() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\x1b[33m🧹 Cleaning global packages...\x1b[0m\n");
    println!("\x1b[90mThis will list global packages. Remove them manually if needed.\x1b[0m\n");

    let agent_option = detect(DetectOptions {
        cwd: std::env::current_dir()?,
        ..Default::default()
    });

    let agent = match agent_option {
        Some(agent) => agent,
        None => {
            println!("  \x1b[90mCould not detect package manager\x1b[0m");
            return Ok(());
        }
    };

    let agent_str = format!("{:?}", agent).to_lowercase();

    let _ = match agent_str.as_str() {
        "npm" | "npmbun" => Command::new("npm")
            .args(&["list", "-g", "--depth=0"])
            .status(),
        "yarn" | "yarnberry" => Command::new("yarn").args(&["global", "list"]).status(),
        "pnpm" => Command::new("pnpm").args(&["list", "-g"]).status(),
        "bun" => Command::new("bun").args(&["pm", "ls", "-g"]).status(),
        _ => return Ok(()),
    };

    println!();
    Ok(())
}

fn clean_all() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n\x1b[33m🧹 Deep cleaning...\x1b[0m\n");

    clean_local()?;
    clean_cache()?;

    println!("\x1b[32m✓\x1b[0m Deep clean completed\n");

    Ok(())
}

fn dir_size(path: &str) -> Result<u64, std::io::Error> {
    let mut size = 0u64;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    size += metadata.len();
                } else if metadata.is_dir() {
                    if let Ok(subsize) = dir_size(&entry.path().to_string_lossy()) {
                        size += subsize;
                    }
                }
            }
        }
    }

    Ok(size)
}
