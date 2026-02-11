use crate::display::StyledOutput;
use crate::utils::find_and_parse_package_json;
use anyhow::Result;

pub fn handle(json: bool) -> Result<()> {
    let spinner = StyledOutput::working("Searching for package.json...");

    let cwd = std::env::current_dir()?;
    let (_path, package) = match find_and_parse_package_json(&cwd) {
        Ok(result) => {
            drop(spinner);
            result
        }
        Err(e) => {
            drop(spinner);
            StyledOutput::error(&e.to_string());
            std::process::exit(1);
        }
    };

    if json {
        if let Some(scripts) = package.scripts {
            println!("{}", serde_json::to_string_pretty(&scripts)?);
        } else {
            println!("{{}}");
        }
    } else {
        let scripts = package.scripts.unwrap_or_default();
        if scripts.is_empty() {
            StyledOutput::info("No scripts found in this package");
        } else {
            let package_name = package.name.as_deref().unwrap_or("unknown");
            let package_version = package.version.as_deref().unwrap_or("0.0.0");
            StyledOutput::list_scripts(package_name, package_version, &scripts);
        }
    }
    Ok(())
}
