use std::env;
use std::fs;
use std::process::{Command, Stdio};

const GITHUB_RELEASE_URL: &str = "https://github.com/wangsizhu0504/kn/releases/latest/download";
const GITHUB_API_URL: &str = "https://api.github.com/repos/wangsizhu0504/kn/releases/latest";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

// 获取最新版本号
fn fetch_latest_version() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("curl")
        .args([
            "-s",
            "-m", "5",
            "-H", "Accept: application/vnd.github.v3+json",
            GITHUB_API_URL,
        ])
        .output()?;

    if !output.status.success() {
        return Err("Failed to fetch version info".into());
    }

    let response: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let tag_name = response["tag_name"]
        .as_str()
        .ok_or("No tag_name found")?;

    Ok(tag_name.trim_start_matches('v').to_string())
}

pub fn handle() -> Result<(), Box<dyn std::error::Error>> {
    println!();

    // 获取最新版本
    print!("Checking for updates... ");
    std::io::Write::flush(&mut std::io::stdout())?;

    let latest_version = match fetch_latest_version() {
        Ok(v) => {
            println!("\x1b[32m✓\x1b[0m");
            v
        }
        Err(e) => {
            println!("\x1b[31m✗\x1b[0m");
            eprintln!("\x1b[31mError:\x1b[0m Failed to check for updates: {}", e);
            std::process::exit(1);
        }
    };

    // 检查是否需要更新
    if CURRENT_VERSION == latest_version {
        println!();
        println!("\x1b[32m✓\x1b[0m Already using the latest version \x1b[36m{}\x1b[0m", CURRENT_VERSION);
        println!();
        return Ok(());
    }

    // 显示版本信息
    println!();
    println!("Upgrading kn \x1b[90m{}\x1b[0m → \x1b[32m{}\x1b[0m", CURRENT_VERSION, latest_version);
    println!();

    // 获取当前可执行文件路径
    let current_exe = env::current_exe()?;

    // 检测操作系统和架构
    let (os, arch) = detect_platform()?;
    let archive_name = format!("kn-{}-{}.{}", os, arch, if os == "windows" { "zip" } else { "tar.gz" });
    let download_url = format!("{}/{}", GITHUB_RELEASE_URL, archive_name);

    // 创建临时目录
    let temp_dir = env::temp_dir().join("kn-upgrade");
    fs::create_dir_all(&temp_dir)?;
    let archive_path = temp_dir.join(&archive_name);

    // 下载文件
    print!("Downloading {}... ", archive_name);
    std::io::Write::flush(&mut std::io::stdout())?;

    let status = Command::new("curl")
        .args([
            "-#",
            "-L",
            "-o", archive_path.to_str().unwrap(),
            &download_url,
        ])
        .status()?;

    if !status.success() {
        println!("\x1b[31m✗\x1b[0m");
        eprintln!("\x1b[31mError:\x1b[0m Failed to download the release");
        std::process::exit(1);
    }

    // 解压文件
    print!("Extracting... ");
    std::io::Write::flush(&mut std::io::stdout())?;
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

    if !extract_status.success() {
        println!("\x1b[31m✗\x1b[0m");
        eprintln!("\x1b[31mError:\x1b[0m Failed to extract the archive");
        std::process::exit(1);
    }

    println!("\x1b[32m✓\x1b[0m");

    // 安装新版本
    print!("Installing... ");
    std::io::Write::flush(&mut std::io::stdout())?;

    // 查找解压后的可执行文件
    let new_binary = if os == "windows" {
        extract_dir.join("kn.exe")
    } else {
        extract_dir.join("kn")
    };

    if !new_binary.exists() {
        println!("\x1b[31m✗\x1b[0m");
        eprintln!("\x1b[31mError:\x1b[0m Could not find kn binary in the archive");
        std::process::exit(1);
    }

    #[cfg(unix)]
    {
        // 在 Unix 系统上，保留权限
        let perms = fs::metadata(&current_exe)?.permissions();
        fs::set_permissions(&new_binary, perms)?;
    }

    // 替换文件
    if let Err(_e) = fs::rename(&new_binary, &current_exe) {
        // 如果 rename 失败（可能跨文件系统），尝试 copy + remove
        fs::copy(&new_binary, &current_exe)?;
        let _ = fs::remove_file(&new_binary);
    }

    // 清理临时文件
    let _ = fs::remove_dir_all(&temp_dir);

    println!("\x1b[32m✓\x1b[0m");

    // 成功消息
    println!();
    println!("\x1b[32m✓\x1b[0m Successfully upgraded kn to \x1b[36mv{}\x1b[0m", latest_version);
    println!();

    Ok(())
}

fn detect_platform() -> Result<(String, String), Box<dyn std::error::Error>> {
    let os = if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        return Err("Unsupported operating system".into());
    };

    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        return Err("Unsupported architecture".into());
    };

    Ok((os.to_string(), arch.to_string()))
}
