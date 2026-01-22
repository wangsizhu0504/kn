use crate::storage::{load, dump, STORAGE};
use crate::display::StyledOutput;

pub fn handle(count: Option<usize>) -> Result<(), Box<dyn std::error::Error>> {
    load();
    let storage = STORAGE.lock();

    if let Some(storage) = storage.as_ref() {
        if storage.command_history.is_empty() {
            println!("\n\x1b[90mNo command history yet\x1b[0m\n");
            return Ok(());
        }

        let limit = count.unwrap_or(10).min(storage.command_history.len());

        println!("\n\x1b[1m📜 Recent Commands\x1b[0m\n");

        for (i, cmd) in storage.command_history.iter().rev().take(limit).enumerate() {
            let index = storage.command_history.len() - i;
            println!("  \x1b[90m{:>3}\x1b[0m  \x1b[36m{}\x1b[0m", index, cmd);
        }

        println!("\n\x1b[90mTip: Run 'kn !!' to repeat last command or 'kn !N' to run command N\x1b[0m\n");
    }

    Ok(())
}

pub fn add_to_history(command: &str) {
    load();
    let mut storage_guard = STORAGE.lock();

    if let Some(storage) = storage_guard.as_mut() {
        // Avoid duplicates of consecutive commands
        if storage.command_history.last().map_or(true, |last| last != command) {
            storage.command_history.push(command.to_string());

            // Keep only last 100 commands
            if storage.command_history.len() > 100 {
                storage.command_history.remove(0);
            }

            storage.last_run_command = Some(command.to_string());
            let _ = dump(storage);
        }
    }
}

pub fn run_from_history(index: usize) -> Result<String, Box<dyn std::error::Error>> {
    load();
    let storage = STORAGE.lock();

    if let Some(storage) = storage.as_ref() {
        if storage.command_history.is_empty() {
            StyledOutput::error("No command history");
            std::process::exit(1);
        }

        if index == 0 || index > storage.command_history.len() {
            StyledOutput::error(&format!("Invalid history index: {}", index));
            std::process::exit(1);
        }

        let cmd = storage.command_history[index - 1].clone();
        println!("\n\x1b[90mRunning:\x1b[0m \x1b[36m{}\x1b[0m\n", cmd);
        Ok(cmd)
    } else {
        StyledOutput::error("No command history");
        std::process::exit(1);
    }
}

pub fn run_last() -> Result<String, Box<dyn std::error::Error>> {
    load();
    let storage = STORAGE.lock();

    if let Some(storage) = storage.as_ref() {
        if let Some(last) = storage.command_history.last() {
            let cmd = last.clone();
            println!("\n\x1b[90mRunning:\x1b[0m \x1b[36m{}\x1b[0m\n", cmd);
            Ok(cmd)
        } else {
            StyledOutput::error("No command history");
            std::process::exit(1);
        }
    } else {
        StyledOutput::error("No command history");
        std::process::exit(1);
    }
}
