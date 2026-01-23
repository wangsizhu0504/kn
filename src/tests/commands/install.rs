use crate::agents::Agent;
use crate::parse::parse_ni;

#[test]
fn test_parse_ni() {
    // Test basic install
    let (cmd, args) = parse_ni(Agent::Npm, vec!["lodash".to_string()], None);
    assert_eq!(cmd, "npm");
    assert_eq!(args, vec!["install", "lodash"]);

    // Test global install
    let (cmd, args) = parse_ni(Agent::Npm, vec!["-g".to_string(), "typescript".to_string()], None);
    assert_eq!(cmd, "npm");
    assert!(args.contains(&"-g".to_string()));
    assert!(args.contains(&"typescript".to_string()));

    // Test frozen install
    let (cmd, args) = parse_ni(Agent::Npm, vec!["--frozen".to_string()], None);
    assert_eq!(cmd, "npm");
    assert_eq!(args, vec!["ci"]);

    // Test yarn install
    let (cmd, args) = parse_ni(Agent::Yarn, vec!["react".to_string()], None);
    assert_eq!(cmd, "yarn");
    assert_eq!(args, vec!["add", "react"]);

    // Test bun install
    let (cmd, args) = parse_ni(Agent::Bun, vec!["express".to_string()], None);
    assert_eq!(cmd, "bun");
    assert_eq!(args, vec!["add", "express"]);
}
