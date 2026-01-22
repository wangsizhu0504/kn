use crate::command_utils::{detect_package_manager_fast};
use crate::display::StyledOutput;
use std::process;

pub fn handle(verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    StyledOutput::header("Package Manager Information");
    
    let spinner = StyledOutput::working("Analyzing environment...");
    let manager = detect_package_manager_fast()?;
    drop(spinner); // Explicitly drop spinner
    
    let version = get_package_manager_version(&manager)?;
    StyledOutput::package_info("Package Manager", &version, &manager);
    
    // Show lock file info
    show_lock_file_info(&manager)?;
    
    // Show node info if available
    show_node_info();
    
    if verbose {
        StyledOutput::section_title("Environment Details");
        show_verbose_info();
    }
    
    Ok(())
}

fn get_package_manager_version(manager: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = process::Command::new(manager)
        .arg("--version")
        .output()?;
    
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn show_lock_file_info(manager: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    
    let current_dir = env::current_dir()?;
    let lock_files = [
        ("pnpm-lock.yaml", "pnpm"),
        ("yarn.lock", "yarn"),
        ("package-lock.json", "npm"),
        ("bun.lockb", "bun"),
        ("bun.lock", "bun"),
    ];
    
    StyledOutput::section_title("Lock File Analysis");
    
    for (lock_file, lock_manager) in &lock_files {
        let lock_path = current_dir.join(lock_file);
        if lock_path.exists() {
            if *lock_manager == manager {
                StyledOutput::success(&format!("Found matching lock file: {}", lock_file));
            } else {
                StyledOutput::warning(&format!("Lock file mismatch: {} (detected {})", lock_file, manager));
            }
            return Ok(());
        }
    }
    
    StyledOutput::info("No lock file found in project");
    Ok(())
}

fn show_node_info() {
    StyledOutput::section_title("Runtime Information");
    
    if let Ok(output) = std::process::Command::new("node")
        .arg("--version")
        .output() 
    {
        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        StyledOutput::key_value("Node.js", &version);
    } else {
        StyledOutput::warning("Node.js not found in PATH");
    }
}

fn show_verbose_info() {
    use std::env;
    
    StyledOutput::key_value("Working Directory", &env::current_dir()
        .unwrap_or_else(|_| "<unknown>".into())
        .display()
        .to_string()
    );
    
    // Show npm config
    if let Ok(output) = process::Command::new("npm")
        .args(["config", "get", "prefix"])
        .output() 
    {
        let prefix_str = String::from_utf8_lossy(&output.stdout).into_owned();
        let prefix_str = prefix_str.trim();
        if !prefix_str.is_empty() {
            StyledOutput::key_value("NPM Global Prefix", prefix_str);
        }
    }
    
    // Show environment
    if let Ok(cache_dir) = env::var("npm_config_cache") {
        StyledOutput::key_value("NPM Cache Dir", &cache_dir);
    }
}