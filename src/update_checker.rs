use std::fs;
use std::path::PathBuf;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use console::style;

use crate::version::{compare_versions, fetch_latest_version, CURRENT_VERSION};

const CHECK_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60);
const CHECK_TIMEOUT: Duration = Duration::from_secs(5);

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

/// Build update notification message.
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

/// Check for updates in a background thread.
///
/// Returns a `JoinHandle` that resolves to an optional update message string.
/// The caller is responsible for joining the thread and printing the message
/// after the main command has finished, avoiding output interleaving.
pub fn check_for_updates() -> JoinHandle<Option<String>> {
    std::thread::spawn(|| {
        let cache = read_cache();

        // Cache is still fresh — just return the cached notification if applicable.
        if !should_check_update(&cache) {
            if let Some(ref c) = cache {
                if let Some(ref latest) = c.latest_version {
                    if compare_versions(CURRENT_VERSION, latest) {
                        return Some(build_update_message(latest));
                    }
                }
            }
            return None;
        }

        // Cache expired — try to fetch latest version from GitHub.
        match fetch_latest_version(CHECK_TIMEOUT) {
            Ok(latest_version) => {
                let new_cache = UpdateCache {
                    last_check: get_current_timestamp(),
                    latest_version: Some(latest_version.clone()),
                };
                write_cache(&new_cache);

                if compare_versions(CURRENT_VERSION, &latest_version) {
                    Some(build_update_message(&latest_version))
                } else {
                    None
                }
            }
            Err(_) => {
                // Network failed — do NOT update last_check so we retry next time.
                // But still show cached update hint if available.
                if let Some(ref c) = cache {
                    if let Some(ref latest) = c.latest_version {
                        if compare_versions(CURRENT_VERSION, latest) {
                            return Some(build_update_message(latest));
                        }
                    }
                }
                None
            }
        }
    })
}
