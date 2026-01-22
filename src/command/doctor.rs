use crate::display::StyledOutput;
use crate::command_utils::detect_package_manager_fast;
use std::process::Command;
use std::path::Path;
use std::fs;
use serde_json::Value;

pub fn handle() -> Result<(), Box<dyn std::error::Error>> {
    StyledOutput::header("Project Health Check");
    println!();

    let mut passed = 0;
    let mut warnings = 0;
    let mut errors = 0;

    // Check 1: Package.json exists and is valid
    let result = check_package_json()?;
    passed += result.0;
    warnings += result.1;
    errors += result.2;

    // Check 2: Node modules installed
    let result = check_node_modules()?;
    passed += result.0;
    warnings += result.1;
    errors += result.2;

    // Check 3: Security audit
    let result = check_security()?;
    passed += result.0;
    warnings += result.1;
    errors += result.2;

    // Check 4: Node.js version
    let result = check_node_version()?;
    passed += result.0;
    warnings += result.1;
    errors += result.2;

    // Check 5: Lock file consistency
    let result = check_lock_files()?;
    passed += result.0;
    warnings += result.1;
    errors += result.2;

    // Check 6: Duplicate dependencies
    let result = check_duplicates()?;
    passed += result.0;
    warnings += result.1;
    errors += result.2;

    // Summary
    StyledOutput::summary_box("Health Check Summary", passed, warnings, errors);

    if errors == 0 && warnings == 0 {
        println!();
        StyledOutput::success("✨ Excellent! Your project is in great shape!");
    } else if errors > 0 {
        println!();
        StyledOutput::error(&format!("Found {} critical issue(s) that need attention", errors));
    } else {
        println!();
        StyledOutput::warning(&format!("Found {} warning(s) - consider addressing these", warnings));
    }
    println!();

    Ok(())
}

fn check_package_json() -> Result<(usize, usize, usize), Box<dyn std::error::Error>> {
    StyledOutput::section_title("📋 Package.json Validation");

    if !Path::new("package.json").exists() {
        StyledOutput::check_item(false, "package.json not found");
        return Ok((0, 0, 1));
    }

    match fs::read_to_string("package.json") {
        Ok(content) => {
            match serde_json::from_str::<Value>(&content) {
                Ok(json) => {
                    let mut passed = 0;
                    let mut warnings = 0;

                    // Check for required fields
                    if json.get("name").is_some() {
                        StyledOutput::check_item(true, "Package name defined");
                        passed += 1;
                    } else {
                        StyledOutput::check_item(false, "Missing 'name' field");
                        warnings += 1;
                    }

                    if json.get("version").is_some() {
                        StyledOutput::check_item(true, "Version defined");
                        passed += 1;
                    } else {
                        StyledOutput::check_item(false, "Missing 'version' field");
                        warnings += 1;
                    }

                    // Check for dependencies
                    let dep_count = json.get("dependencies")
                        .and_then(|d| d.as_object())
                        .map(|d| d.len())
                        .unwrap_or(0);
                    let dev_dep_count = json.get("devDependencies")
                        .and_then(|d| d.as_object())
                        .map(|d| d.len())
                        .unwrap_or(0);

                    if dep_count > 0 || dev_dep_count > 0 {
                        StyledOutput::check_item(true, &format!("{} dependencies, {} devDependencies", dep_count, dev_dep_count));
                        passed += 1;
                    } else {
                        StyledOutput::detail_item("ℹ", "No dependencies defined");
                    }

                    // Check for scripts
                    if let Some(scripts) = json.get("scripts").and_then(|s| s.as_object()) {
                        StyledOutput::check_item(true, &format!("{} scripts defined", scripts.len()));
                        passed += 1;
                    }

                    Ok((passed, warnings, 0))
                }
                Err(e) => {
                    StyledOutput::check_item(false, &format!("Invalid JSON: {}", e));
                    Ok((0, 0, 1))
                }
            }
        }
        Err(e) => {
            StyledOutput::check_item(false, &format!("Cannot read file: {}", e));
            Ok((0, 0, 1))
        }
    }
}

