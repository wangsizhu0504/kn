use crate::parse::parse_nu;
use crate::runner::run_cli;

pub fn handle(
    packages: Vec<String>,
    interactive: bool,
    latest: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut args = packages;

    // Handle flags
    if interactive {
        args.push("-i".to_string());
    }
    if latest {
        args.push("--latest".to_string());
    }

    run_cli(parse_nu, None, args)
}
