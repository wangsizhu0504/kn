use crate::command_utils::run_script_fast;
use crate::display::StyledOutput;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn handle(scripts: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    if scripts.is_empty() {
        StyledOutput::error("No scripts specified for parallel execution");
        println!("\n\x1b[90mUsage:\x1b[0m kn parallel \x1b[36m<script1>\x1b[0m \x1b[36m<script2>\x1b[0m ...\n");
        std::process::exit(1);
    }

    println!(
        "\n\x1b[1m⚡ Running {} scripts in parallel...\x1b[0m\n",
        scripts.len()
    );

    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for (idx, script) in scripts.iter().enumerate() {
        let script_name = script.clone();
        let results_clone = Arc::clone(&results);

        let handle = thread::spawn(move || {
            let start = std::time::Instant::now();

            println!(
                "\x1b[36m[{}]\x1b[0m Starting \x1b[90m{}\x1b[0m",
                idx + 1,
                script_name
            );

            let result = run_script_fast(&script_name, &[]);
            let duration = start.elapsed();

            let mut results = results_clone.lock().unwrap();
            results.push((script_name.clone(), result.is_ok(), duration));

            if result.is_ok() {
                println!(
                    "\x1b[32m[{}] ✓\x1b[0m {} \x1b[90m({:.2}s)\x1b[0m",
                    idx + 1,
                    script_name,
                    duration.as_secs_f64()
                );
            } else {
                println!(
                    "\x1b[31m[{}] ✗\x1b[0m {} \x1b[90m({:.2}s)\x1b[0m",
                    idx + 1,
                    script_name,
                    duration.as_secs_f64()
                );
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        let _ = handle.join();
    }

    // Print summary
    let results = results.lock().unwrap();
    let successful = results.iter().filter(|(_, ok, _)| *ok).count();
    let failed = results.len() - successful;

    println!("\n\x1b[1m📊 Summary\x1b[0m");
    println!("  \x1b[32m✓ Successful:\x1b[0m {}", successful);
    if failed > 0 {
        println!("  \x1b[31m✗ Failed:\x1b[0m {}", failed);
    }

    let total_time: f64 = results.iter().map(|(_, _, d)| d.as_secs_f64()).sum();
    let max_time = results
        .iter()
        .map(|(_, _, d)| d.as_secs_f64())
        .fold(0.0, f64::max);

    println!("  \x1b[90mTotal time:\x1b[0m {:.2}s", max_time);
    println!(
        "  \x1b[90mTime saved:\x1b[0m ~{:.2}s\n",
        total_time - max_time
    );

    if failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}
