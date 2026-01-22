use crate::display::StyledOutput;
use std::fs;
use std::path::Path;
use comfy_table::{Table, Cell, Color, Attribute, ContentArrangement, presets::UTF8_FULL};

pub fn handle() -> Result<(), Box<dyn std::error::Error>> {
    StyledOutput::header("Dependency Size Analysis");

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
                                let scoped_name = format!("{}/{}", name, scoped_entry.file_name().to_string_lossy());
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

    // Display top 20 packages in table format
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("No.").add_attribute(Attribute::Bold).fg(Color::Cyan),
            Cell::new("Package Name").add_attribute(Attribute::Bold).fg(Color::Cyan),
            Cell::new("Size").add_attribute(Attribute::Bold).fg(Color::Cyan),
            Cell::new("%").add_attribute(Attribute::Bold).fg(Color::Cyan),
            Cell::new("Size Bar").add_attribute(Attribute::Bold).fg(Color::Cyan),
        ]);

    let display_count = packages.len().min(20);
    for (i, (name, size)) in packages.iter().take(display_count).enumerate() {
        let size_str = format_size(*size);
        let percentage = (*size as f64 / total_size as f64) * 100.0;

        let bar_width = ((percentage / 2.0) as usize).min(40);
        let bar = "█".repeat(bar_width);

        // 根据大小设置颜色
        let size_color = if *size > 5 * 1024 * 1024 {
            Color::Red
        } else if *size > 1024 * 1024 {
            Color::Yellow
        } else {
            Color::Green
        };

        table.add_row(vec![
            Cell::new((i + 1).to_string()),
            Cell::new(name),
            Cell::new(size_str).fg(size_color),
            Cell::new(format!("{:.1}%", percentage)),
            Cell::new(bar).fg(Color::Blue),
        ]);
    }

    println!("\n{}", table);

    // Summary
    println!("\n📊 Summary");
    let mut summary_table = Table::new();
    summary_table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    let large_packages: Vec<_> = packages.iter().filter(|(_, size)| *size > 1024 * 1024).collect();
    let avg_size = total_size / packages.len() as u64;

    summary_table.add_row(vec![
        Cell::new("Total packages").add_attribute(Attribute::Bold),
        Cell::new(packages.len().to_string()).fg(Color::Cyan),
    ]);
    summary_table.add_row(vec![
        Cell::new("Total size").add_attribute(Attribute::Bold),
        Cell::new(format_size(total_size)).fg(Color::Yellow),
    ]);
    summary_table.add_row(vec![
        Cell::new("Packages > 1MB").add_attribute(Attribute::Bold),
        Cell::new(large_packages.len().to_string()).fg(if large_packages.len() > 10 { Color::Red } else { Color::Green }),
    ]);
    summary_table.add_row(vec![
        Cell::new("Average package size").add_attribute(Attribute::Bold),
        Cell::new(format_size(avg_size)).fg(Color::Magenta),
    ]);

    println!("{}", summary_table);

    // Show size breakdown
    if !large_packages.is_empty() {
        println!();
        StyledOutput::warning(&format!("⚠  {} package(s) are larger than 1 MB", large_packages.len()));
        StyledOutput::info("💡 Consider using lighter alternatives for large packages");
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

