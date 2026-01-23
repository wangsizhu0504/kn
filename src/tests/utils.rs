use std::fs;
use std::path::Path;

pub fn create_test_package_json(content: &str, dir: &Path) {
    let package_json_path = dir.join("package.json");
    fs::write(package_json_path, content).expect("Failed to write package.json");
}

pub fn create_test_lock_file(lock_file: &str, dir: &Path) {
    let lock_path = dir.join(lock_file);
    fs::write(lock_path, "test lock content").expect("Failed to write lock file");
}

pub fn cleanup_test_dir(dir: &Path) {
    if dir.exists() {
        fs::remove_dir_all(dir).expect("Failed to cleanup test directory");
    }
}
