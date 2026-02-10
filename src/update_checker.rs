use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use console::style;

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CHECK_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60);
const GITHUB_API_URL: &str = "https://api.github.com/repos/wangsizhu0504/kn/releases/latest";

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct UpdateCache {
    last_check: u64,
    latest_version: Option<String>,
}

fn get_cache_path() -> Option<PathBuf> {
    dirs::cache_dir().map(|dir| dir.join("kn").join("update_cache.json"))
}

fn read_cache() -> Option<UpdateCache> {
    let cache_path = get_cache_path()?;
    let content = fs::read_to_string(cache_path).ok()?;
    serde_json::from_str(&content).ok()
}

fn write_cache(cache: &UpdateCache) {
    if let Some(cache_path) = get_cache_path() {
        if let Some(parent) = cache_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(content) = serde_json::to_string(cache) {
            let _ = fs::write(cache_path, content);
        }
    }
}

fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs()
}

fn should_check_update(cache: &Option<UpdateCache>) -> bool {
    match cache {
        Some(cache) => {
            let elapsed = get_current_timestamp().saturating_sub(cache.last_check);
            elapsed >= CHECK_INTERVAL.as_secs()
        }
        None => true,
    }
}

fn fetch_latest_version() -> Option<String> {
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(5))
        .build();

    let response = agent
        .get(GITHUB_API_URL)
        .set("Accept", "application/vnd.github.v3+json")
        .set("User-Agent", "kn-cli")
        .call()
        .ok()?;

    let body: serde_json::Value = response.into_json().ok()?;
    let tag_name = body["tag_name"].as_str()?;

    Some(tag_name.trim_start_matches('v').to_string())
}

fn compare_versions(current: &str, latest: &str) -> bool {
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

/// Build update notification
fn build_update_message(latest_version: &str) -> String {
    let mut msg = String::new();
    msg.push('\n');
    msg.push_str(&format!(
        "  {} {}  {} {} {}\n",
        style("▲").yellow().bold(),
        style("Update available").bold(),
        style(CURRENT_VERSION).dim(),
        style("→").bold(),
        style(latest_version).green().bold(),
    ));
    msg.push_str(&format!(
        "    Run {} to update\n",
        style("kn upgrade-self").cyan(),
    ));
    msg.push('\n');
    msg
}

pub fn check_for_updates() {
    std::thread::spawn(|| {
        let cache = read_cache();

        if !should_check_update(&cache) {
            if let Some(ref c) = cache {
                if let Some(ref latest) = c.latest_version {
                    if compare_versions(CURRENT_VERSION, latest) {
                        let msg = build_update_message(latest);
                        eprint!("{}", msg);
                    }
                }
            }
            return;
        }

        if let Some(latest_version) = fetch_latest_version() {
            let new_cache = UpdateCache {
                last_check: get_current_timestamp(),
                latest_version: Some(latest_version.clone()),
            };
            write_cache(&new_cache);

            if compare_versions(CURRENT_VERSION, &latest_version) {
                let msg = build_update_message(&latest_version);
                eprint!("{}", msg);
            }
        } else {
            let new_cache = UpdateCache {
                last_check: get_current_timestamp(),
                latest_version: cache.and_then(|c| c.latest_version),
            };
            write_cache(&new_cache);
        }
    });
}
