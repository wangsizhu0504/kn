use crate::command_utils::{parse_package_json};
use crate::display::StyledOutput;
use serde_json;
use std::process;

pub fn handle(json: bool) -> Result<(), Box<dyn std::error::Error>> {
    let spinner = StyledOutput::working("Searching for package.json...");

    // Output scripts in JSON format
    let mut current_dir = std::env::current_dir()?;
    let package_json_path = loop {
        let package_json_path = current_dir.join("package.json");
        if package_json_path.is_file() {
            break package_json_path.to_string_lossy().to_string();
        }
        if !current_dir.pop() {
            drop(spinner);
            StyledOutput::error("No package.json found");
            process::exit(1);
        }
    };

    let package = parse_package_json(&package_json_path)?;
    drop(spinner);

    if json {
        if let Some(scripts) = package.scripts {
            println!("{}", serde_json::to_string_pretty(&scripts)?);
        } else {
            println!("{{}}");
        }
    } else {
        // Show scripts with enhanced display
        let scripts = package.scripts.unwrap_or_default();
        if scripts.is_empty() {
            StyledOutput::info("No scripts found in this package");
        } else {
            let package_name = package.name.as_deref().unwrap_or("unknown");
            let package_version = package.version.as_deref().unwrap_or("0.0.0");
            StyledOutput::enhanced_list_scripts(package_name, package_version, &scripts);
        }
    }
    Ok(())
}
