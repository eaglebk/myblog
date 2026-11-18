// Step 1: Propagating errors with `?` while iterating over `.lines()`

use std::io::{self, BufRead, Cursor, ErrorKind, Result};

pub fn parse_config_values(reader: impl BufRead) -> Result<Vec<u32>> {
    let mut values = Vec::new();

    for line_result in reader.lines() {
        // `?` returns early on read failures or invalid UTF-8 sequences:
        let line = line_result?;
        let trimmed = line.trim();

        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            let num = trimmed
                .parse::<u32>()
                .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
            values.push(num);
        }
    }

    Ok(values)
}

fn main() {
    let raw_config = "100\n# comment\n200\n300\n";
    let cursor = Cursor::new(raw_config);

    match parse_config_values(cursor) {
        Ok(parsed) => println!("Parsed numbers: {:?}", parsed),
        Err(err) => eprintln!("Read error: {err}"),
    }
}

// ! **1. Safe Iteration Over `.lines()`**
// ! - `.lines()` yields `Result<String>` instead of plain `String` because I/O can fail mid-stream at any moment.
// ! - Using `?` propagates errors cleanly without panicking.

// ---

// Step 2: Writing errors to `stderr` and returning `ExitCode`

// ?hidden:start
use std::io::{self, BufRead, Cursor, ErrorKind, Result};

pub fn parse_config_values(reader: impl BufRead) -> Result<Vec<u32>> {
    let mut values = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            let num = trimmed
                .parse::<u32>()
                .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
            values.push(num);
        }
    }
    Ok(values)
}
// ?hidden:end

use std::process::ExitCode;

fn run_cli_pipeline() -> Result<()> {
    let sample = Cursor::new("42\n84\n");
    let results = parse_config_values(sample)?;
    println!("Processed items: {}", results.len());
    Ok(())
}

fn main() -> ExitCode {
    // Write errors to `stderr` via `eprintln!` and return `ExitCode`
    // so all `Drop` destructors and buffer flushes run cleanly:
    match run_cli_pipeline() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Fatal CLI error: {err}");
            ExitCode::FAILURE
        }
    }
}

// ! **2. Separating `stdout`/`stderr` and Returning `ExitCode`**
// ! - Printing errors with `eprintln!` protects Shell pipelines (`cat data | my_tool > output.txt`) from corrupting output files.
// ! - Returning `ExitCode::FAILURE` from `main()` reports failure to scripts and CI/CD while ensuring `Drop` runs.
