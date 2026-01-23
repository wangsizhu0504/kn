use crate::parse::parse_ni;
use crate::runner::{run_cli, DetectOptions};

pub fn handle(force: bool, no_optional: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Vec::new();

    if force {
        args.push("--force".to_string());
    } else {
        args.push("--frozen-if-present".to_string());
    }

    if no_optional {
        args.push("--no-optional".to_string());
    }

    let options = DetectOptions::new().with_auto_install(true);

    run_cli(parse_ni, Some(options), args)
}
