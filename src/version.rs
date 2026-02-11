use anyhow::{Context, Result};
use std::time::Duration;

pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GITHUB_API_URL: &str =
    "https://api.github.com/repos/wangsizhu0504/kn/releases/latest";
pub const GITHUB_RELEASE_URL: &str =
    "https://github.com/wangsizhu0504/kn/releases/latest/download";

/// Fetch the latest release version from GitHub API.
pub fn fetch_latest_version(timeout: Duration) -> Result<String> {
    let agent = ureq::AgentBuilder::new().timeout(timeout).build();

    let response = agent
        .get(GITHUB_API_URL)
        .set("Accept", "application/vnd.github.v3+json")
        .set("User-Agent", "kn-cli")
        .call()
        .context("Failed to fetch version info from GitHub")?;

    let body: serde_json::Value = response
        .into_json()
        .context("Failed to parse GitHub API response")?;

    let tag_name = body["tag_name"]
        .as_str()
        .context("No tag_name found in release")?;

    Ok(tag_name.trim_start_matches('v').to_string())
}

/// Compare two semver-style version strings.
/// Returns `true` if `latest` is newer than `current`.
pub fn compare_versions(current: &str, latest: &str) -> bool {
    let current_parts: Vec<u32> = current.split('.').filter_map(|s| s.parse().ok()).collect();
    let latest_parts: Vec<u32> = latest.split('.').filter_map(|s| s.parse().ok()).collect();

    for i in 0..std::cmp::max(current_parts.len(), latest_parts.len()) {
        let curr = current_parts.get(i).unwrap_or(&0);
        let lat = latest_parts.get(i).unwrap_or(&0);

        if lat > curr {
            return true;
        } else if lat < curr {
            return false;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_versions() {
        assert!(compare_versions("0.1.0", "0.2.0"));
        assert!(compare_versions("0.2.0", "1.0.0"));
        assert!(compare_versions("1.0.0", "1.0.1"));
        assert!(!compare_versions("0.2.0", "0.2.0"));
        assert!(!compare_versions("1.0.0", "0.9.0"));
        assert!(!compare_versions("0.2.0", "0.1.9"));
    }
}
