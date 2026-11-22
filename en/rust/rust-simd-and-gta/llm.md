# Speeding Up the Speedable: SIMD and What GTA Has to Do With It

> How SIMD works under the hood, why a JSON parsing bug stalled GTA Online for 5-10 minutes, how simdjson hits gigabytes per second, and four ways to use SIMD in Rust.

- **URL:** https://eagle2000.space/en/rust/rust-simd-and-gta/
- **Date:** 2026-11-23

- **Tags:** rust, simd, performance, optimization

---
{{< lead >}}
How a single CPU instruction can crunch an entire batch of data at once, and why the infamous GTA Online loading bug is a great reminder of what bad parsing costs.
{{< /lead >}}

In this post I will walk through the basics of vector processing (SIMD), look at real-world examples like `simdjson`, and cover the ways you can work with SIMD in Rust. Fair warning: there is a bit of theory here, but it sets up the practical part nicely.

---

## What GTA Online Has to Do With It

Before diving into CPU instructions, let's recall the story of an enthusiast named `t0st` who fixed a long-standing GTA Online bug in 2021 that kept players staring at loading screens for 5-10 minutes.

Rockstar's mistake happened during JSON parsing. While reading a 10 MB JSON file, C's `sscanf` kept calling `strlen` on every single step to recompute the remaining string length. A routine operation ended up burning CPU cycles on redundant work.

