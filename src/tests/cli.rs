use std::process::Command;

#[test]
#[ignore] // 需要先构建二进制文件，通常在 CI 环境运行
fn test_cli_help_and_version() {
    // Test that CLI responds to help and version flags
    let bin_path = if cfg!(debug_assertions) {
        "./target/debug/kn"
    } else {
        "./target/release/kn"
    };

    // Check if binary exists, skip test if not
    if !std::path::Path::new(bin_path).exists() {
        eprintln!("Binary not found at {}, skipping test", bin_path);
        return;
    }

    let help_output = Command::new(bin_path)
        .arg("--help")
        .output()
        .expect("Failed to execute help command");

    assert!(help_output.status.success());
    let help_str = String::from_utf8_lossy(&help_output.stdout);
    assert!(help_str.contains("KN") || help_str.contains("kn"));

    let version_output = Command::new(bin_path)
        .arg("--version")
        .output()
        .expect("Failed to execute version command");

    assert!(version_output.status.success());
    let version_str = String::from_utf8_lossy(&version_output.stdout);
    assert!(version_str.contains("kn"));
}

#[test]
#[ignore] // 需要先构建二进制文件，通常在 CI 环境运行
fn test_empty_args_handling() {
    // Test that CLI handles empty arguments gracefully
    let bin_path = if cfg!(debug_assertions) {
        "./target/debug/kn"
    } else {
        "./target/release/kn"
    };

    // Check if binary exists, skip test if not
    if !std::path::Path::new(bin_path).exists() {
        eprintln!("Binary not found at {}, skipping test", bin_path);
        return;
    }

    let output = Command::new(bin_path)
        .output()
        .expect("Failed to execute empty command");

    // Should show help when no arguments provided
    let output_str = String::from_utf8_lossy(&output.stdout);
    let stderr_str = String::from_utf8_lossy(&output.stderr);

    // Check if help is shown in either stdout or stderr
    let help_shown = output_str.contains("Usage:")
        || output_str.contains("help")
        || stderr_str.contains("Usage:")
        || stderr_str.contains("help");

    assert!(
        help_shown,
        "Help should be shown when no arguments provided. stdout: {}, stderr: {}",
        output_str, stderr_str
    );
}
