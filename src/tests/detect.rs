use std::fs;
use std::path::Path;
use crate::agents::{Agent, AGENT_MAP};
use crate::detect::detect;
use crate::runner::DetectOptions;
use super::utils::{cleanup_test_dir, create_test_package_json, create_test_lock_file};

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
