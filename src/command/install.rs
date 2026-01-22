use crate::parse::parse_ni;
use crate::runner::run_cli;

pub fn handle(packages: Vec<String>, dev: bool, global: bool, exact: bool) -> Result<(), Box<dyn std::error::Error>> {
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
    
    run_cli(parse_ni, None, args)
}