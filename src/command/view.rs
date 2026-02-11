use anyhow::{Context, Result};
use console::style;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::display::StyledOutput;

#[derive(Debug, Deserialize, Serialize)]
struct PackageInfo {
    name: String,
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies", skip_serializing_if = "Option::is_none")]
    dev_dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "peerDependencies", skip_serializing_if = "Option::is_none")]
    peer_dependencies: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<Repository>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bugs: Option<Bugs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dist: Option<Dist>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintainers: Option<Vec<Maintainer>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Author {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Repository {
    #[serde(rename = "type")]
    repo_type: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Bugs {
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Dist {
    tarball: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    shasum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    integrity: Option<String>,
    #[serde(rename = "unpackedSize", skip_serializing_if = "Option::is_none")]
    unpacked_size: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Maintainer {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RegistryResponse {
    name: String,
    #[serde(rename = "dist-tags")]
    dist_tags: HashMap<String, String>,
    versions: HashMap<String, PackageInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<Author>,
    #[serde(skip_serializing_if = "Option::is_none")]
    repository: Option<Repository>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bugs: Option<Bugs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maintainers: Option<Vec<Maintainer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<HashMap<String, String>>,
}

pub fn handle(package: String, version: Option<String>) -> Result<()> {
    tracing::info!("Viewing package: {} (version: {:?})", package, version);

    let spinner = StyledOutput::working("Fetching package info...");

    let registry = get_registry_url();
    let url = format!("{}/{}", registry, package);

    tracing::debug!("Fetching from registry: {}", url);

    let client = ureq::AgentBuilder::new()
        .timeout(std::time::Duration::from_secs(30))
        .build();

    let response = client
        .get(&url)
        .set("Accept", "application/json")
        .call()
        .with_context(|| format!("Failed to fetch package info for '{}'", package))?;

    let registry_data: RegistryResponse = response
        .into_json()
        .context("Failed to parse registry response")?;

    drop(spinner);

    let target_version = version
        .or_else(|| registry_data.dist_tags.get("latest").cloned())
        .context("No version specified and no latest tag found")?;

    let package_info = registry_data
        .versions
        .get(&target_version)
        .ok_or_else(|| anyhow::anyhow!("Version {} not found", target_version))?;

    print_package_info(&registry_data, package_info);

    Ok(())
}

fn get_registry_url() -> String {
    if let Ok(registry) = std::env::var("NPM_CONFIG_REGISTRY") {
        return registry.trim_end_matches('/').to_string();
    }

    if let Ok(home) = std::env::var("HOME") {
        let npmrc_path = std::path::Path::new(&home).join(".npmrc");
        if let Ok(content) = std::fs::read_to_string(npmrc_path) {
            for line in content.lines() {
                if let Some(registry) = line.strip_prefix("registry=") {
                    return registry.trim().trim_end_matches('/').to_string();
                }
            }
        }
    }

    "https://registry.npmjs.org".to_string()
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

fn print_package_info(registry_data: &RegistryResponse, info: &PackageInfo) {
    let deps_count = info.dependencies.as_ref().map(|d| d.len()).unwrap_or(0);
    let versions_count = registry_data.versions.len();
    let license = info.license.as_deref().unwrap_or("UNLICENSED");

    // ── Header card ──
    let title = format!(
        "{}@{}",
        style(&info.name).bold().cyan(),
        style(&info.version).bold(),
    );

    let meta = format!(
        "{}  {}  {}",
        style(license).dim(),
        style(format!("{} deps", deps_count)).dim(),
        style(format!("{} versions", versions_count)).dim(),
    );

    let mut header_lines = vec![title, meta];

    // Description
    if let Some(desc) = info
        .description
        .as_ref()
        .or(registry_data.description.as_ref())
    {
        if !desc.is_empty() {
            header_lines.push(String::new());
            header_lines.push(format!("{}", style(desc).italic()));
        }
    }

    println!();
    for (i, line) in header_lines.iter().enumerate() {
        if i == 0 {
            StyledOutput::titled(line);
        } else if line.is_empty() {
            println!();
        } else {
            StyledOutput::body(line);
        }
    }

    // ── Links section ──
    let homepage = info.homepage.as_ref().or(registry_data.homepage.as_ref());
    let repo = info
        .repository
        .as_ref()
        .or(registry_data.repository.as_ref());
    let bugs = info.bugs.as_ref().or(registry_data.bugs.as_ref());

    if homepage.is_some() || repo.is_some() || bugs.is_some() {
        println!();
        println!("  {}", style("Links").bold());

        let mut link_items: Vec<(&str, String)> = Vec::new();
        if let Some(hp) = homepage {
            link_items.push(("Homepage", hp.clone()));
        }
        if let Some(r) = repo {
            let url = r.url.trim_start_matches("git+").trim_end_matches(".git");
            link_items.push(("Repository", url.to_string()));
        }
        if let Some(b) = bugs {
            link_items.push(("Issues", b.url.clone()));
        }

        for (i, (label, url)) in link_items.iter().enumerate() {
            let is_last = i == link_items.len() - 1;
            let connector = if is_last { "└" } else { "├" };
            println!(
                "  {} {} {}",
                style(connector).dim(),
                style(format!("{:<12}", label)).dim(),
                style(url).underlined(),
            );
        }
    }

    // ── Author ──
    if let Some(author) = info.author.as_ref().or(registry_data.author.as_ref()) {
        println!();
        let author_str = if let Some(email) = &author.email {
            format!("{} <{}>", author.name, style(email).dim())
        } else {
            author.name.clone()
        };
        StyledOutput::kv("Author", &author_str);
    }

    // ── Keywords ──
    if let Some(keywords) = info.keywords.as_ref().or(registry_data.keywords.as_ref()) {
        if !keywords.is_empty() {
            let kw_str: Vec<String> = keywords
                .iter()
                .take(8)
                .map(|k| format!("{}", style(k).cyan()))
                .collect();
            let mut display = kw_str.join(&format!("{}", style(", ").dim()));
            if keywords.len() > 8 {
                display.push_str(&format!(
                    " {}",
                    style(format!("+{} more", keywords.len() - 8)).dim()
                ));
            }
            StyledOutput::kv("Keywords", &display);
        }
    }

    // ── Dist info ──
    if let Some(dist) = &info.dist {
        println!();
        println!("  {}", style("Distribution").bold());

        let mut dist_items: Vec<(&str, String)> = Vec::new();

        if let Some(size) = dist.unpacked_size {
            dist_items.push(("Size", format_size(size)));
        }
        if let Some(shasum) = &dist.shasum {
            let short = if shasum.len() > 16 {
                format!("{}…", &shasum[..16])
            } else {
                shasum.clone()
            };
            dist_items.push(("SHA", short));
        }

        for (i, (label, val)) in dist_items.iter().enumerate() {
            let is_last = i == dist_items.len() - 1;
            let connector = if is_last { "└" } else { "├" };
            println!(
                "  {} {} {}",
                style(connector).dim(),
                style(format!("{:<12}", label)).dim(),
                val,
            );
        }
    }

    // ── Dependencies ──
    if let Some(deps) = &info.dependencies {
        if !deps.is_empty() {
            println!();
            println!(
                "  {} {}",
                style("Dependencies").bold(),
                style(format!("({})", deps.len())).dim(),
            );

            let show_count = 12;
            let total = deps.len();
            let mut sorted: Vec<_> = deps.iter().collect();
            sorted.sort_by_key(|(k, _)| k.to_string());

            for (i, (name, ver)) in sorted.iter().enumerate() {
                if i >= show_count {
                    println!(
                        "  {} {}",
                        style("└").dim(),
                        style(format!("… and {} more", total - show_count)).dim(),
                    );
                    break;
                }
                let is_last = i == total - 1 || i == show_count - 1;
                let connector = if is_last && i < show_count {
                    "└"
                } else {
                    "├"
                };
                println!(
                    "  {} {} {}",
                    style(connector).dim(),
                    style(name).cyan(),
                    style(ver).dim(),
                );
            }
        }
    }

    // ── Dist tags ──
    if !registry_data.dist_tags.is_empty() {
        println!();
        println!("  {}", style("Tags").bold());

        let tags: Vec<_> = registry_data.dist_tags.iter().collect();
        for (i, (tag, version)) in tags.iter().enumerate() {
            let is_last = i == tags.len() - 1;
            let connector = if is_last { "└" } else { "├" };
            println!(
                "  {} {} {}",
                style(connector).dim(),
                style(format!("{:<12}", tag)).cyan(),
                style(version).dim(),
            );
        }
    }

    // ── Maintainers ──
    if let Some(maintainers) = info
        .maintainers
        .as_ref()
        .or(registry_data.maintainers.as_ref())
    {
        if !maintainers.is_empty() {
            println!();
            println!("  {}", style("Maintainers").bold());

            for (i, m) in maintainers.iter().enumerate() {
                let is_last = i == maintainers.len() - 1;
                let connector = if is_last { "└" } else { "├" };
                let name_str = if let Some(email) = &m.email {
                    format!("{} {}", m.name, style(format!("<{}>", email)).dim())
                } else {
                    m.name.clone()
                };
                println!("  {} {}", style(connector).dim(), name_str,);
            }
        }
    }

    // ── Published ──
    if let Some(time) = &registry_data.time {
        if let Some(published) = time.get(&info.version) {
            println!();
            StyledOutput::kv("Published", published);
        }
    }

    println!();
}
