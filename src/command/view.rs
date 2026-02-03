// 此模块保留供将来使用，当前未集成
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

pub fn handle(package: String, version: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Viewing package: {} (version: {:?})", package, version);

    let registry = get_registry_url();
    let url = format!("{}/{}", registry, package);

    tracing::debug!("Fetching from registry: {}", url);

    // Fetch package information from registry
    let client = ureq::AgentBuilder::new()
        .timeout(std::time::Duration::from_secs(30))
        .build();

    let response = client
        .get(&url)
        .set("Accept", "application/json")
        .call()
        .map_err(|e| format!("Failed to fetch package info: {}", e))?;

    let registry_data: RegistryResponse = response.into_json()?;

    // Determine which version to display
    let target_version = version
        .or_else(|| registry_data.dist_tags.get("latest").cloned())
        .ok_or("No version specified and no latest tag found")?;

    let package_info = registry_data
        .versions
        .get(&target_version)
        .ok_or_else(|| format!("Version {} not found", target_version))?;

    // Display package information
    print_package_info(&registry_data, package_info)?;

    Ok(())
}

fn get_registry_url() -> String {
    // Check environment variable first
    if let Ok(registry) = std::env::var("NPM_CONFIG_REGISTRY") {
        return registry.trim_end_matches('/').to_string();
    }

    // Check .npmrc file
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

    // Default to npm registry
    "https://registry.npmjs.org".to_string()
}

fn print_package_info(
    registry_data: &RegistryResponse,
    package_info: &PackageInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    // Header with package name, version, and basic stats
    let deps_count = package_info
        .dependencies
        .as_ref()
        .map(|d| d.len())
        .unwrap_or(0);
    let versions_count = registry_data.versions.len();

    println!();
    println!(
        "\x1b[1;36m{}@{} | {} | deps: {} | versions: {}\x1b[0m",
        package_info.name,
        package_info.version,
        package_info.license.as_deref().unwrap_or("UNLICENSED"),
        deps_count,
        versions_count
    );

    // Description
    if let Some(desc) = package_info
        .description
        .as_ref()
        .or(registry_data.description.as_ref())
    {
        if !desc.is_empty() {
            println!("{}", desc);
        }
    }

    // Homepage
    if let Some(homepage) = package_info
        .homepage
        .as_ref()
        .or(registry_data.homepage.as_ref())
    {
        println!("\n{}", homepage);
    }

    // Keywords
    if let Some(keywords) = package_info
        .keywords
        .as_ref()
        .or(registry_data.keywords.as_ref())
    {
        if !keywords.is_empty() {
            println!("\nkeywords: {}", keywords.join(", "));
        }
    }

    // Distribution info
    if let Some(dist) = &package_info.dist {
        println!("\ndist");
        println!("  tarball: {}", dist.tarball);
        if let Some(shasum) = &dist.shasum {
            println!("  shasum: {}", shasum);
        }
        if let Some(integrity) = &dist.integrity {
            println!("  integrity: {}", integrity);
        }
        if let Some(size) = dist.unpacked_size {
            println!("  unpackedSize: {} MB", size as f64 / 1024.0 / 1024.0);
        }
    }

    // Author
    if let Some(author) = package_info
        .author
        .as_ref()
        .or(registry_data.author.as_ref())
    {
        print!("\nauthor: {}", author.name);
        if let Some(email) = &author.email {
            print!(" <{}>", email);
        }
        println!();
    }

    // Repository
    if let Some(repo) = package_info
        .repository
        .as_ref()
        .or(registry_data.repository.as_ref())
    {
        println!("\nrepository: {}:{}", repo.repo_type, repo.url);
    }

    // Bugs
    if let Some(bugs) = package_info.bugs.as_ref().or(registry_data.bugs.as_ref()) {
        println!("\nbugs: {}", bugs.url);
    }

    // Dependencies
    if let Some(deps) = &package_info.dependencies {
        if !deps.is_empty() {
            println!("\ndependencies: {}", deps.len());
            let show_count = 24;
            for (i, (name, version)) in deps.iter().enumerate() {
                if i >= show_count {
                    println!("  (... and {} more)", deps.len() - show_count);
                    break;
                }
                println!("  {}: {}", name, version);
            }
        }
    }

    // Maintainers
    if let Some(maintainers) = package_info
        .maintainers
        .as_ref()
        .or(registry_data.maintainers.as_ref())
    {
        if !maintainers.is_empty() {
            println!("\nmaintainers:");
            for maintainer in maintainers {
                print!("  - {}", maintainer.name);
                if let Some(email) = &maintainer.email {
                    print!(" <{}>", email);
                }
                println!();
            }
        }
    }

    // Dist-tags
    if !registry_data.dist_tags.is_empty() {
        println!("\ndist-tags:");
        for (tag, version) in &registry_data.dist_tags {
            println!("  {}: {}", tag, version);
        }
    }

    // Published time
    if let Some(time) = &registry_data.time {
        if let Some(published) = time.get(&package_info.version) {
            println!("\npublished: {}", published);
        }
    }

    println!();
    Ok(())
}
