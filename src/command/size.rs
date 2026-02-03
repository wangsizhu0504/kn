use crate::display::StyledOutput;
use std::fs;
use std::path::Path;
use termimad::crossterm::style::Color;
use termimad::{Alignment, MadSkin};

pub fn handle() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new("node_modules").exists() {
        StyledOutput::warning("node_modules not found. Run 'kn install' first.");
        return Ok(());
    }

    StyledOutput::info("Analyzing package sizes...");

    let mut packages = Vec::new();

    // Read node_modules
    if let Ok(entries) = fs::read_dir("node_modules") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();

                // Skip hidden folders
                if name.starts_with('.') {
                    continue;
                }

                // Handle scoped packages
                if name.starts_with('@') {
                    if let Ok(scoped_entries) = fs::read_dir(&path) {
                        for scoped_entry in scoped_entries.flatten() {
                            let scoped_path = scoped_entry.path();
                            if scoped_path.is_dir() {
                                let scoped_name = format!(
                                    "{}/{}",
                                    name,
                                    scoped_entry.file_name().to_string_lossy()
                                );
                                if let Ok(size) = calculate_dir_size(&scoped_path) {
                                    packages.push((scoped_name, size));
                                }
                            }
                        }
                    }
                } else if let Ok(size) = calculate_dir_size(&path) {
                    packages.push((name, size));
                }
            }
        }
    }

    if packages.is_empty() {
        StyledOutput::info("No packages found in node_modules");
        return Ok(());
    }

    // Sort by size (descending)
    packages.sort_by(|a, b| b.1.cmp(&a.1));

    // Calculate total size
    let total_size: u64 = packages.iter().map(|(_, size)| size).sum();

    let mut md = String::new();

    // Display top 20 packages in table format
    md.push_str("|-|-|-|-|-|\n");
    md.push_str("|**No.**|**Package Name**|**Size**|**%**|**Status**|\n");
    md.push_str("|-|-|-|-|-|\n");

    let display_count = packages.len().min(20);
    for (i, (name, size)) in packages.iter().take(display_count).enumerate() {
        let size_str = format_size(*size);
        let percentage = (*size as f64 / total_size as f64) * 100.0;

        // Visual indicator using emojis
        let status = if *size > 5 * 1024 * 1024 {
            "🔴 Large"
        } else if *size > 1024 * 1024 {
            "🟡 Medium"
        } else {
            "🟢 Small"
        };

        md.push_str(&format!(
            "| {} | **{}** | `{}` | {:.1}% | {} |\n",
            i + 1,
            name,
            size_str,
            percentage,
            status
        ));
    }

    md.push_str("|-|-|-|-|-|\n");

    // Summary Section
    md.push_str("### 📈 Summary\n\n");

    let large_packages_count = packages
        .iter()
        .filter(|(_, size)| *size > 1024 * 1024)
        .count();

    let avg_size = total_size / packages.len() as u64;

    md.push_str("|-|-|\n");
    md.push_str("|**Metric**|**Value**|\n");
    md.push_str("|-|-|\n");
    md.push_str(&format!("| Total packages | **{}** |\n", packages.len()));
    md.push_str(&format!("| Total size | `{}` |\n", format_size(total_size)));
    md.push_str(&format!(
        "| Packages > 1MB | **{}** |\n",
        large_packages_count
    ));
    md.push_str(&format!("| Average size | `{}` |\n", format_size(avg_size)));
    md.push_str("|-|-|\n");

    let mut skin = MadSkin::default();
    skin.set_headers_fg(Color::Cyan);
    skin.bold.set_fg(Color::Yellow);
    skin.table.compound_style.set_fg(Color::DarkGrey);
    skin.paragraph.set_fg(Color::White);
    skin.table_border_chars = termimad::ROUNDED_TABLE_BORDER_CHARS;
    skin.table.align = Alignment::Left;

    skin.print_text(&md);

    // Show size breakdown warning if needed
    if large_packages_count > 0 {
        println!();
        StyledOutput::warning(&format!(
            "{} package(s) are larger than 1 MB",
            large_packages_count
        ));
        StyledOutput::info("Consider using lighter alternatives for large packages");
    }

    Ok(())
}

fn calculate_dir_size(path: &Path) -> std::io::Result<u64> {
    let mut size = 0;

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                size += entry.metadata()?.len();
            } else if path.is_dir() {
                size += calculate_dir_size(&path)?;
            }
        }
    } else {
        size = fs::metadata(path)?.len();
    }

    Ok(size)
}

fn format_size(bytes: u64) -> String {
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
