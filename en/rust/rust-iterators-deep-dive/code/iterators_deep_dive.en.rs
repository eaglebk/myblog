fn main() {
    let mut numbers = vec![10, 20, 30];

    // 1. Borrow by &T (equivalent to `for num in &numbers`)
    for num in numbers.iter() {
        println!("By ref (&T): {num}");
    }

    // 2. Mutate in place via &mut T (equivalent to `for num in &mut numbers`)
    for num in numbers.iter_mut() {
        *num += 5;
    }
    println!("After iter_mut: {:?}", numbers);

    // 3. Consume the collection: `for num in numbers` does the same
    for num in numbers.into_iter() {
        println!("Owned (T): {num}");
    }
    // `numbers` has been moved here and can no longer be used
}

// ! **Three iteration modes and IntoIterator**
// ! - `.iter()` (or `for x in &vec`) yields `&T` and leaves the collection intact.
// ! - `.iter_mut()` (or `for x in &mut vec`) yields `&mut T` to mutate items in place.
// ! - `.into_iter()` (or `for x in vec`) takes ownership of the collection and yields `T`.

// ---

fn main() {
    let raw_inputs = ["10", "20", "bad_input", "40"];

    // The chain is lazy: no closures run until a consuming method is called
    let lazy_pipeline = raw_inputs
        .iter()
        .inspect(|s| println!("Reading: {s}"))
        .map(|s| s.parse::<i32>());

    println!("Pipeline created, nothing computed yet!");

    // collect::<Result<Vec<_>, _>>() stops on the first Err and never reaches "40"
    let parsed: Result<Vec<i32>, _> = lazy_pipeline.collect();
    println!("Parsed result: {:?}", parsed);

    // .copied() removes one layer of reference (&i32 -> i32)
    let nums = [1, -2, 3, 4];
    let positive_sum: i32 = nums.iter().copied().filter(|&x| x > 0).sum();
    println!("Positive sum: {positive_sum}");
}

// ! **Lazy evaluation and short-circuiting**
// ! - Calling `.map()`, `.inspect()`, and `.filter()` only wraps the iterator in a new struct.
// ! - Work starts only when a consuming method (`collect`, `sum`, `find`) is called.
// ! - `collect::<Result<Vec<_>, _>>()` halts on the first `Err` without touching the tail.

// ---

use std::iter::FusedIterator;
use std::time::Duration;

struct RetryBackoff {
    current: Duration,
    max_delay: Duration,
    remaining: usize,
}

impl RetryBackoff {
    fn new(initial: Duration, max_delay: Duration, attempts: usize) -> Self {
        Self { current: initial, max_delay, remaining: attempts }
    }
}

impl Iterator for RetryBackoff {
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let delay = self.current.min(self.max_delay);
        self.current = self.current.saturating_mul(2).min(self.max_delay);
        Some(delay)
    }
}

// Guarantee that once exhausted, the iterator always returns None
impl FusedIterator for RetryBackoff {}

fn main() {
    let attempts = 5;
    let schedule: Vec<Duration> =
        RetryBackoff::new(Duration::from_millis(100), Duration::from_millis(500), attempts)
            .collect();
    println!("Retry schedule: {:?}", schedule);
}

// ! **Custom Iterator: Exponential Backoff**
// ! - Implementing `Iterator` only requires `type Item` and `fn next(&mut self) -> Option<Self::Item>`.
// ! - `saturating_mul` and `max_delay` prevent overflow, while `remaining == 0` enables `FusedIterator`.

// ---

use std::collections::VecDeque;
use std::iter::FusedIterator;

// Simulated remote API returning 2 items per page
fn fetch_page(page: usize) -> Vec<String> {
    match page {
        p if p == 0 => vec!["user_1".into(), "user_2".into()],
        p if p == 1 => vec!["user_3".into(), "user_4".into()],
        p if p == 2 => vec!["user_5".into()],
        _ => vec![],
    }
}

struct PageIter {
    next_page: usize,
    buffer: VecDeque<String>,
    done: bool,
}

impl PageIter {
    fn new() -> Self {
        Self { next_page: 0, buffer: VecDeque::new(), done: false }
    }
}

impl Iterator for PageIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while self.buffer.is_empty() && !self.done {
            println!("-> Fetching page {}", self.next_page);
            let items = fetch_page(self.next_page);
            if items.is_empty() {
                self.done = true;
            } else {
                self.next_page += 1;
                self.buffer.extend(items);
            }
        }
        self.buffer.pop_front()
    }
}

impl FusedIterator for PageIter {}

fn main() {
    // Only fetch the first 3 records: page 2 is never requested!
    let first_three: Vec<String> = PageIter::new().take(3).collect();
    println!("Fetched records: {:?}", first_three);
}

// ! **In practice: lazy pagination iterator**
// ! - The iterator fetches the next page only when its local `VecDeque` buffer is empty.
// ! - Thanks to `.take(3)`, no extra network or disk requests are made for the remaining pages.

// ---

// Return a lazy iterator without allocating an intermediate Vec
fn non_empty_lines(text: &str) -> impl Iterator<Item = &str> + '_ {
    text.lines().map(str::trim).filter(|line| !line.is_empty())
}

fn main() {
    // 1. std::iter::from_fn - batching IDs without a custom struct
    let mut cursor = Some(0u64);
    let batches = std::iter::from_fn(move || {
        let start = cursor?;
        if start >= 100 {
            cursor = None;
            return None;
        }
        let end = (start + 25).min(100);
        cursor = Some(end);
        Some(start..end)
    });
    println!("from_fn stack size: {} bytes", std::mem::size_of_val(&batches));
    println!("ID batches: {:?}", batches.collect::<Vec<_>>());

    // 2. Lazy line filtering without allocations
    let log = "\n  INFO start \n\n  WARN disk 85% \n";
    let lines: Vec<_> = non_empty_lines(log).collect();
    println!("Log lines: {:?}", lines);

    // 3. Sliding window windows(2) instead of manual slice[i + 1]
    let temps = [18, 21, 19, 24];
    let diffs: Vec<i32> = temps.windows(2).map(|w| w[1] - w[0]).collect();
    println!("Temp deltas: {:?}", diffs);
}

// ! **Standard library idioms: from_fn, impl Iterator, and windows**
// ! - `std::iter::from_fn` replaces a separate `struct + impl Iterator` for small generators.
// ! - `-> impl Iterator<Item = &str> + '_` returns a lazy stream without heap-allocating a `Vec`.
// ! - `slice.windows(2)` replaces manual `slice[i + 1]` indexing without bounds checks.

