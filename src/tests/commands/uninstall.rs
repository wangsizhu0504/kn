use crate::agents::Agent;
use crate::parse::parse_nun;

#[test]
fn test_parse_nun() {
    // Test basic uninstall
    let (cmd, args) = parse_nun(Agent::Npm, vec!["lodash".to_string()], None);
    assert_eq!(cmd, "npm");
    assert_eq!(args, vec!["uninstall", "lodash"]);

    // Test yarn uninstall
    let (cmd, args) = parse_nun(Agent::Yarn, vec!["react".to_string()], None);
    assert_eq!(cmd, "yarn");
    assert_eq!(args, vec!["remove", "react"]);

    // Test global uninstall
    let (cmd, args) = parse_nun(
        Agent::Pnpm,
        vec!["-g".to_string(), "typescript".to_string()],
        None,
    );
    assert_eq!(cmd, "pnpm");
    assert!(args.contains(&"-g".to_string()));
    assert!(args.contains(&"typescript".to_string()));
}
