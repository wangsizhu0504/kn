use crate::parse::parse_nu;
use crate::runner::run_cli;
use anyhow::Result;

pub fn handle(packages: Vec<String>, interactive: bool, latest: bool) -> Result<()> {
    let mut args = packages;

    if interactive {
        args.push("-i".to_string());
    }
    if latest {
        args.push("--latest".to_string());
    }

    run_cli(parse_nu, None, args)
}
