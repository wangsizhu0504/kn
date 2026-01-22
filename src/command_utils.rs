use std::env;
use std::fs;
use std::io::Read;
use std::path::{Path};
use std::process::Command;
use serde_json;
use indexmap::IndexMap;

#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct Package {
    pub name: Option<String>,
    pub version: Option<String>,
    pub scripts: Option<IndexMap<String, String>>,
}

pub fn parse_package_json(path: &str) -> Result<Package, Box<dyn std::error::Error>> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let json: serde_json::Value = serde_json::from_str(&contents)?;

    let scripts = if let serde_json::Value::Object(scripts_map) = &json["scripts"] {
        Some(scripts_map.iter()
            .filter_map(|(k, v)| {
                if let serde_json::Value::String(cmd) = v {
                    Some((k.clone(), cmd.clone()))
                } else {
                    None
                }
            })
            .collect::<IndexMap<String, String>>())
    } else {
        None
    };

    Ok(Package {
        name: json["name"].as_str().map(|s| s.to_string()),
        version: json["version"].as_str().map(|s| s.to_string()),
        scripts,
    })
}

pub fn run_script_fast(script_name: &str, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    // Find package.json up to directory tree
    let mut current_dir = env::current_dir()?;
    let package_json_path = loop {
        let package_json_path = current_dir.join("package.json");
        if package_json_path.is_file() {
            break package_json_path.to_string_lossy().to_string();
        }
        if !current_dir.pop() {
            eprintln!("No package.json found");
            std::process::exit(1);
        }
    };

    let package = parse_package_json(&package_json_path)?;
    let scripts = match package.scripts {
        Some(s) if !s.is_empty() => s,
        _ => {
            eprintln!("No scripts found in package.json");
            std::process::exit(1);
        }
    };

    let package_path = Path::new(&package_json_path).parent().unwrap_or_else(|| Path::new("."));

    if let Some(script_command) = scripts.get(script_name) {
        let (shell, shell_arg) = if cfg!(target_os = "windows") {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };

        let mut cmd = Command::new(shell);

        cmd.arg(shell_arg).arg(script_command)
            .env("npm_lifecycle_event", script_name)
            .env("npm_lifecycle_script", script_command)
            .env("npm_package_json", &package_json_path)
            .env("npm_execpath", std::env::current_exe()?)
            .current_dir(package_path)
            .args(args)
            .status()?;
    } else {
        eprintln!("Script '{}' not found", script_name);
        std::process::exit(1);
    }

    Ok(())
}

pub fn detect_package_manager_fast() -> Result<String, Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;

    // Check for lock files in order of preference
    let lock_files = [
        ("pnpm-lock.yaml", "pnpm"),
        ("yarn.lock", "yarn"),
        ("package-lock.json", "npm"),
        ("bun.lockb", "bun"),
        ("bun.lock", "bun"),
    ];

    // Only search up until we find a package.json (project root) or reach home directory
    let home_dir = dirs::home_dir();

    for ancestor in current_dir.ancestors() {
        // Stop at home directory to avoid detecting unrelated lock files
        if let Some(ref home) = home_dir {
            if ancestor == home {
                break;
            }
        }

        // Check for lock files in this directory
        for (lock_file, manager) in &lock_files {
            if ancestor.join(lock_file).exists() {
                return Ok(manager.to_string());
            }
        }

        // Stop at package.json (found project root)
        if ancestor.join("package.json").exists() {
            break;
        }
    }

    // Fallback to npm if no lock files found
    Ok("npm".to_string())
}