fn check_node_modules() -> Result<(usize, usize, usize), Box<dyn std::error::Error>> {
    StyledOutput::section_title("📦 Dependencies Installation");

    if !Path::new("node_modules").exists() {
        StyledOutput::check_item(false, "node_modules not found");
        StyledOutput::detail_item("💡", "Run 'kn install' to install dependencies");
        return Ok((0, 1, 0));
    }

    // Count installed packages
    if let Ok(entries) = fs::read_dir("node_modules") {
        let packages: Vec<_> = entries
            .filter_map(|e| e.ok())
            .filter(|e| {
                let name = e.file_name().to_string_lossy().to_string();
                !name.starts_with('.') && e.path().is_dir()
            })
            .collect();

        let count = packages.len();
        StyledOutput::check_item(true, &format!("{} packages installed", count));

        // Check for common packages
        let has_typescript = packages.iter().any(|e| e.file_name() == "typescript");
        let has_eslint = packages.iter().any(|e| e.file_name() == "eslint");

        if has_typescript {
            StyledOutput::detail_item("✓", "TypeScript installed");
        }
        if has_eslint {
            StyledOutput::detail_item("✓", "ESLint installed");
        }

        Ok((1, 0, 0))
    } else {
        Ok((0, 0, 0))
    }
}

fn check_security() -> Result<(usize, usize, usize), Box<dyn std::error::Error>> {
    StyledOutput::section_title("🔒 Security Audit");

    let manager = detect_package_manager_fast()?;

    let audit_cmd = match manager.as_str() {
        "npm" => vec!["audit", "--json"],
        "yarn" => vec!["audit", "--json"],
        "pnpm" => vec!["audit", "--json"],
        "bun" => {
            StyledOutput::detail_item("ℹ", "Bun doesn't support security audit yet");
            return Ok((0, 0, 0));
        }
        _ => vec!["audit", "--json"],
    };

    match Command::new(&manager).args(&audit_cmd).output() {
        Ok(output) => {
            if let Ok(result) = String::from_utf8(output.stdout) {
                if let Ok(json) = serde_json::from_str::<Value>(&result) {
                    let vulnerabilities = json.get("metadata")
                        .and_then(|m| m.get("vulnerabilities"))
                        .and_then(|v| v.as_object());

                    if let Some(vuln) = vulnerabilities {
                        let critical = vuln.get("critical").and_then(|v| v.as_u64()).unwrap_or(0);
                        let high = vuln.get("high").and_then(|v| v.as_u64()).unwrap_or(0);
                        let moderate = vuln.get("moderate").and_then(|v| v.as_u64()).unwrap_or(0);
                        let low = vuln.get("low").and_then(|v| v.as_u64()).unwrap_or(0);

                        let total = critical + high + moderate + low;

                        if total == 0 {
                            StyledOutput::check_item(true, "No known vulnerabilities");
                            return Ok((1, 0, 0));
                        } else {
                            let mut errors = 0;
                            let mut warnings = 0;

                            if critical > 0 {
                                StyledOutput::check_item(false, &format!("{} critical vulnerabilities", critical));
                                errors += 1;
                            }
                            if high > 0 {
                                StyledOutput::check_item(false, &format!("{} high vulnerabilities", high));
                                warnings += 1;
                            }
                            if moderate > 0 || low > 0 {
                                StyledOutput::detail_item("⚠", &format!("{} moderate, {} low", moderate, low));
                            }

                            StyledOutput::detail_item("💡", &format!("Run '{} audit fix' to fix automatically", manager));
                            return Ok((0, warnings, errors));
                        }
                    }
                }
            }
            StyledOutput::detail_item("ℹ", "Unable to parse audit results");
            Ok((0, 0, 0))
        }
        Err(_) => {
            StyledOutput::detail_item("ℹ", "Security audit not available");
            Ok((0, 0, 0))
        }
    }
}

