use crate::parse::parse_ni;
use crate::runner::run_cli;

pub fn handle(
    packages: Vec<String>,
    dev: bool,
    global: bool,
    exact: bool,
    ignore_scripts: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut args = packages.clone();

    // Handle flags
    if global {
        args.push("-g".to_string());
    } else {
        if dev {
            args.push("-D".to_string());
        }
        if exact {
            args.push("-E".to_string());
        }
    }

    // Add ignore-scripts flag if specified
    if ignore_scripts {
        args.push("--ignore-scripts".to_string());
    }

    // Simply pass through to package manager
    run_cli(parse_ni, None, args)
}
