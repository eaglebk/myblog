# Fast I/O in Rust: Buffering, In-Memory Tests, and CLI Errors

> How to cut syscall overhead with BufRead, unit-test streams via std::io::Cursor, handle .lines() failures, and return clean UNIX exit codes in CLI tools.

- **URL:** https://eagle2000.space/en/rust/rust-io-bufread-and-errors/
- **Date:** 2026-11-18

- **Tags:** rust, dev, io, errors

---
## Introduction

Handling input and output streams (I/O) sits at the core of CLI tools, log parsers, and system services. Rust's standard library is built so you do not pay a syscall or heap allocation per byte while still keeping memory safety intact. I covered the basics of `BufReader` in a [previous post on speeding up file I/O](../bufreader/); today I will dig into day-to-day production patterns.

While writing small scripts, `File::open` and `stdin().read_line()` work without surprises. Once you run a few gigabytes of logs through a CLI tool, though, it stalls on syscalls and allocations, `stdin()` code gets awkward to unit-test without temp files, and a pipe like `my_tool | head -n 5` suddenly panics.

Here is how to fix all of that using only `std::io`.

---

## 1. OS I/O Anatomy and `Read` vs `BufRead`

### System Call Overhead
At the operating system level, reading bytes from a file or socket goes through a kernel system call (such as `sys_read` on POSIX).

Every syscall forces a **context switch** from User Mode to Kernel Mode and back. If you read a 1 MB file one byte at a time directly via `std::fs::File`, your program makes **1,048,576 syscalls**, spending almost all of its CPU budget crossing the kernel boundary rather than processing data.

**1. Unbuffered byte-by-byte reading (~1,000,000 syscalls):**

{{< mermaid >}}
graph LR
    U1["User Code"] -- "sys_read()" --> K1["Kernel Mode"] -- "I/O" --> D1["Disk"]
{{< /mermaid >}}

**2. Buffered reading via BufReader (~128 syscalls):**

{{< mermaid >}}
graph LR
    U2["User Code"] -- "RAM Buffer (8 KB)" --> K2["Kernel Mode"] -- "I/O" --> D2["Disk"]
{{< /mermaid >}}

{{< flow-stepper src="flows/bufreader-flow.en.json" >}}

### `Read` vs `BufRead`
Rust's standard library splits I/O reading into two core traits:
1. **`std::io::Read`**: the low-level trait for pulling raw bytes into a slice via `.read(&mut [u8])`. It makes no guarantee of internal buffering.
2. **`std::io::BufRead`**: an extension trait for readers backed by an **in-memory buffer** (8 KB by default in `BufReader<R>`). It unlocks line-oriented and delimiter-based helpers such as `.read_line()`, `.lines()`, `.read_until()`, and `.fill_buf()`.

---

## 2. The "Magic Function" Pattern and Testing with `impl BufRead`

