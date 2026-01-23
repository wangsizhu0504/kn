use crate::storage::{load, ScriptStats, STORAGE};

pub fn handle() -> Result<(), Box<dyn std::error::Error>> {
    load();
    let storage = STORAGE.lock();

    if let Some(storage) = storage.as_ref() {
        if storage.script_stats.is_empty() {
            println!("\n\x1b[90mNo script statistics yet\x1b[0m");
            println!("\x1b[90mRun some scripts to see performance data\x1b[0m\n");
            return Ok(());
        }

        println!("\n\x1b[1m📊 Script Performance Statistics\x1b[0m\n");

        // Sort by total runs
        let mut stats: Vec<_> = storage.script_stats.iter().collect();
        stats.sort_by(|a, b| b.1.total_runs.cmp(&a.1.total_runs));

        println!(
            "  \x1b[1m{:<20} {:>8} {:>12} {:>12}\x1b[0m",
            "Script", "Runs", "Avg Time", "Last Run"
        );
        println!("  {}", "─".repeat(60));

        for (name, stat) in stats.iter() {
            let avg_time = if stat.total_runs > 0 {
                stat.total_time_ms / stat.total_runs as u64
            } else {
                0
            };

            let time_str = format_duration(avg_time);

            println!(
                "  \x1b[36m{:<20}\x1b[0m \x1b[33m{:>8}\x1b[0m \x1b[32m{:>12}\x1b[0m \x1b[90m{:>12}\x1b[0m",
                name,
                stat.total_runs,
                time_str,
                &stat.last_run[..10]
            );
        }

        println!();
    }

    Ok(())
}

fn format_duration(ms: u64) -> String {
    if ms < 1000 {
        format!("{}ms", ms)
    } else if ms < 60000 {
        format!("{:.1}s", ms as f64 / 1000.0)
    } else {
        let mins = ms / 60000;
        let secs = (ms % 60000) / 1000;
        format!("{}m{}s", mins, secs)
    }
}

pub fn record_execution(script_name: &str, duration_ms: u64) {
    load();
    let mut storage_guard = STORAGE.lock();

    if let Some(storage) = storage_guard.as_mut() {
        let now = chrono::Local::now().format("%Y-%m-%d").to_string();

        storage
            .script_stats
            .entry(script_name.to_string())
            .and_modify(|stat| {
                stat.total_runs += 1;
                stat.total_time_ms += duration_ms;
                stat.last_run = now.clone();
            })
            .or_insert(ScriptStats {
                total_runs: 1,
                total_time_ms: duration_ms,
                last_run: now,
            });

        let _ = crate::storage::dump(storage);
    }
}

#[derive(Clone)]
pub struct StatInfo {
    pub script: String,
    pub count: u32,
    pub average_time: u64,
    pub last_run: Option<u64>,
}

pub fn get_all_stats() -> Vec<StatInfo> {
    load();
    let storage = STORAGE.lock();

    if let Some(storage) = storage.as_ref() {
        storage
            .script_stats
            .iter()
            .map(|(name, stat)| {
                let avg_time = if stat.total_runs > 0 {
                    stat.total_time_ms / stat.total_runs as u64
                } else {
                    0
                };

                // Parse last_run date string to timestamp
                let last_run_timestamp =
                    chrono::NaiveDate::parse_from_str(&stat.last_run, "%Y-%m-%d")
                        .ok()
                        .and_then(|date| {
                            date.and_hms_opt(0, 0, 0)
                                .map(|dt| dt.and_utc().timestamp() as u64)
                        });

                StatInfo {
                    script: name.clone(),
                    count: stat.total_runs,
                    average_time: avg_time,
                    last_run: last_run_timestamp,
                }
            })
            .collect()
    } else {
        Vec::new()
    }
}
