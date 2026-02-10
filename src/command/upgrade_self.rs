use anyhow::{bail, Context, Result};
use console::style;
use std::env;
use std::fs;
use std::process::{Command, Stdio};
use std::time::Duration;

use crate::display::StyledOutput;

const GITHUB_RELEASE_URL: &str = "https://github.com/wangsizhu0504/kn/releases/latest/download";
const GITHUB_API_URL: &str = "https://api.github.com/repos/wangsizhu0504/kn/releases/latest";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

fn fetch_latest_version() -> Result<String> {
    let agent = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(10))
        .build();

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

pub fn handle() -> Result<()> {
    let spinner = StyledOutput::working("Checking for updates...");

    let latest_version = match fetch_latest_version() {
        Ok(v) => {
            drop(spinner);
            v
        }
        Err(e) => {
            drop(spinner);
            bail!("Failed to check for updates: {}", e);
        }
    };

    if CURRENT_VERSION == latest_version {
        // Already up to date — show card
        println!();
        StyledOutput::success(&format!(
            "Already on latest version {}",
            style(format!("v{}", CURRENT_VERSION)).cyan(),
        ));
        println!();
        return Ok(());
    }

    // ── Upgrade flow ──
    let header = format!(
        "Upgrade  {} {} {}",
        style(format!("v{}", CURRENT_VERSION)).dim(),
        style("→").bold(),
        style(format!("v{}", latest_version)).green().bold(),
    );

    println!();
    println!("  {}", style(&header).bold());
    StyledOutput::separator();
    println!();

    let (os, arch) = detect_platform()?;
    let current_exe = env::current_exe()?;
    let archive_name = format!(
        "kn-{}-{}.{}",
        os,
        arch,
        if os == "windows" { "zip" } else { "tar.gz" }
    );
    let download_url = format!("{}/{}", GITHUB_RELEASE_URL, archive_name);

    let temp_dir = env::temp_dir().join("kn-upgrade");
    fs::create_dir_all(&temp_dir)?;
    let archive_path = temp_dir.join(&archive_name);

    // Step 1: Download
    let spinner = StyledOutput::working(&format!("Downloading {}...", archive_name));

    let status = Command::new("curl")
        .args([
            "-s", "-L", "-o",
            archive_path.to_str().unwrap(),
            &download_url,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    drop(spinner);

    if !status.success() {
        StyledOutput::error("Download failed");
        bail!("Failed to download the release");
    }
    StyledOutput::tree_item(
        &format!("{} Downloaded", style("✔").green()),
        false,
    );

    // Step 2: Extract
    let spinner = StyledOutput::working("Extracting...");
    let extract_dir = temp_dir.join("extracted");
    fs::create_dir_all(&extract_dir)?;

    let extract_status = if os == "windows" {
        Command::new("powershell")
            .args([
                "-Command",
                &format!(
                    "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                    archive_path.display(),
                    extract_dir.display()
                ),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?
    } else {
        Command::new("tar")
            .args([
                "-xzf",
                archive_path.to_str().unwrap(),
                "-C",
                extract_dir.to_str().unwrap(),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?
    };

    drop(spinner);

    if !extract_status.success() {
        StyledOutput::error("Extraction failed");
        bail!("Failed to extract the archive");
    }
    StyledOutput::tree_item(
        &format!("{} Extracted", style("✔").green()),
        false,
    );

    // Step 3: Install
    let spinner = StyledOutput::working("Installing...");

    let new_binary = if os == "windows" {
        extract_dir.join("kn.exe")
    } else {
        extract_dir.join("kn")
    };

    if !new_binary.exists() {
        drop(spinner);
        StyledOutput::error("Could not find kn binary in archive");
        bail!("Binary not found");
    }

    #[cfg(unix)]
    {
        let perms = fs::metadata(&current_exe)?.permissions();
        fs::set_permissions(&new_binary, perms)?;
    }

    if let Err(_e) = fs::rename(&new_binary, &current_exe) {
        fs::copy(&new_binary, &current_exe)?;
        let _ = fs::remove_file(&new_binary);
    }

    let _ = fs::remove_dir_all(&temp_dir);

    drop(spinner);

    StyledOutput::tree_item(
        &format!("{} Installed", style("✔").green()),
        true,
    );

    // ── Success card ──
    println!();
    StyledOutput::success(&format!(
        "Upgraded to {}",
        style(format!("v{}", latest_version)).cyan().bold(),
    ));
    println!();

    Ok(())
}

fn detect_platform() -> Result<(String, String)> {
    let os = if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        bail!("Unsupported operating system");
    };

    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        bail!("Unsupported architecture");
    };

    Ok((os.to_string(), arch.to_string()))
}
