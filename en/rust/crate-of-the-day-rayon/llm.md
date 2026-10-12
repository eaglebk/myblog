# Crate of the Day: Rayon - Data Parallelism in a Single Line of Code

> Exploring the legendary Rayon crate: how changing a single line from .iter() to .par_iter() parallelizes computation across all CPU cores using Work-Stealing.

- **URL:** https://eagle2000.space/en/rust/crate-of-the-day-rayon/
- **Date:** 2026-08-15

- **Tags:** rust, crate-of-the-day, concurrency, performance

---
{{< lead >}}
"Crate of the Day" — concise breakdowns of popular (and hidden gem) Rust ecosystem libraries that save hundreds of engineering hours.
{{< /lead >}}

Today's featured crate — **`rayon`** (one of the most elegant and widely-used crates in the Rust community).

### 🛠 Problem It Solves

Imagine you have a huge array of 10,000,000 elements and need to run a CPU-intensive operation on every item.

In standard Rust code, you write a regular iterator:

```rust
let results: Vec<_> = data.iter()
    .map(|x| heavy_computation(x))
    .collect();
```

This code executes strictly inside a **single thread**. If you run this on a 16-core CPU, 15 cores remain completely idle.

Spawning manual threads with `thread::spawn`, `Arc<Mutex<T>>`, and channels takes significant boilerplate and increases risk of synchronization bugs.

---

### 🚀 Solution via Rayon

With the `rayon` crate, parallelizing your iterator pipeline comes down to changing **a single method name**:

```rust
use rayon::prelude::*; // 1. Import Rayon traits

// 2. Change .iter() to .par_iter()
let results: Vec<_> = data.par_iter()
    .map(|x| heavy_computation(x))
    .collect();
```

---

### 💡 Why Engineers Love Rayon

1. **Work-Stealing Scheduler:** Rayon automatically spawns a global thread pool matched to your physical CPU core count. If one worker thread finishes its slice of work early, it automatically "steals" uncompleted tasks from neighboring thread queues to keep CPU utilization at 100%.
2. **Compile-Time Safety Guarantees:** Built on top of Rust's `Send` and `Sync` traits, if your type or closure payload is not thread-safe, the Rust compiler will prevent calling `.par_iter()` at compile time.

🔗 **GitHub Repository:** [rayon-rs/rayon](https://github.com/rayon-rs/rayon)  
📦 **Crates.io:** [crates.io/crates/rayon](https://crates.io/crates/rayon)

