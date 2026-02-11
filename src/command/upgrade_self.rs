use anyhow::{bail, Result};
use console::style;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::io::Read;
use std::process::{Command, Stdio};
use std::time::Duration;

use crate::display::StyledOutput;
use crate::version::{compare_versions, fetch_latest_version, CURRENT_VERSION, GITHUB_RELEASE_URL};

const UPGRADE_TIMEOUT: Duration = Duration::from_secs(10);

pub fn handle() -> Result<()> {
    let spinner = StyledOutput::working("Checking for updates...");

    let latest_version = match fetch_latest_version(UPGRADE_TIMEOUT) {
        Ok(v) => {
            drop(spinner);
            v
        }
        Err(e) => {
            drop(spinner);
            bail!("Failed to check for updates: {}", e);
        }
    };

    // Use semantic version comparison to avoid downgrading
    if !compare_versions(CURRENT_VERSION, &latest_version) {
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
    let checksum_url = format!("{}/{}.sha256", GITHUB_RELEASE_URL, archive_name);

    let temp_dir = env::temp_dir().join("kn-upgrade");
    // Clean up any residual files from a previous failed upgrade
    if temp_dir.exists() {
        let _ = fs::remove_dir_all(&temp_dir);
    }
    fs::create_dir_all(&temp_dir)?;
    let archive_path = temp_dir.join(&archive_name);
    let checksum_path = temp_dir.join(format!("{}.sha256", archive_name));

    // Step 1: Download archive and checksum file
    let spinner = StyledOutput::working(&format!("Downloading {}...", archive_name));

    let status = Command::new("curl")
        .args(["-fsSL", "-o", archive_path.to_str().unwrap(), &download_url])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    if !status.success() {
        drop(spinner);
        let _ = fs::remove_dir_all(&temp_dir);
        StyledOutput::error("Download failed");
        bail!("Failed to download the release (HTTP error or network issue)");
    }

    let checksum_status = Command::new("curl")
        .args([
            "-fsSL",
            "-o",
            checksum_path.to_str().unwrap(),
            &checksum_url,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    drop(spinner);

    if !checksum_status.success() {
        let _ = fs::remove_dir_all(&temp_dir);
        StyledOutput::error("Checksum download failed");
        bail!("Failed to download the checksum file");
    }

    StyledOutput::tree_item(&format!("{} Downloaded", style("✔").green()), false);

    // Step 2: Verify SHA-256 checksum
    let spinner = StyledOutput::working("Verifying checksum...");

    let expected_checksum = fs::read_to_string(&checksum_path)?.trim().to_lowercase();
    // The .sha256 file may contain "hash  filename" or just "hash"
    let expected_hash = expected_checksum
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_string();

    let mut file = fs::File::open(&archive_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    let actual_hash = format!("{:x}", hasher.finalize());

    drop(spinner);

    if actual_hash != expected_hash {
        let _ = fs::remove_dir_all(&temp_dir);
        StyledOutput::error("Checksum verification failed");
        bail!(
            "SHA-256 mismatch:\n  expected: {}\n  actual:   {}",
            expected_hash,
            actual_hash
        );
    }

    StyledOutput::tree_item(&format!("{} Verified", style("✔").green()), false);

    // Step 3: Extract
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
        let _ = fs::remove_dir_all(&temp_dir);
        StyledOutput::error("Extraction failed");
        bail!("Failed to extract the archive");
    }
    StyledOutput::tree_item(&format!("{} Extracted", style("✔").green()), false);

    // Step 4: Install with backup & rollback
    let spinner = StyledOutput::working("Installing...");

    let new_binary = if os == "windows" {
        extract_dir.join("kn.exe")
    } else {
        extract_dir.join("kn")
    };

    if !new_binary.exists() {
        drop(spinner);
        let _ = fs::remove_dir_all(&temp_dir);
        StyledOutput::error("Could not find kn binary in archive");
        bail!("Binary not found");
    }

    #[cfg(unix)]
    {
        let perms = fs::metadata(&current_exe)?.permissions();
        fs::set_permissions(&new_binary, perms)?;
    }

    // Create a backup of the current binary for rollback
    let backup_path = temp_dir.join("kn.backup");
    fs::copy(&current_exe, &backup_path)?;

    let install_result = (|| -> Result<()> {
        if fs::rename(&new_binary, &current_exe).is_err() {
            fs::copy(&new_binary, &current_exe)?;
        }
        Ok(())
    })();

    if let Err(e) = install_result {
        // Rollback: restore the backup
        let _ = fs::copy(&backup_path, &current_exe);
        drop(spinner);
        let _ = fs::remove_dir_all(&temp_dir);
        StyledOutput::error("Installation failed, rolled back to previous version");
        bail!("Failed to install new binary: {}", e);
    }

    let _ = fs::remove_dir_all(&temp_dir);

    drop(spinner);

    StyledOutput::tree_item(&format!("{} Installed", style("✔").green()), true);

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
