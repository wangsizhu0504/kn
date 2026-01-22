use crate::parse::parse_nun;
use crate::runner::run_cli;

pub fn handle(packages: Vec<String>, global: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut args = packages;
    
    // Handle global flag
    if global {
        args.push("-g".to_string());
    }
    
    run_cli(parse_nun, None, args)
}