# Rust Iterators: Lazy Data Processing Without Extra Allocations

> A deep dive into Rust iterators: how for loops desugar with IntoIterator, .iter() vs .iter_mut() vs .into_iter(), lazy pipelines, custom pagination iterators, and LLVM SIMD optimizations.

- **URL:** https://eagle2000.space/en/rust/rust-iterators-deep-dive/
- **Date:** 2026-11-13

- **Tags:** rust, dev, iterators, zero-cost-abstractions, functional

---
Last time I covered [closures in Rust](../rust-closures/) and how the compiler moves captured variables into anonymous structs. Today let's look at iterators. Together with closures, they give you short chains like `.filter().map().collect()` that compile down to a plain loop with no extra heap allocations.

In JavaScript, `arr.filter(...).map(...)` allocates a fresh intermediate array on every step and feeds the garbage collector; in Python, even lazy `map()` and `filter()` wrap every step in interpreter calls and heap objects. When I first moved to Rust from Go, I kept writing manual loops with `let mut out = Vec::new()` out of habit, worried about overhead. In reality, Rust does the opposite: an iterator chain allocates nothing on the fly and compiles into a tight machine loop.

Let's walk through how it works: what `for` actually compiles into, how lazy pipelines pull elements, when `std::iter::from_fn` is enough, how to return `impl Iterator` from functions, and when a plain `for` loop is still easier to read.

