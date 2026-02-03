use std::fs::File;
use std::io::Read;
use std::path::Path;
use tracing::{debug, info};

use crate::agents::{Agent, AGENT_MAP};

pub fn detect(options: crate::runner::DetectOptions) -> Option<Agent> {
    let cwd = options.cwd.clone();
    debug!("Detecting package manager in {:?}", cwd);

    // Check for package.json in directory tree
    if let Some(package_json_path) = find_up("package.json", &cwd) {
        let mut file = File::open(&package_json_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let json: serde_json::Value = serde_json::from_str(&contents).unwrap();

        if let Some(package_manager) = json.get("packageManager") {
            let pm_str = package_manager.as_str().unwrap();
            debug!("Found packageManager field: {}", pm_str);
            let parts = if let Some(stripped) = pm_str.strip_prefix('^') {
                String::from(stripped)
            } else {
                String::from(pm_str)
            };
            let parts = parts.split('@').collect::<Vec<&str>>();
            let name = parts[0];

            if name == "yarn" && parts.len() > 1 {
                info!("Detected package manager: YarnBerry");
                return Some(Agent::YarnBerry);
            } else if name == "pnpm" && parts.len() > 1 {
                let ver_str = parts[1];
                // Extract the major version number before the first dot
                let major_ver = if let Some(dot_pos) = ver_str.find('.') {
                    &ver_str[..dot_pos]
                } else {
                    ver_str
                };
                if let Ok(ver) = major_ver.parse::<i32>() {
                    let ver = ver as i32;
                    if ver < 7 {
                        info!("Detected package manager: Pnpm6 (v{})", ver);
                        return Some(Agent::Pnpm6);
                    } else {
                        info!("Detected package manager: Pnpm (v{})", ver);
                        return Some(Agent::Pnpm);
                    }
                }
            }
            let agent = AGENT_MAP
                .iter()
                .find(|(n, _)| *n == name)
                .map(|(_, agent)| *agent);
            if let Some(agent) = agent {
                info!(
                    "Detected package manager from packageManager field: {:?}",
                    agent
                );
            }
            return agent;
        }
    }

    // Check for lock files as fallback
    let lock_files = [
        ("pnpm-lock.yaml", "pnpm"),
        ("yarn.lock", "yarn"),
        ("package-lock.json", "npm"),
        ("bun.lockb", "bun"),
        ("bun.lock", "bun"),
    ];

    // Only search up until we find a package.json (project root) or reach home directory
    let home_dir = dirs::home_dir();

    for ancestor in cwd.ancestors() {
        // Stop at home directory to avoid detecting unrelated lock files
        if let Some(ref home) = home_dir {
            if ancestor == home {
                break;
            }
        }

        // Check for lock files in this directory
        for (lock_file, manager) in &lock_files {
            if ancestor.join(lock_file).exists() {
                debug!("Found lock file: {} in {:?}", lock_file, ancestor);
                let agent = AGENT_MAP
                    .iter()
                    .find(|(n, _)| *n == *manager)
                    .map(|(_, agent)| *agent);
                if let Some(agent) = agent {
                    info!("Detected package manager from lock file: {:?}", agent);
                }
                return agent;
            }
        }

        // Stop at package.json (found project root)
        if ancestor.join("package.json").exists() {
            break;
        }
    }

    // Fallback to npm if no lock files found
    debug!("No package manager detected, falling back to npm");
    Some(Agent::Npm)
}

pub fn find_up(filename: &str, cwd: &Path) -> Option<String> {
    let mut cwd = cwd.to_path_buf();
    loop {
        let file_path = cwd.join(filename);
        if file_path.is_file() {
            return Some(file_path.to_string_lossy().into());
        }
        if !cwd.pop() {
            break;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::AGENT_MAP;
    use crate::runner::DetectOptions;
    use std::fs;
    use std::path::Path;

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
    fn test_npm_detection() {
        let test_dir = Path::new("/tmp/kn_test_npm_detection");
        cleanup_test_dir(test_dir);
        fs::create_dir_all(test_dir).expect("Failed to create test directory");

        create_test_package_json(
            r#"{
"name": "npm-test",
"version": "1.0.0"
}"#,
            test_dir,
        );

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

        create_test_package_json(
            r#"{
"name": "yarn-test",
"version": "1.0.0"
}"#,
            test_dir,
        );

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

        create_test_package_json(
            r#"{
"name": "pnpm-test",
"version": "1.0.0"
}"#,
            test_dir,
        );

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

        create_test_package_json(
            r#"{
"name": "bun-test",
"version": "1.0.0"
}"#,
            test_dir,
        );

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
        let yarn_test_dir = Path::new("/tmp/kn_test_yarn_pm");
        cleanup_test_dir(yarn_test_dir);
        fs::create_dir_all(yarn_test_dir).expect("Failed to create test directory");

        create_test_package_json(
            r#"{
"name": "yarn-berry-test",
"version": "1.0.0",
"packageManager": "yarn@4.0.0"
}"#,
            yarn_test_dir,
        );

        let options = DetectOptions {
            cwd: yarn_test_dir.to_path_buf(),
            auto_install: false,
            programmatic: true,
        };

        let detected = detect(options.clone());
        assert_eq!(detected, Some(Agent::YarnBerry));

        let pnpm_test_dir = Path::new("/tmp/kn_test_pnpm_pm");
        cleanup_test_dir(pnpm_test_dir);
        fs::create_dir_all(pnpm_test_dir).expect("Failed to create test directory");

        create_test_package_json(
            r#"{
"name": "pnpm6-test",
"version": "1.0.0",
"packageManager": "pnpm@6.0.0"
}"#,
            pnpm_test_dir,
        );

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
    fn test_agent_map() {
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

        create_test_package_json(
            r#"{
"name": "fallback-test",
"version": "1.0.0"
}"#,
            test_dir,
        );

        let options = DetectOptions {
            cwd: test_dir.to_path_buf(),
            auto_install: false,
            programmatic: true,
        };

        let detected = detect(options);
        assert_eq!(detected, Some(Agent::Npm));

        cleanup_test_dir(test_dir);
    }
}
