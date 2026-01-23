use crate::agents::Agent;
use crate::parse::parse_na;

#[test]
fn test_parse_na() {
    // Test npm agent
    let (cmd, args) = parse_na(Agent::Npm, vec!["--version".to_string()], None);
    assert_eq!(cmd, "npm");
    assert_eq!(args, vec!["--version"]);

    // Test yarn agent
    let (cmd, args) = parse_na(Agent::Yarn, vec!["--version".to_string()], None);
    assert_eq!(cmd, "yarn");
    assert_eq!(args, vec!["--version"]);

    // Test pnpm agent
    let (cmd, args) = parse_na(Agent::Pnpm, vec!["--version".to_string()], None);
    assert_eq!(cmd, "pnpm");
    assert_eq!(args, vec!["--version"]);
}
