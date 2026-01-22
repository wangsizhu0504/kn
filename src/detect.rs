use std::fs::File;
use std::path::Path;
use std::io::Read;

use crate::agents::{Agent, AGENT_MAP};

pub fn detect(options: crate::runner::DetectOptions) -> Option<Agent> {
    let cwd = options.cwd.clone();
    
    // Check for package.json in directory tree
    if let Some(package_json_path) = find_up("package.json", &cwd) {
        let mut file = File::open(&package_json_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        
        let json: serde_json::Value = serde_json::from_str(&contents).unwrap();
        
        if let Some(package_manager) = json.get("packageManager") {
            let pm_str = package_manager.as_str().unwrap();
            let parts = if let Some(stripped) = pm_str.strip_prefix('^') {
                String::from(stripped)
            } else {
                String::from(pm_str)
            };
            let parts = parts.split('@').collect::<Vec<&str>>();
            let name = parts[0];
            
            if name == "yarn" && parts.len() > 1 {
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
                        return Some(Agent::Pnpm6);
                    } else {
                        return Some(Agent::Pnpm);
                    }
                }
            }
            return AGENT_MAP.iter().find(|(n, _)| *n == name).map(|(_, agent)| *agent);
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
    
    for ancestor in cwd.ancestors() {
        for (lock_file, manager) in &lock_files {
            if ancestor.join(lock_file).exists() {
                return AGENT_MAP.iter().find(|(n, _)| *n == *manager).map(|(_, agent)| *agent);
            }
        }
    }
    
    // Fallback to npm if no lock files found
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