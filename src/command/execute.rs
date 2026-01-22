use crate::parse::parse_nlx;
use crate::runner::run_cli;

pub fn handle(command: String, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut full_args = vec![command];
    full_args.extend(args);
    
    run_cli(parse_nlx, None, full_args)
}