> 📦 **Companion Code Repository:** All runnable examples from this episode are open-sourced on GitHub: [github.com/eaglebk/rust-iterators-examples](https://github.com/eaglebk/rust-iterators-examples).

{{< alert "comment" >}}
**What is a lazy pipeline?** In short, it is a plan of work without doing the work yet. In eager code (like `arr.filter().map()` in JS or list comprehensions `[... for x in arr]` in Python), each step processes the whole array and stores the result in a temporary list. In Rust, `.filter(...).map(...)` touches zero elements: it just records what to do with the data. Actual work starts only when you ask for a result (via `.collect()`, `.sum()`, or a `for` loop), and items flow through the entire chain one at a time with no intermediate arrays. We will look under the hood in Section 2.
{{< /alert >}}

---

## 1. What `for` Desugars Into and How `IntoIterator` Works

A familiar `for item in collection` loop in Rust is syntactic sugar. The compiler expands it (slightly simplified here, without the outer `match` scope for temporaries) into a `loop` calling `IntoIterator::into_iter`:

```rust
// What you write:
for item in names {
    println!("{item}");
}

// What the compiler sees:
let mut iter = IntoIterator::into_iter(names);
loop {
    match iter.next() {
        Some(item) => {
            println!("{item}");
        }
        None => break,
    }
}
```

{{< flow-stepper src="flows/for-desugar.en.json" >}}

This explains the difference between the three ways to iterate over a collection. `Vec<T>` (like most containers in `std`) implements `IntoIterator` three times: for `Vec<T>` by value, for `&Vec<T>`, and for `&mut Vec<T>`.

### `.iter()` or `for item in &vec`

`.iter()` creates an iterator yielding shared references `&T`. The vector stays intact, so you can keep reading it after the loop:

```rust
let names = vec!["Alice".to_string(), "Bob".to_string()];

for name in names.iter() {
    println!("Reading: {name}");
}
// `names` is still alive and usable
println!("Total names: {}", names.len());
```

### `.iter_mut()` or `for item in &mut vec`

`.iter_mut()` yields mutable references `&mut T`, letting you update elements in place without allocating a new vector:

```rust
let mut scores = vec![10, 20, 30];

for score in scores.iter_mut() {
    *score += 5;
}
// scores is now [15, 25, 35]
```

### `.into_iter()` or `for item in vec`

`.into_iter()` consumes the collection and hands you owned `T` values. The vector's buffer is freed, while the strings or structs move straight into the loop body:

```rust
let names = vec!["Alice".to_string(), "Bob".to_string()];

for name in names.into_iter() {
    // Here `name` has type String, not &String
    drop(name);
}
// Using `names` here fails to compile: value moved
```

{{< alert "fire" >}}
**Common `for` loop gotcha:** beginners often trip over `for item in items` without an ampersand. That loop silently calls `.into_iter()` and takes ownership of the vector, so the compiler rejects any use of `items` afterward. If you still need the collection after the loop, write `for item in &items`.
{{< /alert >}}

---

## 2. The Lazy Pull Model: Adapters Wait for a Consumer

Iterators in Rust follow a **pull model**, not a push model. When you write `.filter(...).map(...)`, no data moves anywhere. Instead, each adapter wraps the previous iterator in a new struct on the stack.


### 💻 Checking laziness and stack size


```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // No work done yet! This only builds a Map<Iter<'_, i32>, ...> struct on the stack
    let doubled = numbers.iter().map(|x| {
        println!("Processing {x}");
        x * 2
    });

    println!("Stack size: {} bytes", std::mem::size_of_val(&doubled));
    println!("Actual type: {}", std::any::type_name_of_val(&doubled));
}
```

**Checking laziness and stack size**
- Run the code: no "Processing ..." lines are printed because no consuming method is called.
- `doubled` takes 16 bytes on the stack, and its type reveals the nested `Map<Iter<'_, i32>, ...>` wrapper.


This quick test clears up any worries about hidden allocations: `size_of_val(&doubled)` prints 16 bytes on the stack (two slice pointers, while the non-capturing closure takes 0 bytes), and `type_name_of_val` prints the nested type `core::iter::adapters::map::Map<core::slice::iter::Iter<i32>, ...>`. If you never consume `doubled`, nothing is printed from the closure, and the compiler warns you because `Iterator` is marked `#[must_use = "iterators are lazy and do nothing unless consumed"]`.

Computation only starts when you call a **consuming method** that drives `.next()` in a loop:

- `.collect()` gathers items into a collection (`Vec`, `HashSet`, `HashMap`, `String`) and can even stop on the first error with `collect::<Result<Vec<_>, _>>()` (I wrote more about `FromIterator` in the post on [`collect()`](../rust-collect/)). If you want to skip bad items instead of failing, use `.filter_map(|s| s.parse::<i32>().ok())`.
- `.fold()` and `.reduce()` combine elements into a single value using an accumulator (`try_fold` adds early exit on `Result` or `Option`).
- `.sum()` and `.product()` compute the sum or product.
- `.find()`, `.any()`, and `.all()` stop as soon as they find a match or fail the condition (**short-circuiting**).
- `.for_each()` runs a closure for side effects like logging or sending to a channel.

Because of this pull model, `huge_vec.iter().filter(...).map(...).take(3).collect()` processes only as many elements as needed to produce the first 3 results, and never touches the remaining million items.

{{< flow-stepper src="flows/iterator-pull.en.json" >}}

---

## 3. Writing Your Own `Iterator`: Retry Delays and Lazy Pagination

To make any custom type work with `for` loops and all standard combinators, you only need to implement `std::iter::Iterator`:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

One associated type `Item` and one `next` method. In return, you get 70+ methods for free: `map`, `filter`, `zip`, `enumerate`, `skip`, `take`, `fold`, `collect`, and more.

### Example 1: Exponential Backoff Generator

When a service talks to an external API or database, transient failures are retried with exponential backoff: 100 ms, 200 ms, 400 ms, capped at a max delay and an attempt limit. This fits naturally into an `Iterator` with `saturating_mul` to guard against overflow:

```rust
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
        if self.remaining == 0 { {{< golink id="rb1" />}}
            return None;
        }
        self.remaining -= 1;
        let delay = self.current.min(self.max_delay);
        self.current = self.current.saturating_mul(2).min(self.max_delay); {{< golink id="rb2" />}}
        Some(delay)
    }
}

impl FusedIterator for RetryBackoff {}
```

Two lines drive the behavior: checking `remaining == 0` ({{< codelink id="rb1" />}}) ensures the iterator stays empty once attempts run out, and `saturating_mul(2).min(self.max_delay)` ({{< codelink id="rb2" />}}) doubles the delay safely without overflowing or exceeding the cap. Under a plain `Iterator`, calling `.next()` after `None` could in theory yield `Some(...)` again; `FusedIterator` promises it will stay `None` forever. For iterators without that guarantee, there is `.fuse()`.

### The Shortcut: `std::iter::from_fn` Instead of a Struct

Most of the time you do not need a dedicated struct and a manual `impl Iterator`. If the generator is simple and you do not need to name its type in a public API, `std::iter::from_fn` (or `std::iter::successors` for simple recurrence sequences) is much shorter.


### 💻 Batching with std::iter::from_fn


```rust
fn main() {
    // Split ID range [0..100) into batches of 25
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

    let ranges: Vec<_> = batches.collect();
    println!("Batches: {:?}", ranges);
}
```

**Batch generator with std::iter::from_fn**
- The closure captures `cursor` and yields the next `Range<u64>` on each `.next()` call.


Reach for a separate struct when the iterator holds complex state, needs extra traits (`DoubleEndedIterator`, `ExactSizeIterator`), or must be exposed by type name in a public API.

### Example 2: Lazy Pagination Iterator

In real codebases, custom iterators are rarely about math — they usually wrap paginated APIs, database cursors, or chunked disk reads. Instead of fetching 50 pages into a giant `Vec`, you can wrap the cursor in an `Iterator` backed by a small `VecDeque` buffer:

```rust
use std::collections::VecDeque;
use std::iter::FusedIterator;

struct PageIter {
    next_page: usize,
    buffer: VecDeque<String>,
    done: bool,
}

impl Iterator for PageIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while self.buffer.is_empty() && !self.done { {{< golink id="pg1" />}}
            let items = fetch_page(self.next_page);
            if items.is_empty() {
                self.done = true;
            } else {
                self.next_page += 1;
                self.buffer.extend(items);
            }
        }
        self.buffer.pop_front() {{< golink id="pg2" />}}
    }
}

impl FusedIterator for PageIter {}
```

As long as the local `VecDeque` has items, the iterator pops them with `pop_front()` ({{< codelink id="pg2" />}}) without touching the network. Once the buffer empties, the `while` loop ({{< codelink id="pg1" />}}) fetches just one more page. So `PageIter::new().take(3).collect()` (a full runnable version is in the interactive slides at the end) downloads only the pages needed for the first three records and stops.

Keep in mind that `Iterator` is synchronous: you cannot `.await` inside `next()`. For async pagination in `tokio`, reach for `futures::Stream` and the `async-stream` crate.

### Three Useful Companion Traits

When writing your own iterators, three standard traits are worth knowing:

1. **`DoubleEndedIterator`** adds `next_back()`, letting you iterate from both ends and unlocking `.rev()` and `.rfind()`.
2. **`ExactSizeIterator`** adds `len()` when the exact remaining count is known upfront. That lets `.collect::<Vec<_>>()` allocate once via `Vec::with_capacity(len)` instead of reallocating as the vector grows (`size_hint` handles approximate estimates).
3. **`IntoIterator`** can be implemented on your own wrapper collections so callers can write `for x in &my_collection`.

{{< alert "lightbulb" >}}
**`ExactSizeIterator` subtlety:** `.map()` preserves the exact length (so `collect::<Vec<_>>()` allocates in one shot), whereas `.filter()` drops it because the compiler cannot know ahead of time how many items will pass the predicate.
{{< /alert >}}

### Returning a Lazy Iterator (`-> impl Iterator`)

Instead of building an intermediate `Vec<T>` inside a helper function, return a lazy iterator and let the caller decide whether to collect everything or stop on the first match:

```rust
fn non_empty_lines(text: &str) -> impl Iterator<Item = &str> + '_ {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
}
```

Notice the `+ '_`: the returned iterator borrows from `text`, so in Rust 2021 and earlier you tell the compiler it cannot outlive the input string (in Rust 2024 edition, `impl Trait` captures input lifetimes automatically, so `+ '_` is optional). No `Vec` is allocated inside the function.

---

## 4. Zero-Cost Abstractions: Iterators vs. Index Loops

Anyone coming from C# or Java is understandably skeptical of long method chains: while LINQ and Java Streams are also lazy, their adapters allocate objects on the heap, dispatch through interfaces, and often box primitives, losing out to a plain `for` loop on hot paths.

Rust flips that around for two codegen reasons:

1. **Monomorphization and inlining.** Each adapter (`Filter<Iter<i32>, F>`, `Map<..., G>`) is a concrete type at compile time, not a `dyn Iterator` vtable. The compiler sees the whole pipeline, inlines every `.next()` and closure together, and flattens the chain into a single loop.
2. **Bounds check elimination.** Compare two ways to sum a slice:

```rust
// Approach 1: manual indexing
let mut sum = 0;
for i in 0..slice.len() {
    sum += slice[i];
}

// Approach 2: slice iterator
let sum: i32 = slice.iter().sum();
```

Every `slice[i]` access requires Rust to guarantee the index is in bounds. For a simple `0..slice.len()`, LLVM can usually prove it safe, but add an offset like `slice[i + 1]` or a stride of two and bounds checks with panic branches creep back in. Idiomatic `slice.windows(2)` or `slice.chunks_exact(2)` solve that without a single bounds check inside the loop.

Under the hood, `std::slice::Iter` holds two raw pointers — current and end — and simply advances the pointer inside an `unsafe` wrapper whose safety was already proven when the slice was created. No per-iteration bounds checks, and a branch-free linear loop is easy for LLVM to auto-vectorize with SIMD.

{{< code-before-after before_label="Rust (iterator chain)" after_label="Assembly (ARM64, rustc -O)" badge="rustc -O" caption="Actual rustc -O output: .filter().map().sum() compiles into a branchless SIMD loop with zero function calls or bounds checks" >}}
```rust
#[no_mangle]
pub fn sum_pos(s: &[i32]) -> i32 {
    s.iter()
        .copied()
        .filter(|&x| x > 0)
        .map(|x| x * 2)
        .sum()
}
```
===
```asm
.LBB0_5:
    ldp     q3, q4, [x8, #-16]
    subs    x10, x10, #8
    add     x8, x8, #32
    smax    v3.4s, v3.4s, v0.4s
    smax    v4.4s, v4.4s, v0.4s
    add     v3.4s, v3.4s, v3.4s
    add     v4.4s, v4.4s, v4.4s
    add     v1.4s, v3.4s, v1.4s
    add     v2.4s, v4.4s, v2.4s
    b.ne    .LBB0_5
```
===
.iter() ;; s.iter()||.copied() ;; ldp||q3, q4||add     x8, x8, #32 ;; Loads 8 integers from memory in a single pass with zero per-element bounds checks.
.filter() ;; .filter(|&x| x > 0) ;; smax    v3.4s||smax    v4.4s ;; Instead of a branchy if that stalls the CPU branch predictor, negative values are clamped to 0 with one vector instruction.
.map() ;; .map(|x| x * 2) ;; add     v3.4s, v3.4s, v3.4s||add     v4.4s, v4.4s, v4.4s ;; Multiplying by 2 becomes a fast self-add across 4 elements in a single vector instruction.
.sum() ;; .sum() ;; add     v1.4s||add     v2.4s||subs||b.ne ;; Accumulates the sum across two parallel register accumulators in a tight loop with no calls or allocations.
{{< /code-before-after >}}

You can inspect the full generated function (including the length check and the loop for any leftover items) on [Godbolt (`rustc 1.95.0`, `--edition 2021 -O --target aarch64-unknown-linux-gnu`)](https://godbolt.org/z/Kjh19eMv8).

This listing shows the main loop processing 8 numbers per iteration (if the slice length is not a multiple of 8, the compiler finishes the remainder in a small scalar tail loop). Two practical caveats to keep in mind:

- **Floats (`f32` / `f64`) do not auto-vectorize the same way.** Integers can be added in any order and always give the exact same sum. Floating-point addition can change rounding slightly when reordered, and the compiler will not silently alter precision, so `.sum::<f64>()` adds numbers sequentially unless you opt into fast-math semantics.
- **Iterators are single-threaded.** All that SIMD speedup happens on a single CPU core. If you have a heavy workload and want all cores busy, pull in the `rayon` crate and swap `.iter()` for `.par_iter()`.

---

## 5. Four Common Iterator Pitfalls

### Double References `&&T` in `.filter()`

If you have ever written `nums.iter().filter(|&&x| x > 0)`, you probably wondered where the two ampersands came from. The reason is simple:
- `nums.iter()` yields `&i32`.
- `.filter()` cannot pass items by value into the predicate, otherwise it could not forward passing items downstream. So the predicate always receives `&Self::Item`, which turns `&i32` into `&&i32`.

To skip double dereferencing on simple `Copy` types like integers, drop `.copied()` in front (or `.cloned()` for `Clone` types):

```rust
let sum: i32 = nums.iter().copied().filter(|&x| x > 0).sum();
```

### Mutating a Collection While Iterating

The borrow checker rejects code like this:

```rust
let mut items = vec![1, 2, 3];
for &x in &items {
    if x == 2 {
        items.push(99); // Error: cannot borrow `items` as mutable while `&items` is active
    }
}
```

{{< alert "comment" >}}
**Why the borrow checker blocks this:** in C++, pushing into a vector during iteration can reallocate the heap buffer and leave the iterator's pointers dangling (*iterator invalidation*). In Rust, use `items.retain(|&x| x != 2)` to filter in place, `items.extract_if(.., |x| *x == 2)` (stabilized in Rust 1.87) to pull matching items out, or collect additions into a separate buffer and merge them afterward with `items.extend(new_items)`.
{{< /alert >}}

### Losing the Iterator After `.take(n)`: Use `.by_ref()`

Most adapters (`take`, `skip`, `map`) take `self` by value and consume the iterator. If you want to grab a few header items and then keep reading from the exact same iterator, borrow it mutably with `.by_ref()`.


### 💻 Taking a prefix with .by_ref()


```rust
fn main() {
    let mut it = [10, 20, 30, 40, 50].into_iter();
    let header: Vec<i32> = it.by_ref().take(2).collect();
    let rest: Vec<i32> = it.collect();

    println!("header = {:?}, rest = {:?}", header, rest);
}
```

**Taking a prefix with .by_ref()**
- `take(2)` borrows `it` mutably instead of consuming it, so `it` still yields `[30, 40, 50]` afterward.


### Infinite Iterators Without a Bound

If a custom iterator always returns `Some(...)` from `next()` (like an infinite counter `0..`), calling `.collect()`, `.count()`, or `.last()` will hang forever. Always cap infinite generators with `.take(n)` or `.take_while(...)` before consuming them (`.find(...)` or `.any(...)` only terminate if a matching item is guaranteed to appear).

---

## 6. When to Write `for` vs. an Iterator Chain

Iterators do not have to replace every loop in your codebase. I follow a simple rule of thumb:

- **I pick an iterator chain** when the task reads like a linear pipeline (filtering, mapping, searching, folding into a collection or sum) without extra `let mut` state.
- **I pick a plain `for` loop** when the body needs complex early exits (`return`, `break` to an outer label), multiple `?` error returns, or updates to several local variables at once. Cramming that into `try_fold` only hurts readability.

You can run all of these examples — borrowing modes, `from_fn`, and lazy pagination — right in the browser:


### 💻 Rust Iterators: Internals and Practice


#### Step 1

```rust
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
```

**Three iteration modes and IntoIterator**
- `.iter()` (or `for x in &vec`) yields `&T` and leaves the collection intact.
- `.iter_mut()` (or `for x in &mut vec`) yields `&mut T` to mutate items in place.
- `.into_iter()` (or `for x in vec`) takes ownership of the collection and yields `T`.


#### Step 2

```rust
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
```

**Lazy evaluation and short-circuiting**
- Calling `.map()`, `.inspect()`, and `.filter()` only wraps the iterator in a new struct.
- Work starts only when a consuming method (`collect`, `sum`, `find`) is called.
- `collect::<Result<Vec<_>, _>>()` halts on the first `Err` without touching the tail.


#### Step 3

```rust
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
```

**Custom Iterator: Exponential Backoff**
- Implementing `Iterator` only requires `type Item` and `fn next(&mut self) -> Option<Self::Item>`.
- `saturating_mul` and `max_delay` prevent overflow, while `remaining == 0` enables `FusedIterator`.


#### Step 4

```rust
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
```

**In practice: lazy pagination iterator**
- The iterator fetches the next page only when its local `VecDeque` buffer is empty.
- Thanks to `.take(3)`, no extra network or disk requests are made for the remaining pages.


#### Step 5

```rust
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
```

**Standard library idioms: from_fn, impl Iterator, and windows**
- `std::iter::from_fn` replaces a separate `struct + impl Iterator` for small generators.
- `-> impl Iterator<Item = &str> + '_` returns a lazy stream without heap-allocating a `Vec`.
- `slice.windows(2)` replaces manual `slice[i + 1]` indexing without bounds checks.


---

## Test Your Knowledge

A quick quiz to lock in `IntoIterator`, lazy pipelines, double references, and how the optimizer handles iterators.


### 📝 Rust Iterators


**1. What is the difference between `.iter()` and `.into_iter()` on a `Vec<T>`?**

- [ ] `.iter()` borrows items by reference `&T`, while `.into_iter()` consumes the vector and yields owned `T` values
- [ ] `.into_iter()` yields `&T` references, but prevents mutating the vector until the end of the function
- [ ] `.iter()` clones each element of the vector, while `.into_iter()` reads them without copying

**2. What happens to a vector `items` after `for item in items { ... }`?**

- [ ] The vector remains readable, but can no longer be mutated
- [ ] A `for` loop by value calls `IntoIterator::into_iter(items)`, moving the vector so it is no longer usable afterward
- [ ] The compiler automatically borrows the vector as `&items`

**3. What happens if you write `numbers.iter().map(|x| x * 2);` as a standalone statement without calling a consuming method (`collect`, `sum`, `for_each`)?**

- [ ] No computation happens, and the compiler emits a `#[must_use]` warning about the unused iterator
- [ ] The closure runs once for the first item to check types, then stops
- [ ] The elements are computed immediately and stored in a temporary stack buffer

**4. Which single method must be implemented when manually writing `impl Iterator`?**

- [ ] `fn next(&mut self) -> Option<Self::Item>`
- [ ] `fn size_hint(&self) -> (usize, Option<usize>)`
- [ ] `fn into_iter(self) -> Self`

**5. Why do double ampersands `&&x` appear in `vec.iter().filter(|&&x| x > 0)`?**

- [ ] `.iter()` yields `&T`, and `.filter()` passes `&Self::Item` to the predicate so it does not take ownership before filtering
- [ ] Double references only happen for `Copy` types; `String` receives a single `&String`
- [ ] The first `&` borrows `vec` itself, and the second `&` borrows the current element

**6. What does `iter.collect::<Result<Vec<_>, _>>()` do if the third element is an `Err(...)`?**

- [ ] Skips the error and keeps collecting the remaining `Ok` values
- [ ] Stops iterating immediately and returns that first `Err` without processing the rest
- [ ] Processes the whole iterator and returns a tuple of `Ok` values and the first error

**7. Why can an iterator chain (for instance with filtering or `.windows(2)`) match or beat a manual index loop in speed?**

- [ ] Standard adapters automatically split iterations across a thread pool
- [ ] Slice iterators advance raw pointers without per-element bounds checks, and monomorphization + inlining collapse the chain into a flat loop
- [ ] The compiler evaluates iterator chains at compile time via `const eval`

**8. What does the `FusedIterator` marker trait guarantee?**

- [ ] That the iterator can be safely sent across threads
- [ ] That once `.next()` returns `None`, all subsequent `.next()` calls will continue returning `None`
- [ ] That two adjacent `.map()` adapters were merged into a single closure


