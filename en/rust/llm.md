# Rust


> My personal journey learning Rust: from first experiments and basic syntax to Tokio async runtime and ESP32 embedded projects.


- **URL раздела:** https://eagle2000.space/en/rust/
- **Количество статей:** 17
- **Язык:** en

---




## 📚 Каталог статей раздела:


### 1. [Conditionals and Control Flow in Rust](https://eagle2000.space/en/rust/rust-conditionals/llm.md)

> Master branching and control flow in Rust: code blocks, if-else expressions, single-pattern matching with if let, and let-chains.

- **Дата:** 2026-07-31
- **Теги:** rust, dev
- **Markdown URL для агентов:** `https://eagle2000.space/en/rust/rust-conditionals/llm.md`


### 2. [Enums in Rust: From Syntax to Pattern Matching](https://eagle2000.space/en/rust/rust-enums/llm.md)

> In-depth overview of Rust enums: data-carrying variants, uninhabited types, the Option pattern, and match pattern matching with hands-on examples.

- **Дата:** 2026-08-01
- **Теги:** rust, dev
- **Markdown URL для агентов:** `https://eagle2000.space/en/rust/rust-enums/llm.md`


### 3. [Traits in Rust: From Fundamentals to Monomorphic Zero-Cost Code](https://eagle2000.space/en/rust/rust-traits/llm.md)

> Learn trait syntax and impl blocks, default method implementations, generic bounds, impl Trait parameters, and zero-cost monomorphization.

- **Дата:** 2026-08-07
- **Теги:** rust, dev, traits, generics, middle
- **Markdown URL для агентов:** `https://eagle2000.space/en/rust/rust-traits/llm.md`


### 4. [Memory Management in Rust: RAII, Box<T>, and Deref Coercion](https://eagle2000.space/en/rust/rust-raii-box-deref/llm.md)

> Learn how automatic resource cleanup works via Drop, allocate heap data with smart pointers, and create 'static references using Box::leak.

- **Дата:** 2026-08-12
- **Теги:** rust, dev, memory, raii, middle
- **Markdown URL для агентов:** `https://eagle2000.space/en/rust/rust-raii-box-deref/llm.md`


### 5. [Trait Objects: How Dynamic Polymorphism Works in Rust](https://eagle2000.space/en/rust/rust-dyn-traits/llm.md)

> Learn how trait objects (dyn Trait) work in Rust in simple terms: fat pointers, vtables, memory alignment, and object safety rules in practice.

- **Дата:** 2026-08-14
- **Теги:** rust, dev, traits, memory, middle
- **Markdown URL для агентов:** `https://eagle2000.space/en/rust/rust-dyn-traits/llm.md`


### 6. [Closures in Rust: Functions with Their Own Memory](https://eagle2000.space/en/rust/rust-closures/llm.md)

> A practical breakdown of environment capture, function pointers, how Fn, FnMut, and FnOnce map to struct methods, and when you need for<'a>.

- **Дата:** 2026-11-05
- **Теги:** rust, dev, closures, advanced
- **Markdown URL для агентов:** `https://eagle2000.space/en/rust/rust-closures/llm.md`


### 7. [Interior Mutability in Rust: Pattern Philosophy, Cell, and RefCell](https://eagle2000.space/en/rust/rust-interior-mutability/llm.md)

> Exploring the Interior Mutability pattern in Rust: core mechanics, Cell and RefCell containers, inherited vs interior mutability, and practical code examples.

- **Дата:** 2026-09-09
- **Теги:** rust, dev, memory, interior-mutability, refcell
- **Markdown URL для агентов:** `https://eagle2000.space/en/rust/rust-interior-mutability/llm.md`


### 8. [Choosing std::collections in Rust: Maps, Sets, and Queues](https://eagle2000.space/en/rust/rust-collections-overview/llm.md)

> A practical guide to collections in Rust: when to use VecDeque, how BTreeMap compares to HashMap, and why you almost never need LinkedList.

- **Дата:** 2026-09-18
- **Теги:** rust, dev, collections, memory
- **Markdown URL для агентов:** `https://eagle2000.space/en/rust/rust-collections-overview/llm.md`


### 9. [Rust Iterators: Lazy Data Processing Without Extra Allocations](https://eagle2000.space/en/rust/rust-iterators-deep-dive/llm.md)

> Explore how for loops desugar via IntoIterator, compare .iter() vs .into_iter(), build custom pagination, and leverage LLVM SIMD optimizations.

