use crate::agents::Agent;
use crate::parse::parse_nu;

#[test]
fn test_parse_nu() {
    // Test npm upgrade
    let (cmd, args) = parse_nu(Agent::Npm, vec!["lodash".to_string()], None);
    assert_eq!(cmd, "npm");
    assert_eq!(args, vec!["update", "lodash"]);

    // Test yarn upgrade
    let (cmd, args) = parse_nu(Agent::Yarn, vec!["react".to_string()], None);
    assert_eq!(cmd, "yarn");
    assert_eq!(args, vec!["upgrade", "react"]);

    // Test yarn berry upgrade
    let (cmd, args) = parse_nu(Agent::YarnBerry, vec!["react".to_string()], None);
    assert_eq!(cmd, "yarn");
    assert_eq!(args, vec!["up", "react"]);
}
