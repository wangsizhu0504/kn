use crate::parse::parse_nun;
use crate::runner::run_cli;
use anyhow::Result;

pub fn handle(packages: Vec<String>, global: bool) -> Result<()> {
    let mut args = packages;

    if global {
        args.push("-g".to_string());
    }

    run_cli(parse_nun, None, args)
}