fn check_node_version() -> Result<(usize, usize, usize), Box<dyn std::error::Error>> {
    StyledOutput::section_title("🚀 Node.js Runtime");

    match Command::new("node").arg("--version").output() {
        Ok(output) => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            StyledOutput::check_item(true, &format!("Node.js {}", version));

            // Check against package.json engines
            if let Ok(content) = fs::read_to_string("package.json") {
                if let Ok(json) = serde_json::from_str::<Value>(&content) {
                    if let Some(engines) = json.get("engines").and_then(|e| e.get("node")) {
                        StyledOutput::detail_item("ℹ", &format!("Required: node {}", engines));
                    }
                }
            }

            Ok((1, 0, 0))
        }
        Err(_) => {
            StyledOutput::check_item(false, "Node.js not found");
            Ok((0, 0, 1))
        }
    }
}

fn check_lock_files() -> Result<(usize, usize, usize), Box<dyn std::error::Error>> {
    StyledOutput::section_title("🔒 Lock Files");

    let lock_files = [
        ("package-lock.json", "npm"),
        ("yarn.lock", "yarn"),
        ("pnpm-lock.yaml", "pnpm"),
        ("bun.lockb", "bun"),
        ("bun.lock", "bun"),
    ];

    let found: Vec<(&str, &str)> = lock_files
        .iter()
        .filter(|(f, _)| Path::new(f).exists())
        .copied()
        .collect();

    if found.is_empty() {
        StyledOutput::check_item(false, "No lock file found");
        StyledOutput::detail_item("💡", "Lock files ensure consistent installs across environments");
        return Ok((0, 1, 0));
    }

    if found.len() > 1 {
        StyledOutput::check_item(false, "Multiple lock files detected");
        for (file, manager) in &found {
            StyledOutput::detail_item("•", &format!("{} ({})", file, manager));
        }
        StyledOutput::detail_item("💡", "Keep only one lock file for consistency");
        return Ok((0, 1, 0));
    }

    let (file, manager) = found[0];
    StyledOutput::check_item(true, &format!("{} ({} lockfile)", file, manager));
    Ok((1, 0, 0))
}

fn check_duplicates() -> Result<(usize, usize, usize), Box<dyn std::error::Error>> {
    StyledOutput::section_title("🔍 Dependency Analysis");

    if !Path::new("package.json").exists() {
        return Ok((0, 0, 0));
    }

    match fs::read_to_string("package.json") {
        Ok(content) => {
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                let deps = json.get("dependencies")
                    .and_then(|d| d.as_object())
                    .map(|d| d.keys().cloned().collect::<Vec<_>>())
                    .unwrap_or_default();

                let dev_deps = json.get("devDependencies")
                    .and_then(|d| d.as_object())
                    .map(|d| d.keys().cloned().collect::<Vec<_>>())
                    .unwrap_or_default();

                let duplicates: Vec<_> = deps
                    .iter()
                    .filter(|d| dev_deps.contains(d))
                    .collect();

                if duplicates.is_empty() {
                    StyledOutput::check_item(true, "No duplicate dependencies");
                    Ok((1, 0, 0))
                } else {
                    StyledOutput::check_item(false, &format!("{} duplicate(s) in both deps & devDeps", duplicates.len()));
                    for dup in duplicates.iter().take(3) {
                        StyledOutput::detail_item("•", dup);
                    }
                    if duplicates.len() > 3 {
                        StyledOutput::detail_item("•", &format!("... and {} more", duplicates.len() - 3));
                    }
                    StyledOutput::detail_item("💡", "Move to either dependencies or devDependencies");
                    Ok((0, 1, 0))
                }
            } else {
                Ok((0, 0, 0))
            }
        }
        Err(_) => Ok((0, 0, 0))
    }
}
