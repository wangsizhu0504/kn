use crate::command_utils::detect_package_manager_fast;
use crate::display::StyledOutput;
use std::process;
use termimad::crossterm::style::Color;
use termimad::*;

pub fn handle(verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let spinner = StyledOutput::working("Analyzing environment...");
    let manager = detect_package_manager_fast()?;
    drop(spinner);

    // 1. 获取基础信息
    let version = get_package_manager_version(&manager).unwrap_or_else(|_| "unknown".to_string());
    let icon = get_manager_icon(&manager);

    // 2. 获取 Lock 文件信息
    let (lock_file, lock_status_text, is_valid_lock) = get_lock_file_info(&manager);

    // 3. 获取 Runtime 信息
    let node_version = get_node_version().unwrap_or_else(|_| "Not Found".to_string());
    let npm_version = get_npm_version().unwrap_or_else(|_| "Not Found".to_string());

    // 4. 构建 Markdown 内容
    let mut md = String::new();

    // -- 标题区域 --
    // 使用一级标题，Termimad 会自动加粗并变色
    md.push_str(&format!(
        "{} {} **v{}**\n",
        icon,
        manager.to_uppercase(),
        version
    ));

    // -- 核心状态表格 --
    // 这是一个 Markdown 表格，Termimad 会将其渲染成漂亮的终端表格
    md.push_str("|-|-|-|\n");
    md.push_str("|**Component**|**Status**|**Details**|\n");
    md.push_str("|-|-|-|\n");

    // Lock File 行
    let lock_icon = if is_valid_lock {
        "✅"
    } else if lock_file == "None" {
        "🔘"
    } else {
        "⚠️"
    };

    let lock_status_display = if is_valid_lock {
        "**Valid**".to_string()
    } else if lock_file == "None" {
        "Missing".to_string()
    } else {
        format!("**{}**", lock_status_text)
    };

    let lock_display = if lock_file == "None" {
        "No lockfile found"
    } else {
        &lock_file
    };

    md.push_str(&format!(
        "| Lock File | {} {} | `{}` |\n",
        lock_icon, lock_status_display, lock_display
    ));
    md.push_str(&format!(
        "| Runtime | 🐢 **Node.js** | `{}` \n",
        node_version
    ));

    // Runtime 行 (npm) - 即使不是当前 manager 也显示，了解环境
    md.push_str(&format!("| System | 📦 **npm** | `{}` |\n", npm_version));
    md.push_str(&format!("|-"));

    md.push_str("\n");

    // -- 详细环境信息 (Verbose) --
    if verbose {
        md.push_str("### 🔧 Environment Details\n\n");

        if let Ok(dir) = std::env::current_dir() {
            md.push_str(&format!("* **CWD**: `{}`\n", dir.display()));
        }

        if let Ok(output) = process::Command::new("npm")
            .args(["config", "get", "prefix"])
            .output()
        {
            let prefix = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !prefix.is_empty() {
                md.push_str(&format!("* **Global Prefix**: `{}`\n", prefix));
            }
        }

        if let Ok(cache) = std::env::var("npm_config_cache") {
            md.push_str(&format!("* **Cache**: `{}`\n", cache));
        }
        md.push_str("\n");
    } else {
        // 提示信息
        md.push_str("> *Run with* `kn info -v` *to see environment details*\n");
    }

    // 5. 配置皮肤 (Skin)
    let mut skin = MadSkin::default();

    // 设置标题颜色
    skin.set_headers_fg(Color::Cyan);
    // 设置加粗文本颜色 (高亮)
    skin.bold.set_fg(Color::Yellow);
    // 设置表格边框字符和颜色
    skin.table.compound_style.set_fg(Color::DarkGrey);
    // 让表格看起来更像卡片/面板
    skin.paragraph.set_fg(Color::White);
    // 使用圆角边框，提升高级感
    skin.table_border_chars = termimad::ROUNDED_TABLE_BORDER_CHARS;
    // 居中对齐表格
    skin.table.align = Alignment::Left;

    // 渲染输出
    skin.print_text(&md);

    Ok(())
}

fn get_lock_file_info(manager: &str) -> (String, String, bool) {
    use std::env;
    let lock_files = [
        ("pnpm-lock.yaml", "pnpm"),
        ("yarn.lock", "yarn"),
        ("package-lock.json", "npm"),
        ("bun.lockb", "bun"),
        ("bun.lock", "bun"),
    ];

    if let Ok(current_dir) = env::current_dir() {
        for (lock_file, lock_manager) in &lock_files {
            if current_dir.join(lock_file).exists() {
                if *lock_manager == manager {
                    return (lock_file.to_string(), "Valid".to_string(), true);
                } else {
                    return (
                        lock_file.to_string(),
                        format!("Expected {}", lock_manager),
                        false,
                    );
                }
            }
        }
    }

    ("None".to_string(), "Missing".to_string(), false)
}

fn get_node_version() -> Result<String, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("node")
        .arg("--version")
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn get_npm_version() -> Result<String, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("npm")
        .arg("--version")
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn get_package_manager_version(manager: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = process::Command::new(manager).arg("--version").output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn get_manager_icon(manager: &str) -> &str {
    match manager {
        "npm" => "📦",
        "pnpm" => "🚀",
        "yarn" => "🧶",
        "bun" => "🥟",
        _ => "📦",
    }
}

// 下面的旧函数可以安全删除了，因为主要逻辑已经迁移到 handle 中
// show_lock_file_info, show_node_info, show_verbose_info 已被移除

// start_of_old_implementation_placeholder
fn _unused() {}
