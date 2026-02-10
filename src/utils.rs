use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::command_utils::Package;

/// Levenshtein distance for fuzzy matching (shared implementation)
pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(matrix[i][j + 1] + 1, matrix[i + 1][j] + 1),
                matrix[i][j] + cost,
            );
        }
    }

    matrix[len1][len2]
}

/// Find a file by walking up the directory tree, returns the file path
pub fn find_up(filename: &str, cwd: &Path) -> Option<PathBuf> {
    let mut current = cwd.to_path_buf();
    loop {
        let file_path = current.join(filename);
        if file_path.is_file() {
            return Some(file_path);
        }
        if !current.pop() {
            break;
        }
    }
    None
}

/// Find package.json in the directory tree and parse it
pub fn find_and_parse_package_json(cwd: &Path) -> Result<(PathBuf, Package)> {
    let package_json_path = find_up("package.json", cwd)
        .context("No package.json found in current directory or any parent")?;

    let contents = fs::read_to_string(&package_json_path)
        .with_context(|| format!("Failed to read {}", package_json_path.display()))?;

    let package: Package = serde_json::from_str(&contents)
        .with_context(|| format!("Failed to parse {}", package_json_path.display()))?;

    Ok((package_json_path, package))
}

/// Format a file size in human-readable form
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Calculate directory size recursively
pub fn dir_size(path: &Path) -> Result<u64> {
    let mut size = 0u64;
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if metadata.is_file() {
                size += metadata.len();
            } else if metadata.is_dir() {
                size += dir_size(&entry.path())?;
            }
        }
    }
    Ok(size)
}

/// Get package.json (legacy helper, prefer find_and_parse_package_json)
#[allow(dead_code)]
pub fn get_package_json(path: &str) -> Package {
    let path = Path::new(path);
    if path.exists() && path.is_file() {
        if let Ok(contents) = fs::read_to_string(path) {
            return serde_json::from_str::<Package>(&contents).unwrap_or_default();
        }
    }
    Package::default()
}
