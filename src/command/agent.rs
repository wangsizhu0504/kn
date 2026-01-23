use crate::parse::parse_na;
use crate::runner::run_cli;

pub fn handle(
    _manager: Option<String>,
    args: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    run_cli(parse_na, None, args)
}