Rockstar confirmed the root cause and awarded the researcher a \$10,000 bounty (you can read the full story on [Ars Technica](https://arstechnica.com/gaming/2021/03/developers-to-update-gta-online-to-address-poor-load-times-after-community-fix/)).

I bring this up for a reason: it shows how a poorly implemented low-level operation can bottleneck an entire application.

---

## Speeding Up JSON: The simdjson Approach

When JSON parsing performance comes up, [simdjson](https://github.com/simdjson/simdjson) (by Daniel Lemire and Geoff Langdale) is hard to ignore.

At first glance, JSON looks like a poor fit for vectorization. Standard sequential parsing depends on preceding characters: you have to track quotes, `\` escapes, and nested objects.

The `simdjson` authors split the work into two stages:

1. **Structural indexing with SIMD.** Vector instructions scan large chunks of text in parallel, locate structural characters, find string boundaries, and handle escape sequences.
2. **Document parsing.** The second stage builds the actual structure using the already indexed positions.

As a result, `simdjson` parses JSON at several gigabytes per second, outperforming many traditional parsers and even some binary format implementations.

---

## What SIMD Is and How It Works

To see where that speedup comes from, let's drop down to machine instructions.

A machine instruction is a basic command a CPU executes directly: add two numbers, compare values, or load data from memory into a register.

For a long time, most code relied on **scalar operations** that handle one value at a time, such as the classic `ADD EAX, EBX` instruction adding two 32-bit integers. Over time, CPUs gained instruction set extensions: **SSE**, **AVX**, **AVX2**, **AVX-512** on x86, and **NEON** on ARM. These brought vector instructions to everyday processors.

**SIMD** stands for *Single Instruction, Multiple Data*.

With SIMD, the CPU operates on multiple values in parallel. Instead of comparing bytes one by one, it can compare 16, 32, or 64 bytes in a single vector instruction.

The exact element count depends on register width and element size:

- **128-bit** holds 16 bytes (or four 32-bit `f32` floats);
- **256-bit** holds 32 bytes (or eight `f32` floats);
- **512-bit** holds 64 bytes (or sixteen `f32` floats).

### How It Looks on the Hardware

The idea is straightforward: instead of loading a single number into a register, the CPU loads an entire vector. A 128-bit SSE register fits four `f32` values, and one vector instruction adds all four pairs at once.

This is not four scalar additions bundled together as syntax sugar. It is **one vector instruction** running on dedicated vector registers and SIMD execution units inside a single core, with no extra threads or cores required.

{{< flow-stepper src="flows/simd-flow.en.json" >}}

Of course, SIMD pairs well with multithreading. Multithreading spreads work across multiple CPU cores, while SIMD speeds up uniform operations inside each core. They complement each other rather than competing.

---

## Where to Look for SIMD Opportunities

Spotting vectorization candidates in existing code is fairly straightforward. Whenever you see loops like this (pseudocode):

```text
for (byte in bytes) { ... }
for (item in array) { ... }
for (char in string) { ... }
```

it is worth taking a closer look. Loops where each iteration handles independent elements are prime targets: adding two arrays, applying a filter, or scanning byte sequences.

Conceptually, processing one element per step turns into processing a whole chunk:

```text
for (8 elements in array) { ... }
```

Not every loop can be vectorized, though:

- **Loop-carried dependencies.** If an iteration depends on the previous iteration's result, vectorization may be impossible or counterproductive.
- **Small datasets.** SIMD shines on hundreds of thousands or millions of elements. On tiny slices, setup overhead can wipe out any gain.
- **Memory bottlenecks.** If the program spends most of its time waiting on RAM, faster arithmetic will barely move the needle.

Always verify SIMD changes with benchmarks. Today, vector instructions are everywhere: image and audio processing, digital signal processing (DSP), matrix math, cryptography, and runtimes like Node.js and Bun (see the [simdjson real-world usage list](https://github.com/simdjson/simdjson#real-world-usage)).

---

## Working With SIMD in Rust

Since I mostly write Rust, let's look at four ways to use SIMD in the language (though the same ideas apply to C/C++, Go, or Zig).

### 1. Auto-vectorization

The simplest path: we write a plain idiomatic Rust loop, and LLVM tries to turn it into SIMD instructions in release builds.

There is no guarantee it will happen, though. It depends on the target CPU, compiler flags, and whether the optimizer can prove there are no aliasing or dependency issues. You can check the generated assembly to see what LLVM actually emitted.

### 2. Direct Intrinsics (`std::arch`)

When you want full control, Rust's `std::arch` module exposes SSE, AVX2, AVX-512, and ARM NEON intrinsics directly.

On `x86_64`, for instance, you can load four `f32` values with `_mm_loadu_ps` and add them in one shot with `_mm_add_ps`. The `u` suffix stands for *unaligned* (loading without requiring strict vector boundary alignment; we will dig into alignment in the practical follow-up).

The upside is exact control over the instructions emitted into the binary. The downside is `unsafe` code, handling CPU architectures explicitly, and dealing with any leftover tail elements that do not fill a full vector.

{{< code-before-after before_label="Scalar loop (1 element per step)" after_label="Explicit SIMD (4 elements at once)" badge="4× f32 / op" caption="Comparison: a regular loop adds one pair at a time, while SSE intrinsics load and sum four f32 values in one instruction" >}}
```rust
fn add_scalar(a: &[f32; 4], b: &[f32; 4], out: &mut [f32; 4]) {
    for i in 0..4 {
        out[i] = a[i] + b[i];
    }
}
```
===
```rust
#[cfg(target_arch = "x86_64")]
unsafe fn add_sse(a: &[f32; 4], b: &[f32; 4], out: &mut [f32; 4]) {
    use std::arch::x86_64::{_mm_add_ps, _mm_loadu_ps, _mm_storeu_ps};
    let va = _mm_loadu_ps(a.as_ptr());
    let vb = _mm_loadu_ps(b.as_ptr());
    let sum = _mm_add_ps(va, vb);
    _mm_storeu_ps(out.as_mut_ptr(), sum);
}
```
===
Loading ;; out[i] = a[i] + b[i]; ;; let va = _mm_loadu_ps(a.as_ptr());||let vb = _mm_loadu_ps(b.as_ptr()); ;; Scalar code reads one element per iteration, whereas _mm_loadu_ps loads 128 bits (4 × f32) into XMM registers at once.
Addition ;; a[i] + b[i] ;; let sum = _mm_add_ps(va, vb); ;; Instead of four separate additions, one SSE vector instruction sums all 4 pairs in parallel.
Storing ;; out[i] = ... ;; _mm_storeu_ps(out.as_mut_ptr(), sum); ;; Writes the resulting 128-bit vector back to memory in one step without strict alignment requirements.
{{< /code-before-after >}}

### 3. Portable SIMD (`std::simd`)

To avoid tying your code to a single CPU architecture, Rust is developing `std::simd`. Instead of `_mm_add_ps`, you work with a portable `Simd::<f32, 4>` vector type and a regular `+` operator, letting the compiler pick the right instructions for the target.

One catch: `std::simd` is still experimental and requires nightly Rust with `#![feature(portable_simd)]`.

### 4. Ready-Made SIMD Crates

If you want to skip low-level intrinsics and `unsafe` on stable Rust, the ecosystem has solid crates:

- [wide](https://github.com/Lokathor/wide) - portable vector types with standard arithmetic;
- [pulp](https://github.com/sarah-quinones/pulp) - cross-architecture SIMD abstraction with runtime dispatch;
- [faster](https://github.com/AdamNiederer/faster) - a crate for ergonomic iterator vectorization.

Try the different approaches in the interactive examples below:


### 💻 Working With SIMD in Rust


#### Step 1

```rust
// Step 1: Auto-vectorization of a plain loop
fn add_slices(a: &[f32], b: &[f32], c: &mut [f32]) {
    let len = a.len().min(b.len()).min(c.len());
    for i in 0..len {
        c[i] = a[i] + b[i];
    }
}

fn main() {
    let a = [1.0_f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let b = [10.0_f32, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0];
    let mut c = [0.0_f32; 8];

    add_slices(&a, &b, &mut c);
    println!("Auto-vectorization result: {:?}", c);
}
```

**1. Auto-vectorization**
- The code looks like a regular scalar loop with no special APIs.
- In `--release` builds, LLVM can replace the loop with vector instructions on its own.
- Whether it actually vectorizes depends on the target CPU, compiler flags, and absence of loop-carried dependencies.


#### Step 2

```rust
// Step 2: Direct control via std::arch (Intrinsics)
fn add_simd_4(a: &[f32; 4], b: &[f32; 4]) -> [f32; 4] {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use std::arch::x86_64::{_mm_add_ps, _mm_loadu_ps, _mm_storeu_ps};
        let va = _mm_loadu_ps(a.as_ptr());
        let vb = _mm_loadu_ps(b.as_ptr());
        let sum = _mm_add_ps(va, vb);
        let mut out = [0.0_f32; 4];
        _mm_storeu_ps(out.as_mut_ptr(), sum);
        return out;
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        use std::arch::aarch64::{vaddq_f32, vld1q_f32, vst1q_f32};
        let va = vld1q_f32(a.as_ptr());
        let vb = vld1q_f32(b.as_ptr());
        let sum = vaddq_f32(va, vb);
        let mut out = [0.0_f32; 4];
        vst1q_f32(out.as_mut_ptr(), sum);
        return out;
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
    }
}

fn main() {
    let a = [1.0_f32, 2.0, 3.0, 4.0];
    let b = [10.0_f32, 20.0, 30.0, 40.0];
    let result = add_simd_4(&a, &b);
    println!("std::arch result: {:?}", result);
}
```

**2. Intrinsics via `std::arch`**
- Uses SSE on `x86_64` (`_mm_loadu_ps` + `_mm_add_ps`) and NEON on `aarch64` (`vld1q_f32` + `vaddq_f32`).
- The `u` suffix in `_mm_loadu_ps` stands for *unaligned*: loading without strict memory alignment requirements.
- Working directly with intrinsics requires `unsafe` blocks and manual handling of any tail elements that do not fill a full vector.


#### Step 3

```rust
// ?ignore
// Step 3: Portable SIMD (requires nightly and #![feature(portable_simd)])
#![feature(portable_simd)]
use std::simd::Simd;

fn main() {
    let a = Simd::<f32, 4>::from_array([1.0, 2.0, 3.0, 4.0]);
    let b = Simd::<f32, 4>::from_array([10.0, 20.0, 30.0, 40.0]);
    let result = a + b;
    println!("Portable SIMD result: {:?}", result);
}
```

**3. Portable SIMD (`std::simd`)**
- Lets you write portable vector code without tying yourself to SSE, AVX2, or NEON.
- Familiar operators like `+` work directly on the `Simd<f32, 4>` vector type.
- `std::simd` is still experimental and only available on nightly Rust.


---

## What's Next

That wraps up the theory. In the next post we will move to practice: pick a real algorithm (such as parsing or a DSP filter), vectorize it in a few different ways, inspect the generated assembly, and compare real benchmarks.

### Useful Links

- [GTA Online: the JSON parsing fix story](https://arstechnica.com/gaming/2021/03/developers-to-update-gta-online-to-address-poor-load-times-after-community-fix/)
- [simdjson on GitHub](https://github.com/simdjson/simdjson)
- [Intel Intrinsics Guide](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html)
- [ARM NEON Intrinsics](https://developer.arm.com/architectures/instruction-sets/intrinsics/)
- [Rust `std::arch` docs](https://doc.rust-lang.org/stable/std/arch/)
- [Portable SIMD (`std::simd`) docs](https://doc.rust-lang.org/nightly/std/simd/)
- SIMD crates: [wide](https://github.com/Lokathor/wide), [pulp](https://github.com/sarah-quinones/pulp), [faster](https://github.com/AdamNiederer/faster)


### 📝 Test Yourself: SIMD and Rust Basics


**1. How do SIMD and multithreading work together?**

- [ ] They are mutually exclusive: you cannot spawn multiple threads when using SIMD
- [ ] Multithreading spreads work across CPU cores, while SIMD speeds up uniform operations inside a single core
- [ ] SIMD automatically spawns background OS threads for each loop iteration
- [ ] Multithreading only works for f32, while SIMD is strictly for strings

**2. What does the acronym SIMD stand for?**

- [ ] Single Input, Multiple Devices
- [ ] Single Instruction, Multiple Data
- [ ] Simple Integer, Memory Direct
- [ ] Sequential Instruction, Multi-threaded Dispatch

**3. How many 32-bit f32 values fit into a single 256-bit vector register (such as AVX2)?**

- [ ] 4 values
- [ ] 8 values
- [ ] 16 values
- [ ] 32 values

**4. When is compiler auto-vectorization most likely to fail or bring no speedup?**

- [ ] When adding two large independent arrays of numbers
- [ ] When applying the same check to every byte in a slice
- [ ] When the next loop iteration depends on the previous one or the dataset is tiny
- [ ] When compiling with release optimizations enabled

**5. Which Rust standard library module gives direct access to platform-specific SIMD intrinsics (SSE, AVX2, NEON)?**

- [ ] std::simd
- [ ] std::arch
- [ ] std::thread
- [ ] std::mem

