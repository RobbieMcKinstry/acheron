use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{process, thread};

use acheron::Parser;

const DIR: &str = "examples/satisfiable/uniform-random-3sat";
const MAX_FILES: usize = 1700;
const TIME_LIMIT: Duration = Duration::from_secs(60);

fn main() {
    // Phase 1: Discover and sort CNF files from the directory.
    let mut paths: Vec<PathBuf> = fs::read_dir(DIR)
        .expect("failed to read benchmark directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension().is_some_and(|ext| ext == "cnf") {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    // Natural sort: order by variable count (uf20 before uf125), then instance number.
    paths.sort_by(|a, b| {
        let key = |p: &PathBuf| -> (u32, u32) {
            let stem = p.file_stem().unwrap_or_default().to_string_lossy();
            // Filename format: "ufNN-MM"
            let parts: Vec<&str> = stem.splitn(2, '-').collect();
            let var_count = parts
                .first()
                .and_then(|s| s.strip_prefix("uf"))
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            let instance = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            (var_count, instance)
        };
        key(a).cmp(&key(b))
    });
    paths.truncate(MAX_FILES);

    // Phase 2: Parse all files upfront, building a queue of solvers.
    eprintln!("Loading {} files...", paths.len());
    let solvers: Vec<_> = paths
        .iter()
        .map(|path| {
            let path_str = path.to_str().expect("non-UTF-8 path");
            let parser = Parser::new(path_str);
            (path.clone(), parser.solver)
        })
        .collect();
    eprintln!("All files loaded. Starting timer.");

    // Phase 3: Start a timer thread that sets a stop flag after TIME_LIMIT.
    let stop = Arc::new(AtomicBool::new(false));
    let stop_timer = Arc::clone(&stop);
    thread::spawn(move || {
        thread::sleep(TIME_LIMIT);
        stop_timer.store(true, Ordering::Relaxed);
    });

    // Phase 4: Run solvers sequentially until the stop flag is set.
    let start = Instant::now();
    let mut solved = 0;

    for (path, solver) in &solvers {
        if stop.load(Ordering::Relaxed) {
            break;
        }
        match solver.solve_interruptible(&stop) {
            Some(result) => {
                solved += 1;
                let label = if result { "SAT" } else { "UNSAT" };
                eprintln!("  [{}] {} -> {}", solved, path.display(), label);
            }
            None => break, // interrupted mid-solve
        }
    }

    let elapsed = start.elapsed();
    let secs = elapsed.as_secs_f64();
    eprintln!("Solved {} problems in {:.2}s", solved, secs);
    println!("{{\"solved\":{},\"elapsed_secs\":{:.2}}}", solved, secs);
    process::exit(0);
}
