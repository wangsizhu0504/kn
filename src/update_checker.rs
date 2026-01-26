use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CHECK_INTERVAL: Duration = Duration::from_secs(24 * 60 * 60); // 24 hours
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

fn write_cache(cache: &UpdateCache) -> Result<(), Box<dyn std::error::Error>> {
    let cache_path = get_cache_path().ok_or("Failed to get cache path")?;

    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = serde_json::to_string(cache)?;
    fs::write(cache_path, content)?;
    Ok(())
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
    // 仅获取 GitHub Release 页面的版本信息，不下载源码
    // 使用 GitHub API 获取最新 release 的 tag_name（只有几 KB 的 JSON 数据）
    let output = std::process::Command::new("curl")
        .args([
            "-s",                                           // 静默模式
            "-m", "5",                                      // 最大 5 秒超时
            "-H", "Accept: application/vnd.github.v3+json", // GitHub API v3
            GITHUB_API_URL,                                 // 只请求 /releases/latest 端点
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    // 解析 JSON 响应，只提取 tag_name 字段
    let response: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
    let tag_name = response["tag_name"].as_str()?;

    // 移除版本号前缀 'v' (例如 "v0.1.0" -> "0.1.0")
    Some(tag_name.trim_start_matches('v').to_string())
}

fn compare_versions(current: &str, latest: &str) -> bool {
    // Simple version comparison (works for semver)
    let current_parts: Vec<u32> = current
        .split('.')
        .filter_map(|s| s.parse().ok())
        .collect();
    let latest_parts: Vec<u32> = latest
        .split('.')
        .filter_map(|s| s.parse().ok())
        .collect();

    for i in 0..std::cmp::max(current_parts.len(), latest_parts.len()) {
        let curr = current_parts.get(i).unwrap_or(&0);
        let lat = latest_parts.get(i).unwrap_or(&0);

        if lat > curr {
            return true; // Latest is newer
        } else if lat < curr {
            return false; // Current is newer
        }
    }

    false // Versions are equal
}

pub fn check_for_updates() {
    // Run in a separate thread to not block the main program
    // This is non-blocking and won't affect performance
    std::thread::spawn(|| {
        let cache = read_cache();

        // Check if we should query GitHub (respects 24-hour interval)
        if !should_check_update(&cache) {
            // Use cached result if available and recent
            if let Some(cache) = cache {
                if let Some(latest) = cache.latest_version {
                    if compare_versions(CURRENT_VERSION, &latest) {
                        show_update_message(&latest);
                    }
                }
            }
            return;
        }

        // Fetch latest release version from GitHub API (lightweight, no source download)
        if let Some(latest_version) = fetch_latest_version() {
            // Cache the result
            let new_cache = UpdateCache {
                last_check: get_current_timestamp(),
                latest_version: Some(latest_version.clone()),
            };

            let _ = write_cache(&new_cache);

            // Show update message if newer version is available
            if compare_versions(CURRENT_VERSION, &latest_version) {
                show_update_message(&latest_version);
            }
        } else {
            // Even if fetch fails, update the last_check time to avoid checking too frequently
            let new_cache = UpdateCache {
                last_check: get_current_timestamp(),
                latest_version: cache.and_then(|c| c.latest_version),
            };
            let _ = write_cache(&new_cache);
        }
    });
}

fn show_update_message(latest_version: &str) {
    eprintln!();
    eprintln!("\x1b[33m╭─────────────────────────────────────────────────────────╮\x1b[0m");
    eprintln!("\x1b[33m│                                                         │\x1b[0m");
    eprintln!("\x1b[33m│   \x1b[1m📦 Update available!\x1b[0m                              │\x1b[0m");
    eprintln!("\x1b[33m│                                                         │\x1b[0m");
    eprintln!("\x1b[33m│   Current: \x1b[90m{:<10}\x1b[0m → Latest: \x1b[32m{:<10}\x1b[0m       │\x1b[0m", CURRENT_VERSION, latest_version);
    eprintln!("\x1b[33m│                                                         │\x1b[0m");
    eprintln!("\x1b[33m│   Run \x1b[36mkn upgrade-self\x1b[0m to update                     │\x1b[0m");
    eprintln!("\x1b[33m│                                                         │\x1b[0m");
    eprintln!("\x1b[33m╰─────────────────────────────────────────────────────────╯\x1b[0m");
    eprintln!();
}