- **Дата:** 2026-11-13
- **Теги:** rust, dev, iterators, zero-cost-abstractions, functional
- **Markdown URL для агентов:** `https://eagle2000.space/en/rust/rust-iterators-deep-dive/llm.md`


### 10. [Rust Lifetimes Deep Dive: Elision, HRTB, and Variance](https://eagle2000.space/en/rust/rust-lifetimes-deep-dive/llm.md)

> Master Rust lifetimes: how lifetime elision works, storing references in structs, &'static vs T: 'static, higher-rank trait bounds (for<'a>), and variance.

- **Дата:** 2026-10-21
- **Теги:** rust, dev, lifetimes, memory, advanced
- **Markdown URL для агентов:** `https://eagle2000.space/en/rust/rust-lifetimes-deep-dive/llm.md`


### 11. [Pattern Matching in Rust: Destructuring, Bindings, and Guards](https://eagle2000.space/en/rust/rust-pattern-matching-deep-dive/llm.md)

> In-depth guide to pattern matching in Rust: exhaustiveness, destructuring, @-bindings in dupe-krill, slice patterns, match guards, and let chains.

- **Дата:** 2026-10-05
- **Теги:** rust, dev, pattern-matching, middle
- **Markdown URL для агентов:** `https://eagle2000.space/en/rust/rust-pattern-matching-deep-dive/llm.md`


### 12. [Advanced Traits in Rust: Associated Types and Supertraits](https://eagle2000.space/en/rust/rust-advanced-traits/llm.md)

> Learn how to use associated constants, fully qualified syntax <Type as Trait>::method, and blanket implementations in complex Rust codebases.

- **Дата:** 2026-08-18
- **Теги:** rust, dev, traits, generics, advanced
- **Markdown URL для агентов:** `https://eagle2000.space/en/rust/rust-advanced-traits/llm.md`


### 13. [Fast I/O in Rust: Buffering, In-Memory Tests, and CLI Errors](https://eagle2000.space/en/rust/rust-io-bufread-and-errors/llm.md)

> How to cut syscall overhead with BufRead, unit-test streams via std::io::Cursor, handle .lines() failures, and return clean UNIX exit codes in CLI tools.

- **Дата:** 2026-11-18
- **Теги:** rust, dev, io, errors
- **Markdown URL для агентов:** `https://eagle2000.space/en/rust/rust-io-bufread-and-errors/llm.md`


### 14. [BufReader in Rust: Accelerating I/O](https://eagle2000.space/en/rust/bufreader/llm.md)

> Learn how to speed up file reading by minimizing OS system calls, mastering BufRead vs Read, and benchmarking line-by-line streaming.

- **Дата:** 2025-08-26
- **Теги:** rust, dev
- **Markdown URL для агентов:** `https://eagle2000.space/en/rust/bufreader/llm.md`


### 15. [Rake of the Day: the server returned 202 Accepted... after 9 seconds, and the client timed out](https://eagle2000.space/en/rust/http-202-async-email-timeout/llm.md)

> Why an async email endpoint makes a mobile app crash with TimeoutException, and how to offload SMTP properly with tokio::spawn.

- **Дата:** 2026-11-25
- **Теги:** rust, tokio, axum, flutter, debugging
- **Markdown URL для агентов:** `https://eagle2000.space/en/rust/http-202-async-email-timeout/llm.md`


### 16. [Speeding Up the Speedable: SIMD and What GTA Has to Do With It](https://eagle2000.space/en/rust/rust-simd-and-gta/llm.md)

> How SIMD works under the hood, why a JSON parsing bug stalled GTA Online for 5-10 minutes, how simdjson hits gigabytes per second, and four ways to use SIMD in Rust.

- **Дата:** 2026-11-23
- **Теги:** rust, simd, performance, optimization
- **Markdown URL для агентов:** `https://eagle2000.space/en/rust/rust-simd-and-gta/llm.md`


### 17. [Crate of the Day: Strum - iterating and converting Rust Enum](https://eagle2000.space/en/rust/crate-of-the-day-strum/llm.md)

> Exploring the popular Strum crate: how to iterate over enum variants, convert enums to strings and back, using a real-world example from esp-generate.

- **Дата:** 2026-08-16
- **Теги:** rust, crate-of-the-day, enums, macros, esp32
- **Markdown URL для агентов:** `https://eagle2000.space/en/rust/crate-of-the-day-strum/llm.md`


