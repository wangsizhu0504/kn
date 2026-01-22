use crate::storage::{load, dump, STORAGE};
use crate::display::StyledOutput;

pub fn handle(action: Option<String>, key: Option<String>, value: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    load();

    match action.as_deref() {
        None | Some("list") | Some("ls") => list_aliases(),
        Some("set") | Some("add") => {
            if let (Some(k), Some(v)) = (key, value) {
                set_alias(&k, &v)
            } else {
                StyledOutput::error("Usage: kn alias set <name> <script>");
                std::process::exit(1);
            }
        }
        Some("remove") | Some("rm") | Some("delete") => {
            if let Some(k) = key {
                remove_alias(&k)
            } else {
                StyledOutput::error("Usage: kn alias remove <name>");
                std::process::exit(1);
            }
        }
        Some(unknown) => {
            StyledOutput::error(&format!("Unknown alias action: {}", unknown));
            std::process::exit(1);
        }
    }
}

fn list_aliases() -> Result<(), Box<dyn std::error::Error>> {
    let storage = STORAGE.lock();

    if let Some(storage) = storage.as_ref() {
        if storage.aliases.is_empty() {
            println!("\n\x1b[90mNo aliases defined\x1b[0m");
            println!("\n\x1b[90mCreate one with:\x1b[0m kn alias set \x1b[36m<name>\x1b[0m \x1b[36m<script>\x1b[0m\n");
            return Ok(());
        }

        println!("\n\x1b[1m🔗 Script Aliases\x1b[0m\n");

        for (name, script) in storage.aliases.iter() {
            println!("  \x1b[36m{:<15}\x1b[0m → \x1b[90m{}\x1b[0m", name, script);
        }
        println!();
    }

    Ok(())
}

fn set_alias(name: &str, script: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut storage_guard = STORAGE.lock();

    if let Some(storage) = storage_guard.as_mut() {
        storage.aliases.insert(name.to_string(), script.to_string());
        dump(storage)?;
        println!("\n\x1b[32m✓\x1b[0m Alias created: \x1b[36m{}\x1b[0m → \x1b[90m{}\x1b[0m\n", name, script);
    }

    Ok(())
}

fn remove_alias(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut storage_guard = STORAGE.lock();

    if let Some(storage) = storage_guard.as_mut() {
        if storage.aliases.remove(name).is_some() {
            dump(storage)?;
            println!("\n\x1b[32m✓\x1b[0m Alias removed: \x1b[36m{}\x1b[0m\n", name);
        } else {
            StyledOutput::error(&format!("Alias not found: {}", name));
        }
    }

    Ok(())
}

pub fn resolve_alias(name: &str) -> Option<String> {
    load();
    let storage = STORAGE.lock();

    if let Some(storage) = storage.as_ref() {
        storage.aliases.get(name).cloned()
    } else {
        None
    }
}
