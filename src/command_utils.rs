use anyhow::{bail, Context, Result};
use indexmap::IndexMap;
use std::env;
use std::path::Path;
use std::process::Command;

use crate::agents::Agent;
use crate::detect::detect;
use crate::runner::DetectOptions;

#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct Package {
    pub name: Option<String>,
    pub version: Option<String>,
    pub scripts: Option<IndexMap<String, String>>,
}

#[cfg(test)]
pub fn parse_package_json(path: &str) -> Result<Package> {
    use std::fs;

    let contents = fs::read_to_string(path).with_context(|| format!("Failed to read {}", path))?;

    let json: serde_json::Value = serde_json::from_str(&contents)
        .with_context(|| format!("Failed to parse JSON in {}", path))?;

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

/// Detect the package manager for the current project (unified detection)
pub fn detect_agent() -> Agent {
    let options = DetectOptions {
        cwd: env::current_dir().unwrap_or_default(),
        ..Default::default()
    };
    detect(options).unwrap_or(Agent::Npm)
}

pub fn run_script_fast(script_name: &str, args: &[String]) -> Result<()> {
    let cwd = env::current_dir().context("Failed to get current directory")?;
    let (package_json_path, package) = crate::utils::find_and_parse_package_json(&cwd)?;

    let scripts = match package.scripts {
        Some(s) if !s.is_empty() => s,
        _ => bail!("No scripts found in package.json"),
    };

    let package_path = package_json_path.parent().unwrap_or_else(|| Path::new("."));

    if let Some(script_command) = scripts.get(script_name) {
        let agent = detect_agent();

        let mut cmd = Command::new(agent.name());
        cmd.arg("run").arg(script_name);

        // Add additional arguments
        if !args.is_empty() {
            // For npm and pnpm, need to add -- before extra args
            if matches!(agent, Agent::Npm | Agent::Pnpm | Agent::Pnpm6) {
                cmd.arg("--");
            }
            cmd.args(args);
        }

        let status = cmd
            .env("npm_lifecycle_event", script_name)
            .env("npm_lifecycle_script", script_command)
            .env(
                "npm_package_json",
                package_json_path.to_string_lossy().as_ref(),
            )
            .env("npm_execpath", env::current_exe().unwrap_or_default())
            .current_dir(package_path)
            .status()
            .with_context(|| format!("Failed to execute script '{}'", script_name))?;

        if !status.success() {
            let code = status.code().unwrap_or(1);
            std::process::exit(code);
        }
    } else {
        bail!("Script '{}' not found in package.json", script_name);
    }

    Ok(())
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
