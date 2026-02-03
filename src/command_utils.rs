use indexmap::IndexMap;
use serde_json;
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::Command;

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
        Some(
            scripts_map
                .iter()
                .filter_map(|(k, v)| {
                    if let serde_json::Value::String(cmd) = v {
                        Some((k.clone(), cmd.clone()))
                    } else {
                        None
                    }
                })
                .collect::<IndexMap<String, String>>(),
        )
    } else {
        None
    };

    Ok(Package {
        name: json["name"].as_str().map(|s| s.to_string()),
        version: json["version"].as_str().map(|s| s.to_string()),
        scripts,
    })
}

pub fn run_script_fast(
    script_name: &str,
    args: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
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

    let package_path = Path::new(&package_json_path)
        .parent()
        .unwrap_or_else(|| Path::new("."));

    if let Some(script_command) = scripts.get(script_name) {
        // Detect package manager to run the script
        let pm = detect_package_manager_fast()?;

        let mut cmd = Command::new(&pm);

        // Different package managers have different run command formats
        match pm.as_str() {
            "npm" | "pnpm" | "yarn" | "bun" => {
                cmd.arg("run").arg(script_name);
            }
            _ => {
                cmd.arg("run").arg(script_name);
            }
        }

        // Add additional arguments
        if !args.is_empty() {
            // For npm and pnpm, need to add -- before extra args
            if pm == "npm" || pm == "pnpm" {
                cmd.arg("--");
            }
            cmd.args(args);
        }

        cmd.env("npm_lifecycle_event", script_name)
            .env("npm_lifecycle_script", script_command)
            .env("npm_package_json", &package_json_path)
            .env("npm_execpath", std::env::current_exe()?)
            .current_dir(package_path)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn cleanup_test_dir(dir: &Path) {
        if dir.exists() {
            fs::remove_dir_all(dir).expect("Failed to cleanup test directory");
        }
    }

    fn create_test_package_json(content: &str, dir: &Path) {
        let package_json_path = dir.join("package.json");
        fs::write(package_json_path, content).expect("Failed to write package.json");
    }

    #[test]
    fn test_package_parsing() {
        let test_dir = Path::new("/tmp/kn_test_package_parsing");
        cleanup_test_dir(test_dir);
        fs::create_dir_all(test_dir).expect("Failed to create test directory");

        let package_json_content = r#"{
"name": "test-package",
"version": "1.0.0",
"scripts": {
"start": "node index.js",
"test": "jest",
"build": "webpack --mode production"
},
"dependencies": {
"lodash": "^4.17.21",
"express": "^4.18.2"
},
"devDependencies": {
"jest": "^29.0.0"
}
}"#;

        create_test_package_json(package_json_content, test_dir);

        let package = parse_package_json(&test_dir.join("package.json").to_string_lossy())
            .expect("Failed to parse package.json");

        assert_eq!(package.name, Some("test-package".to_string()));
        assert_eq!(package.version, Some("1.0.0".to_string()));

        let scripts = package.scripts.expect("Scripts should be present");
        assert_eq!(scripts.get("start"), Some(&"node index.js".to_string()));
        assert_eq!(scripts.get("test"), Some(&"jest".to_string()));
        assert_eq!(
            scripts.get("build"),
            Some(&"webpack --mode production".to_string())
        );
        assert_eq!(scripts.len(), 3);

        cleanup_test_dir(test_dir);
    }

    #[test]
    fn test_package_parsing_no_scripts() {
        let test_dir = Path::new("/tmp/kn_test_no_scripts");
        cleanup_test_dir(test_dir);
        fs::create_dir_all(test_dir).expect("Failed to create test directory");

        let package_json_content = r#"{
"name": "no-scripts-package",
"version": "1.0.0",
"dependencies": {
"lodash": "^4.17.21"
}
}"#;

        create_test_package_json(package_json_content, test_dir);

        let package = parse_package_json(&test_dir.join("package.json").to_string_lossy())
            .expect("Failed to parse package.json");

        assert_eq!(package.name, Some("no-scripts-package".to_string()));
        assert_eq!(package.version, Some("1.0.0".to_string()));
        assert!(package.scripts.is_none() || package.scripts.as_ref().unwrap().is_empty());

        cleanup_test_dir(test_dir);
    }
}
