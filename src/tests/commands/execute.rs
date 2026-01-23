use crate::agents::Agent;
use crate::parse::parse_nlx;

#[test]
fn test_parse_nlx() {
    // Test npm execute
    let (cmd, args) = parse_nlx(Agent::Npm, vec!["cowsay".to_string(), "Hello".to_string()], None);
    assert_eq!(cmd, "npx");
    assert_eq!(args, vec!["cowsay", "Hello"]);

    // Test yarn execute
    let (cmd, args) = parse_nlx(Agent::Yarn, vec!["cowsay".to_string(), "Hello".to_string()], None);
    assert_eq!(cmd, "yarn");
    assert_eq!(args, vec!["dlx", "cowsay", "Hello"]);

    // Test bun execute
    let (cmd, args) = parse_nlx(Agent::Bun, vec!["cowsay".to_string(), "Hello".to_string()], None);
    assert_eq!(cmd, "bunx");
    assert_eq!(args, vec!["cowsay", "Hello"]);
}
