use std::fs;
use std::path::Path;
use crate::command_utils::parse_package_json;
use super::utils::{cleanup_test_dir, create_test_package_json};

#[test]
fn test_package_parsing() {
    let test_dir = Path::new("/tmp/kn_test_package_parsing");
    cleanup_test_dir(test_dir);
    fs::create_dir_all(test_dir).expect("Failed to create test directory");

    let package_json_content = r#"
{
"name": "test-package",
"version": "1.0.0",
"scripts": {
"start": "node index.js",
"test": "jest",
"build": "webpack --mode production"
},
"dependencies": {
"lodash": "^4.17.21",
"express": "^4.18.2"
},
"devDependencies": {
"jest": "^29.0.0"
}
}
"#;

    create_test_package_json(package_json_content, test_dir);

    let package = parse_package_json(&test_dir.join("package.json").to_string_lossy())
        .expect("Failed to parse package.json");

    assert_eq!(package.name, Some("test-package".to_string()));
    assert_eq!(package.version, Some("1.0.0".to_string()));

    let scripts = package.scripts.expect("Scripts should be present");
    assert_eq!(scripts.get("start"), Some(&"node index.js".to_string()));
    assert_eq!(scripts.get("test"), Some(&"jest".to_string()));
    assert_eq!(scripts.get("build"), Some(&"webpack --mode production".to_string()));
    assert_eq!(scripts.len(), 3);

    cleanup_test_dir(test_dir);
}

#[test]
fn test_package_parsing_no_scripts() {
    let test_dir = Path::new("/tmp/kn_test_no_scripts");
    cleanup_test_dir(test_dir);
    fs::create_dir_all(test_dir).expect("Failed to create test directory");

    let package_json_content = r#"
{
"name": "no-scripts-package",
"version": "1.0.0",
"dependencies": {
"lodash": "^4.17.21"
}
}
"#;

    create_test_package_json(package_json_content, test_dir);

    let package = parse_package_json(&test_dir.join("package.json").to_string_lossy())
        .expect("Failed to parse package.json");

    assert_eq!(package.name, Some("no-scripts-package".to_string()));
    assert_eq!(package.version, Some("1.0.0".to_string()));
    assert!(package.scripts.is_none() || package.scripts.as_ref().unwrap().is_empty());

    cleanup_test_dir(test_dir);
}
