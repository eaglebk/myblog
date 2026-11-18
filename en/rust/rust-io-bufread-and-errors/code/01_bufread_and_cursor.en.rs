// Step 1: Magic Function approach and clean testability with `impl BufRead` and `Cursor`

use std::io::{BufRead, Cursor, Result};

pub struct LogMetrics {
    pub total_lines: usize,
    pub error_lines: usize,
}

// Accept generic `impl BufRead` instead of hardcoding `StdinLock` or `BufReader<File>`:
pub fn analyze_log_stream(input: impl BufRead) -> Result<LogMetrics> {
    let mut total_lines = 0;
    let mut error_lines = 0;

    for line in input.lines() {
        let text = line?; // Handle read errors on each line
        total_lines += 1;
        if text.contains("[ERROR]") || text.contains("[FATAL]") {
            error_lines += 1;
        }
    }

    Ok(LogMetrics {
        total_lines,
        error_lines,
    })
}

fn main() {
    // In tests, wrap an in-memory string in std::io::Cursor with zero filesystem I/O:
    let fake_log_data = "[INFO] System start\n[ERROR] Connection timeout\n[INFO] Retrying\n";
    let cursor = Cursor::new(fake_log_data);

    let metrics = analyze_log_stream(cursor).expect("Failed to analyze log stream");
    assert_eq!(metrics.total_lines, 3);
    assert_eq!(metrics.error_lines, 1);
    println!(
        "Lines analyzed: {}, errors found: {}",
        metrics.total_lines, metrics.error_lines
    );
}

// ! **1. Magic Function and BufRead**
// ! - Design the function signature from the ideal call site, choosing the minimal trait bound `impl BufRead`.
// ! - `std::io::Cursor` lets you unit-test streaming readers directly in memory without touching disk.

// ---

// Step 2: Locking stdin().lock() once for high-throughput input
// ?ignore

use std::io::{self, stdin, BufRead};

fn process_user_stream(input: impl BufRead) -> io::Result<usize> {
    let mut count = 0;
    for line in input.lines() {
        let _ = line?; // Propagate I/O or UTF-8 errors explicitly instead of dropping them silently
        count += 1;
    }
    Ok(count)
}

fn main() -> io::Result<()> {
    // stdin().lock() acquires the global stdin mutex once for the entire loop
    // instead of locking and unlocking on every line:
    let locked_stdin = stdin().lock();

    // Pass the locked handle into the same generic function:
    let count = process_user_stream(locked_stdin)?;
    println!("Lines read from standard input: {count}");
    Ok(())
}

// ! **2. Acquiring the Lock via `stdin().lock()`**
// ! - Standard `stdin()` is guarded by an internal Mutex. Calling `.read_line()` repeatedly locks and unlocks it on every iteration.
// ! - `stdin().lock()` returns `StdinLock`, which implements `BufRead` and eliminates per-line synchronization overhead.

