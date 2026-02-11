use crate::parse::parse_ni;
use crate::runner::run_cli;
use anyhow::Result;

pub fn handle(
    packages: Vec<String>,
    dev: bool,
    global: bool,
    exact: bool,
    ignore_scripts: bool,
) -> Result<()> {
    let mut args = packages;

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

    if ignore_scripts {
        args.push("--ignore-scripts".to_string());
    }

    run_cli(parse_ni, None, args)
}
