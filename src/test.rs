use std::fs;
use std::path::Path;
use std::process::Command;

use crate::agents::{Agent, AGENT_MAP};
use crate::command_utils::parse_package_json;
use crate::detect::detect;
use crate::parse::{parse_ni, parse_nun, parse_nlx, parse_nu, parse_na};
use crate::runner::DetectOptions;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_package_json(content: &str, dir: &Path) {
        let package_json_path = dir.join("package.json");
        fs::write(package_json_path, content).expect("Failed to write package.json");
    }

    fn create_test_lock_file(lock_file: &str, dir: &Path) {
        let lock_path = dir.join(lock_file);
        fs::write(lock_path, "test lock content").expect("Failed to write lock file");
    }

    fn cleanup_test_dir(dir: &Path) {
        if dir.exists() {
            fs::remove_dir_all(dir).expect("Failed to cleanup test directory");
        }
    }

    #[test]
    fn test_package_parsing() {
        let test_dir = Path::new("/tmp/kn_test_package_parsing");
        cleanup_test_dir(test_dir);
        fs::create_dir_all(test_dir).expect("Failed to create test directory");

        let package_json_content = r#"
{
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
}
"#;

        create_test_package_json(package_json_content, test_dir);

        let package = parse_package_json(&test_dir.join("package.json").to_string_lossy())
            .expect("Failed to parse package.json");

        assert_eq!(package.name, Some("test-package".to_string()));
        assert_eq!(package.version, Some("1.0.0".to_string()));

        let scripts = package.scripts.expect("Scripts should be present");
        assert_eq!(scripts.get("start"), Some(&"node index.js".to_string()));
        assert_eq!(scripts.get("test"), Some(&"jest".to_string()));
        assert_eq!(scripts.get("build"), Some(&"webpack --mode production".to_string()));
        assert_eq!(scripts.len(), 3);

        cleanup_test_dir(test_dir);
    }

    #[test]
    fn test_package_parsing_no_scripts() {
        let test_dir = Path::new("/tmp/kn_test_no_scripts");
        cleanup_test_dir(test_dir);
        fs::create_dir_all(test_dir).expect("Failed to create test directory");

        let package_json_content = r#"
{
  "name": "no-scripts-package",
  "version": "1.0.0",
  "dependencies": {
    "lodash": "^4.17.21"
  }
}
"#;

        create_test_package_json(package_json_content, test_dir);

        let package = parse_package_json(&test_dir.join("package.json").to_string_lossy())
            .expect("Failed to parse package.json");

        assert_eq!(package.name, Some("no-scripts-package".to_string()));
        assert_eq!(package.version, Some("1.0.0".to_string()));
        assert!(package.scripts.is_none() || package.scripts.as_ref().unwrap().is_empty());

        cleanup_test_dir(test_dir);
    }

    #[test]
    fn test_npm_detection() {
        let test_dir = Path::new("/tmp/kn_test_npm_detection");
        cleanup_test_dir(test_dir);
        fs::create_dir_all(test_dir).expect("Failed to create test directory");

        create_test_package_json(r#"
{
  "name": "npm-test",
  "version": "1.0.0"
}
"#, test_dir);

        create_test_lock_file("package-lock.json", test_dir);

        let options = DetectOptions {
            cwd: test_dir.to_path_buf(),
            auto_install: false,
            programmatic: true,
        };

        let detected = detect(options);
        assert_eq!(detected, Some(Agent::Npm));

        cleanup_test_dir(test_dir);
    }

    #[test]
    fn test_yarn_detection() {
        let test_dir = Path::new("/tmp/kn_test_yarn_detection");
        cleanup_test_dir(test_dir);
        fs::create_dir_all(test_dir).expect("Failed to create test directory");

        create_test_package_json(r#"
{
  "name": "yarn-test",
  "version": "1.0.0"
}
"#, test_dir);

        create_test_lock_file("yarn.lock", test_dir);

        let options = DetectOptions {
            cwd: test_dir.to_path_buf(),
            auto_install: false,
            programmatic: true,
        };

        let detected = detect(options);
        assert_eq!(detected, Some(Agent::Yarn));

        cleanup_test_dir(test_dir);
    }

    #[test]
    fn test_pnpm_detection() {
        let test_dir = Path::new("/tmp/kn_test_pnpm_detection");
        cleanup_test_dir(test_dir);
        fs::create_dir_all(test_dir).expect("Failed to create test directory");

        create_test_package_json(r#"
{
  "name": "pnpm-test",
  "version": "1.0.0"
}
"#, test_dir);

        create_test_lock_file("pnpm-lock.yaml", test_dir);

        let options = DetectOptions {
            cwd: test_dir.to_path_buf(),
            auto_install: false,
            programmatic: true,
        };

        let detected = detect(options);
        assert_eq!(detected, Some(Agent::Pnpm));

        cleanup_test_dir(test_dir);
    }

    #[test]
    fn test_bun_detection() {
        let test_dir = Path::new("/tmp/kn_test_bun_detection");
        cleanup_test_dir(test_dir);
        fs::create_dir_all(test_dir).expect("Failed to create test directory");

        create_test_package_json(r#"
{
  "name": "bun-test",
  "version": "1.0.0"
}
"#, test_dir);

        create_test_lock_file("bun.lockb", test_dir);

        let options = DetectOptions {
            cwd: test_dir.to_path_buf(),
            auto_install: false,
            programmatic: true,
        };

        let detected = detect(options);
        assert_eq!(detected, Some(Agent::Bun));

        cleanup_test_dir(test_dir);
    }

    #[test]
    fn test_package_manager_field_detection() {
        // Test yarn berry detection
        let yarn_test_dir = Path::new("/tmp/kn_test_yarn_pm");
        cleanup_test_dir(yarn_test_dir);
        fs::create_dir_all(yarn_test_dir).expect("Failed to create test directory");

        create_test_package_json(r#"
{
  "name": "yarn-berry-test",
  "version": "1.0.0",
  "packageManager": "yarn@4.0.0"
}
"#, yarn_test_dir);

        let options = DetectOptions {
            cwd: yarn_test_dir.to_path_buf(),
            auto_install: false,
            programmatic: true,
        };

        let detected = detect(options.clone());
        assert_eq!(detected, Some(Agent::YarnBerry));

        // Test pnpm 6 detection
        let pnpm_test_dir = Path::new("/tmp/kn_test_pnpm_pm");
        cleanup_test_dir(pnpm_test_dir);
        fs::create_dir_all(pnpm_test_dir).expect("Failed to create test directory");

        create_test_package_json(r#"
{
  "name": "pnpm6-test",
  "version": "1.0.0",
  "packageManager": "pnpm@6.0.0"
}
"#, pnpm_test_dir);

        let pnpm_options = DetectOptions {
            cwd: pnpm_test_dir.to_path_buf(),
            auto_install: false,
            programmatic: true,
        };

        let detected = detect(pnpm_options);
        assert_eq!(detected, Some(Agent::Pnpm6));

        cleanup_test_dir(yarn_test_dir);
        cleanup_test_dir(pnpm_test_dir);
    }

    #[test]
    fn test_parse_ni() {
        // Test basic install
        let (cmd, args) = parse_ni(Agent::Npm, vec!["lodash".to_string()], None);
        assert_eq!(cmd, "npm");
        assert_eq!(args, vec!["install", "lodash"]);

        // Test global install
        let (cmd, args) = parse_ni(Agent::Npm, vec!["-g".to_string(), "typescript".to_string()], None);
        assert_eq!(cmd, "npm");
        assert!(args.contains(&"-g".to_string()));
        assert!(args.contains(&"typescript".to_string()));

        // Test frozen install
        let (cmd, args) = parse_ni(Agent::Npm, vec!["--frozen".to_string()], None);
        assert_eq!(cmd, "npm");
        assert_eq!(args, vec!["ci"]);

        // Test yarn install
        let (cmd, args) = parse_ni(Agent::Yarn, vec!["react".to_string()], None);
        assert_eq!(cmd, "yarn");
        assert_eq!(args, vec!["add", "react"]);

        // Test bun install
        let (cmd, args) = parse_ni(Agent::Bun, vec!["express".to_string()], None);
        assert_eq!(cmd, "bun");
        assert_eq!(args, vec!["add", "express"]);
    }

    #[test]
    fn test_parse_nun() {
        // Test basic uninstall
        let (cmd, args) = parse_nun(Agent::Npm, vec!["lodash".to_string()], None);
        assert_eq!(cmd, "npm");
        assert_eq!(args, vec!["uninstall", "lodash"]);

        // Test yarn uninstall
        let (cmd, args) = parse_nun(Agent::Yarn, vec!["react".to_string()], None);
        assert_eq!(cmd, "yarn");
        assert_eq!(args, vec!["remove", "react"]);

        // Test global uninstall
        let (cmd, args) = parse_nun(Agent::Pnpm, vec!["-g".to_string(), "typescript".to_string()], None);
        assert_eq!(cmd, "pnpm");
        assert!(args.contains(&"-g".to_string()));
        assert!(args.contains(&"typescript".to_string()));
    }

    #[test]
    fn test_parse_nlx() {
        // Test npm execute
        let (cmd, args) = parse_nlx(Agent::Npm, vec!["cowsay".to_string(), "Hello".to_string()], None);
        assert_eq!(cmd, "npx");
        assert_eq!(args, vec!["cowsay", "Hello"]);

        // Test yarn execute
        let (cmd, args) = parse_nlx(Agent::Yarn, vec!["cowsay".to_string(), "Hello".to_string()], None);
        assert_eq!(cmd, "yarn");
        assert_eq!(args, vec!["dlx", "cowsay", "Hello"]);

        // Test bun execute
        let (cmd, args) = parse_nlx(Agent::Bun, vec!["cowsay".to_string(), "Hello".to_string()], None);
        assert_eq!(cmd, "bunx");
        assert_eq!(args, vec!["cowsay", "Hello"]);
    }

    #[test]
    fn test_parse_nu() {
        // Test npm upgrade
        let (cmd, args) = parse_nu(Agent::Npm, vec!["lodash".to_string()], None);
        assert_eq!(cmd, "npm");
        assert_eq!(args, vec!["update", "lodash"]);

        // Test yarn upgrade
        let (cmd, args) = parse_nu(Agent::Yarn, vec!["react".to_string()], None);
        assert_eq!(cmd, "yarn");
        assert_eq!(args, vec!["upgrade", "react"]);

        // Test yarn berry upgrade
        let (cmd, args) = parse_nu(Agent::YarnBerry, vec!["react".to_string()], None);
        assert_eq!(cmd, "yarn");
        assert_eq!(args, vec!["up", "react"]);
    }

    #[test]
    fn test_parse_na() {
        // Test npm agent
        let (cmd, args) = parse_na(Agent::Npm, vec!["--version".to_string()], None);
        assert_eq!(cmd, "npm");
        assert_eq!(args, vec!["--version"]);

        // Test yarn agent
        let (cmd, args) = parse_na(Agent::Yarn, vec!["--version".to_string()], None);
        assert_eq!(cmd, "yarn");
        assert_eq!(args, vec!["--version"]);

        // Test pnpm agent
        let (cmd, args) = parse_na(Agent::Pnpm, vec!["--version".to_string()], None);
        assert_eq!(cmd, "pnpm");
        assert_eq!(args, vec!["--version"]);
    }

    #[test]
    fn test_agent_map() {
        // Test that all expected agents are in the map
        assert!(AGENT_MAP.iter().any(|(name, _)| *name == "npm"));
        assert!(AGENT_MAP.iter().any(|(name, _)| *name == "yarn"));
        assert!(AGENT_MAP.iter().any(|(name, _)| *name == "pnpm"));
        assert!(AGENT_MAP.iter().any(|(name, _)| *name == "bun"));
        assert!(AGENT_MAP.iter().any(|(name, _)| *name == "yarn@berry"));
        assert!(AGENT_MAP.iter().any(|(name, _)| *name == "pnpm@6"));
    }

    #[test]
    fn test_fallback_to_npm() {
        let test_dir = Path::new("/tmp/kn_test_fallback");
        cleanup_test_dir(test_dir);
        fs::create_dir_all(test_dir).expect("Failed to create test directory");

        create_test_package_json(r#"
{
  "name": "fallback-test",
  "version": "1.0.0"
}
"#, test_dir);

        let options = DetectOptions {
            cwd: test_dir.to_path_buf(),
            auto_install: false,
            programmatic: true,
        };

        let detected = detect(options);
        // Should fallback to npm when no lock file or packageManager field
        assert_eq!(detected, Some(Agent::Npm));

        cleanup_test_dir(test_dir);
    }

    #[test]
    fn test_cli_help_and_version() {
        // Test that CLI responds to help and version flags
        let bin_path = if cfg!(debug_assertions) {
            "./target/debug/kn"
        } else {
            "./target/release/kn"
        };

        let help_output = Command::new(bin_path)
            .arg("--help")
            .output()
            .expect("Failed to execute help command");

        assert!(help_output.status.success());
        let help_str = String::from_utf8_lossy(&help_output.stdout);
        assert!(help_str.contains("KN") || help_str.contains("kn"));

        let version_output = Command::new(bin_path)
            .arg("--version")
            .output()
            .expect("Failed to execute version command");

        assert!(version_output.status.success());
        let version_str = String::from_utf8_lossy(&version_output.stdout);
        assert!(version_str.contains("kn"));
        assert!(version_str.contains("0.1.0"));
    }

    #[test]
    fn test_empty_args_handling() {
        // Test that CLI handles empty arguments gracefully
        let bin_path = if cfg!(debug_assertions) {
            "./target/debug/kn"
        } else {
            "./target/release/kn"
        };

        let output = Command::new(bin_path)
            .output()
            .expect("Failed to execute empty command");

        // Should show help when no arguments provided
        let output_str = String::from_utf8_lossy(&output.stdout);
        let stderr_str = String::from_utf8_lossy(&output.stderr);

        // Check if help is shown in either stdout or stderr
        let help_shown = output_str.contains("Usage:") ||
                       output_str.contains("help") ||
                       stderr_str.contains("Usage:") ||
                       stderr_str.contains("help");

        assert!(help_shown, "Help should be shown when no arguments provided. stdout: {}, stderr: {}", output_str, stderr_str);
    }
}
