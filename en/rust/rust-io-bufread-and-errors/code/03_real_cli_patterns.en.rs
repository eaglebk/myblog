// Step 1: A single interface for files, stdin, and unit tests (Box<dyn BufRead>)

use std::fs::File;
use std::io::{self, BufRead, BufReader, Cursor};

// At the CLI boundary: "-" or None reads stdin, anything else opens a file.
#[allow(dead_code)]
fn open_input(path: Option<&str>) -> io::Result<Box<dyn BufRead>> {
    match path {
        Some("-") | None => Ok(Box::new(io::stdin().lock())),
        Some(p) => Ok(Box::new(BufReader::new(File::open(p)?))),
    }
}

// Core logic stays simple: it just takes impl BufRead.
fn count_matching(mut reader: impl BufRead, pattern: &str) -> io::Result<usize> {
    let mut count = 0;
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        let clean = line.trim_end_matches(&['\r', '\n']);
        if clean.contains(pattern) {
            count += 1;
        }
        line.clear();
    }
    Ok(count)
}

fn main() -> io::Result<()> {
    // In tests or Playground, swap the file/stdin for an in-memory Cursor:
    let fake_input = Cursor::new("INFO ok\nERROR disk full\nERROR timeout\n");
    let errors = count_matching(fake_input, "ERROR")?;
    assert_eq!(errors, 2);
    println!("Matching lines: {errors}");

    // In a real CLI binary:
    // let reader = open_input(std::env::args().nth(1).as_deref())?;
    Ok(())
}

// ! **1. Unified input like `bat` and `xsv`**
// ! - `Box<dyn BufRead>` erases the difference between `BufReader<File>` and `StdinLock`.
// ! - `read_line` keeps `\n` at the end, so strip it with `trim_end_matches` before matching.
// ! - `StdinLock` is `!Send` (and keeps `stdin` locked for its lifetime): to move a reader across threads, wrap `io::stdin()` in `Box<dyn BufRead + Send>`.

// ---

// Step 2: Fast buffered output via BufWriter<StdoutLock>

use std::io::{self, BufRead, BufWriter, Cursor, Write};

fn filter_logs(mut reader: impl BufRead, mut out: impl Write, pattern: &str) -> io::Result<()> {
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        let clean = line.trim_end_matches(&['\r', '\n']);
        if clean.contains(pattern) {
            // Write into the buffer instead of calling println! (avoids per-line lock + flush)
            writeln!(out, "{clean}")?;
        }
        line.clear();
    }

    // Explicit flush sends remaining bytes and surfaces any write error
    out.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let input = Cursor::new("INFO start\nERROR db\nINFO done\n");
    let stdout = io::stdout();
    let out = BufWriter::new(stdout.lock());

    filter_logs(input, out, "ERROR")?;
    Ok(())
}

// ! **2. Buffered output instead of `println!` in loops**
// ! - `stdout().lock()` acquires the mutex once, and `BufWriter` batches writes in memory.
// ! - Calling `out.flush()?` explicitly is important because errors during `Drop` flush are ignored.

// ---

// Step 3: Clean ExitCode and silent BrokenPipe handling (like ripgrep)

use std::io::{self, BufWriter, ErrorKind, Write};
use std::process::ExitCode;

fn run_stream(mut out: impl Write) -> io::Result<()> {
    for i in 0..5 {
        writeln!(out, "Log line #{i}")?;
    }
    out.flush()?;
    Ok(())
}

fn handle_result(res: io::Result<()>) -> ExitCode {
    match res {
        Ok(()) => ExitCode::SUCCESS,
        // If a downstream pipe closes early (e.g. `my_tool | head -n 5`), exit cleanly with 0:
        Err(err) if err.kind() == ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Runtime error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn main() -> ExitCode {
    let stdout = io::stdout();
    let out = BufWriter::new(stdout.lock());

    // Verify BrokenPipe handling and run the stream:
    assert_eq!(
        handle_result(Err(io::Error::new(ErrorKind::BrokenPipe, "pipe closed"))),
        ExitCode::SUCCESS
    );
    println!("BrokenPipe handled cleanly (ExitCode::SUCCESS)");
    handle_result(run_stream(out))
}

// ! **3. Playing nicely with UNIX pipes and `ExitCode`**
// ! - Returning `ExitCode` from `main()` runs `Drop` destructors for all open files and buffers (unlike `process::exit`).
// ! - On `ErrorKind::BrokenPipe`, the tool exits with `ExitCode::SUCCESS`, just like `ripgrep` and `bat`.