### Design from Invocation (Magic Function)
I first came across this idea in [the great book "The Secrets of Rust: Tools"](https://bitfieldconsulting.com/books/rust-tools). Reading a few chapters changed how I design CLI utilities. Let's see how this approach works in practice.

When designing library functions, a great trick is working backwards from the ideal call site: write the call in `main()` as if the perfect helper already exists:

```rust
// Ideal call site in main():
let metrics = analyze_log_stream(input)?;
```

Only after writing that line do you decide the minimal trait bounds required for `input`.

### Why Concrete Types Hurt Testability
If you hardcode `&mut File` or `StdinLock`:

```rust
// Rigid signature: tied to physical files on disk
pub fn analyze_file(file: &mut std::fs::File) -> std::io::Result<usize> { ... }
```

writing fast unit tests becomes painful: every test has to create temporary files on disk, clean them up afterwards, and deal with filesystem state.

### The Clean Solution: `impl BufRead` and `std::io::Cursor`
Accepting a generic `impl BufRead` makes your function completely source-agnostic:

- In **production code**, pass a `BufReader<File>` (for large files, bump the buffer via `BufReader::with_capacity(64 * 1024, file)`) or a locked `stdin().lock()`.
- In **unit tests**, pass `std::io::Cursor::new("line 1\nline 2")` - an in-memory reader backed by a byte slice or string with zero disk I/O.

{{< alert "comment" >}}
**What is `std::io::Cursor`?** In short, it is a lightweight wrapper around in-memory data (`&str`, `&[u8]`, or `Vec<u8>`) that tracks a byte offset (`pos: u64`) internally. That cursor lets a plain slice or vector behave just like an open file: it implements `Read`, `BufRead`, `Seek`, and `Write` (for mutable buffers). Every `.read()` or `.read_line()` simply advances the offset in RAM with zero syscalls or temp files.
{{< /alert >}}

Here is a step-by-step example combining `impl BufRead`, in-memory `Cursor` testing, and `stdin().lock()`:


### 💻 BufRead, Cursor in tests, and stdin().lock()


#### Step 1

```rust
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
```

**1. Magic Function and BufRead**
- Design the function signature from the ideal call site, choosing the minimal trait bound `impl BufRead`.
- `std::io::Cursor` lets you unit-test streaming readers directly in memory without touching disk.


#### Step 2

```rust
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
```

**2. Acquiring the Lock via `stdin().lock()`**
- Standard `stdin()` is guarded by an internal Mutex. Calling `.read_line()` repeatedly locks and unlocks it on every iteration.
- `stdin().lock()` returns `StdinLock`, which implements `BufRead` and eliminates per-line synchronization overhead.


---

## 3. Inside `stdin().lock()` and the Global Mutex

### How `std::io::stdin()` Works Under the Hood
A process's standard input (`stdin`) is a global shared resource that multiple threads might touch at once. To prevent data corruption and races, `std::io::Stdin` in the standard library is protected by a global reentrant mutex (`ReentrantMutex`).

{{< alert "comment" >}}
**Why a `ReentrantMutex` instead of a regular `Mutex`?** Calling `.lock()` twice on a standard `std::sync::Mutex` from the same thread causes a deadlock - the thread waits on itself forever. A reentrant mutex (`ReentrantMutex` / `ReentrantLock` inside `std`) tracks the owning thread and a lock counter. If the same thread already holds `stdin().lock()` and calls `stdin()` again deeper in the call stack, it won't deadlock: the counter simply increments, while other threads still block until every guard is dropped.

Because multiple locks can coexist on the same thread, it only yields a shared `&T` reference rather than `&mut T` (paired internally with `RefCell` for runtime borrow checking in `Stdin`), which is why regular `Mutex` remains the default choice in application code.
{{< /alert >}}

If you call methods directly on `stdin()` inside a loop without locking it first:

```rust
// Slow path:
let mut input = std::io::stdin();
for _ in 0..1000 {
    let mut line = String::new();
    input.read_line(&mut line)?; // Locks and unlocks the Mutex on EVERY iteration!
}
```

on every single line your program:
1. Acquires the `stdin` mutex.
2. Reads one line.
3. Releases the `stdin` mutex.

### The Fix: Lock Once with `.lock()` and Reuse the Buffer
Calling `stdin().lock()` claims ownership of the stream for the current thread up front and returns `StdinLock<'static>`, which implements `BufRead`. Pairing that with a single `String` buffer cleared via `line.clear()` eliminates both per-line mutex locking and repeated heap allocations:

{{< code-before-after before_label="Slow (lock + alloc per line)" after_label="Fast (stdin.lock + buf.clear)" badge="0 extra locks" caption="Comparison: locking stdin().lock() once before the loop and reusing a single String buffer removes redundant synchronization and allocations" >}}
```rust
let input = std::io::stdin();
for _ in 0..1000 {
    let mut line = String::new();
    input.read_line(&mut line)?;
    process(&line);
}
```
===
```rust
let stdin = std::io::stdin();
let mut handle = stdin.lock();
let mut line = String::new();
while handle.read_line(&mut line)? > 0 {
    process(&line);
    line.clear();
}
```
===
Mutex lock ;; let input = std::io::stdin(); ;; let mut handle = stdin.lock(); ;; Instead of locking and unlocking the global mutex on every line, acquire StdinLock once before the loop.
Allocation ;; let mut line = String::new(); ;; let mut line = String::new();||line.clear(); ;; Allocate the String buffer once outside the loop and reset the String's length (len = 0) with line.clear() while keeping its heap capacity.
Reading ;; input.read_line(&mut line)?; ;; handle.read_line(&mut line)? ;; StdinLock implements BufRead and reads lines straight from the buffer with zero per-line lock overhead.
{{< /code-before-after >}}

Synchronization overhead drops to zero across the loop, making line-by-line reads dramatically faster.

---

## 4. `.lines()` Error Paths and UNIX Shell Integration

### Why `.lines()` Yields `Result<String>`
Unlike iterating over an in-memory `Vec<T>`, reading from an I/O stream can fail mid-way at any moment:
- A disk/device error, disconnected drive, or dropped socket (`EIO` / `ErrorKind::ConnectionReset`).
- Insufficient file permissions (`ErrorKind::PermissionDenied`).
- Invalid UTF-8 bytes in the input stream (`ErrorKind::InvalidData`).

*(Note: OS signal interruptions `EINTR` / `ErrorKind::Interrupted` are automatically retried inside `std::io::BufRead::read_until`, so `.lines()` never surfaces them.)*

That is why `.lines()` yields `Result<String, std::io::Error>`, ensuring every line is checked with `?` or `match`.

### Memory Allocation: `lines()` vs `read_line()`
As shown in the previous section, allocation behavior differs drastically:
- **`reader.lines()`**: allocates a **brand-new `String`** on every iteration (10,000,000 lines = 10,000,000 heap allocations).
- **`reader.read_line(&mut buf)`**: reuses a **single `String` buffer** via `buf.clear()`, keeping allocations flat.

> [!WARNING]
> **Migration gotcha from `.lines()` to `read_line()`:** `.lines()` strips trailing `\n` and `\r\n` automatically, whereas `read_line(&mut buf)` **keeps** them in the buffer. Always trim line endings with `buf.trim_end_matches(&['\r', '\n'])` before parsing.
>
> For raw system logs that may contain non-UTF-8 bytes, prefer `reader.read_until(b'\n', &mut byte_buf)` with `String::from_utf8_lossy`. And if external input might lack `\n` (such as a multi-gigabyte single-line file), cap line reads with `reader.by_ref().take(MAX_LINE_LEN)` to avoid running out of memory (OOM).

### Separating `stdout` / `stderr` and `ExitCode` Instead of `process::exit`
Well-behaved CLI utilities are built to compose inside UNIX pipes:

```bash
cat server.log | my_rust_tool | grep "CRITICAL" > result.txt
```

To keep pipelines clean, follow two rules:

1. **Always write errors to `stderr` with `eprintln!`**: if you print errors with `println!`, they go into `stdout` and end up inside `result.txt`, silently corrupting downstream data.
2. **Return standard exit codes via `std::process::ExitCode`**:
   - `0` (`ExitCode::SUCCESS`) - success.
   - `1` (`ExitCode::FAILURE`) - general runtime or data-processing error.
   - `2` (`ExitCode::from(2)`) - invalid CLI usage or bad arguments.
   - Prefer returning `std::process::ExitCode` directly from `main()` over calling `std::process::exit(code)`. `process::exit` terminates immediately **without running `Drop` destructors**, which leaves `BufWriter<File>` buffers unflushed and temp files uncleaned.

### Buffered Output and `BrokenPipe` Like `ripgrep` and `bat`
Calling `println!` inside a loop creates two issues: `std::io::stdout()` wraps a `LineWriter` (locking `stdout` and flushing on every `\n`), and the macro calls `.unwrap()` internally. When a downstream pipe closes early - such as `my_tool big.log | head -n 5` - Rust turns `SIGPIPE` (`SIG_IGN`) into `ErrorKind::BrokenPipe`, and `println!` **panics** with a stack trace.

In production tools, `stdout().lock()` is wrapped in a `BufWriter`, output uses `writeln!`, `out.flush()?` is called explicitly (because `Drop` ignores flush errors), and `ErrorKind::BrokenPipe` returns `ExitCode::SUCCESS` from `try_main()`. Under `set -o pipefail` in CI/CD scripts, this exits cleanly with `0` instead of a panic or signal `141` - the approach used in [`BurntSushi/ripgrep`](https://github.com/BurntSushi/ripgrep/blob/master/crates/core/main.rs) ([issue #200](https://github.com/BurntSushi/ripgrep/issues/200)) and `sharkdp/bat`.

To accept both file paths and `-`/piped `stdin` in one CLI, return `Box<dyn BufRead>` at the entrypoint: `BufReader<File>` and `StdinLock` are different concrete types so `impl BufRead` won't compile across `match` arms, while core logic stays generic over `impl BufRead`.

Below is an example of `.lines()` error handling and `ExitCode`:


### 💻 I/O error paths, stderr, and ExitCode


#### Step 1

```rust
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
```

**1. Safe Iteration Over `.lines()`**
- `.lines()` yields `Result<String>` instead of plain `String` because I/O can fail mid-stream at any moment.
- Using `?` propagates errors cleanly without panicking.


#### Step 2

```rust
// Step 2: Writing errors to `stderr` and returning `ExitCode`

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
```

**2. Separating `stdout`/`stderr` and Returning `ExitCode`**
- Printing errors with `eprintln!` protects Shell pipelines (`cat data | my_tool > output.txt`) from corrupting output files.
- Returning `ExitCode::FAILURE` from `main()` reports failure to scripts and CI/CD while ensuring `Drop` runs.



### 💻 Production CLI Patterns: File/stdin, BufWriter, and BrokenPipe


#### Step 1

```rust
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
```

**1. Unified input like `bat` and `xsv`**
- `Box<dyn BufRead>` erases the difference between `BufReader<File>` and `StdinLock`.
- `read_line` keeps `\n` at the end, so strip it with `trim_end_matches` before matching.
- `StdinLock` is `!Send` (and keeps `stdin` locked for its lifetime): to move a reader across threads, wrap `io::stdin()` in `Box<dyn BufRead + Send>`.


#### Step 2

```rust
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
```

**2. Buffered output instead of `println!` in loops**
- `stdout().lock()` acquires the mutex once, and `BufWriter` batches writes in memory.
- Calling `out.flush()?` explicitly is important because errors during `Drop` flush are ignored.


#### Step 3

```rust
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
```

**3. Playing nicely with UNIX pipes and `ExitCode`**
- Returning `ExitCode` from `main()` runs `Drop` destructors for all open files and buffers (unlike `process::exit`).
- On `ErrorKind::BrokenPipe`, the tool exits with `ExitCode::SUCCESS`, just like `ripgrep` and `bat`.


---

## Summary

Three habits make Rust I/O both fast and maintainable:
1. **Throughput**: `BufRead`, `stdin().lock()`, and `BufWriter<StdoutLock>` eliminate per-byte syscalls and repeated mutex locking.
2. **Testability**: `impl BufRead` (along with `Box<dyn BufRead>` at the CLI boundary) lets you unit-test parsers directly in RAM with `std::io::Cursor` instead of temp files.
3. **UNIX etiquette**: `eprintln!` for errors, returning `ExitCode` from `main()`, and handling `BrokenPipe` cleanly keep your CLI tools safe inside shell scripts and CI pipelines.

---

## Test Your Knowledge!

Take a quick quiz on Rust I/O and error handling:


### 📝 I/O Streams and Error Handling


**1. Why should streaming functions accept `impl BufRead` instead of `&mut File` or `StdinLock`?**

- [ ] So the compiler automatically compresses the input buffer in memory
- [ ] To decouple the logic from the data source and unit-test easily via in-memory `std::io::Cursor`
- [ ] To skip error handling when reading from disk

**2. What is the benefit of calling `std::io::stdin().lock()` before a read loop?**

- [ ] It automatically converts input bytes into `Vec<u8>` without UTF-8 validation
- [ ] It disables OS buffering so lines are read directly from disk
- [ ] It acquires the global stdin mutex once, removing per-line lock/unlock overhead

**3. Why does `.lines()` yield `Result<String>` instead of `String` on each step?**

- [ ] Reading from an external stream can fail mid-way due to I/O errors or invalid UTF-8 bytes
- [ ] Because `BufRead` always reads files backwards
- [ ] So the compiler can parallelize the loop across threads

**4. Why should CLI tools print errors via `eprintln!` and return `std::process::ExitCode` from `main()` instead of calling `process::exit(1)`?**

- [ ] Because `process::exit(1)` cannot be called inside a `match` arm
- [ ] Errors go to `stderr` without polluting `stdout` pipes, and returning `ExitCode` runs `Drop` destructors and flushes buffers cleanly
- [ ] Because `eprintln!` automatically exits the process with code `1`

**5. What gotcha should you watch out for when replacing `.lines()` with `read_line(&mut buf)` and `buf.clear()`?**

- [ ] `read_line()` automatically trims all leading and trailing whitespace
- [ ] `read_line()` ignores invalid UTF-8 sequences
- [ ] `read_line()` keeps the trailing `\n` / `\r\n` in `buf`, so you must strip it before parsing

**6. Why call `out.flush()?` explicitly on `BufWriter<StdoutLock>`, and how should `ErrorKind::BrokenPipe` be handled?**

- [ ] To reset the `StdinLock` counter, and panic immediately on `BrokenPipe`
- [ ] `Drop` ignores flush errors so explicit `flush()?` catches them; on `BrokenPipe` (e.g., `| head -n 5`) return `ExitCode::SUCCESS` quietly
- [ ] `flush()` clears the terminal screen, and `BrokenPipe` only occurs when disk is full


