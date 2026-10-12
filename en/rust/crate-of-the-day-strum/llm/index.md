# Crate of the Day: Strum - iterating and converting Rust Enum

> Exploring the popular Strum crate: how to iterate over enum variants, convert enums to strings and back, using a real-world example from esp-generate.

- **URL:** http://localhost:1314/en/rust/crate-of-the-day-strum/
- **Date:** 2026-08-16

- **Tags:** rust, crate-of-the-day, enums, macros, esp32

---
{{< lead >}}
"Crate of the Day" — concise breakdowns of popular and practical Rust ecosystem libraries that eliminate boilerplate and clean up your architecture.
{{< /lead >}}

Today's featured crate — **`strum`** (a suite of procedural macros for working with `enum`s in Rust).

---

## The Problem: Rust Enum lack built-in iterators

Enums are widely used in Rust to represent finite sets of states or variants. However, out of the box, **Rust enums provide no built-in mechanism to iterate over all their variants**:

```rust
// We want to loop over all enum variants:
for color in Color::all_variants() { ... } // ❌ Error! No such method exists
```

If you are developing a CLI tool (like `esp-generate`), a web service, or an API, you frequently need to:
1. Print a list of all supported options in `--help` or an interactive console selector.
2. Validate and parse an incoming user input string or config file value into an `enum`.
3. Convert an enum variant into a formatted string (e.g., for `Color`: `"black"` or `"white"`).

Without third-party crates, developers often resort to manually maintaining arrays like `const ALL_COLORS: &[Color] = &[...];`. 

> **The primary risk of this approach:** as soon as a new variant (like `GREY`) is added to the enum, someone will inevitably forget to update the manual array.

---

## Community solution: real-world example from `esp-generate`

The official project generator for Espressif microcontrollers — **`esp-generate`** (developed by the `esp-rs` team) solves this exact problem using `strum`.

Here is how their `Chip` enum is declared in the project source code:

```rust
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    clap::ValueEnum,
    strum::EnumIter,
    strum::Display,
    strum::EnumString,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum Chip {
    Esp32,
    Esp32c2,
    Esp32c3,
    Esp32c5,
    Esp32c6,
    Esp32c61,
    Esp32h2,
    Esp32s2,
    Esp32s3,
}
```

Just three macros from `strum` cover all requirements for string conversions and variant iteration!

---

## Core strum features

### 1. Iterating over Enum variants (`strum::EnumIter`)

The `EnumIter` derive macro, combined with the `strum::IntoEnumIterator` trait, endows the `enum` with an `.iter()` method:

```rust
use strum::IntoEnumIterator;

for chip in Chip::iter() {
    println!("Supported microcontroller: {chip}");
}
```

Now, whenever a new chip variant (such as `Esp32p4`) is added, it **automatically** participates in all iteration loops across your project without modifying any downstream loop logic!

---

### 2. Converting Enums to Strings (`strum::Display`)

The `Display` macro eliminates the boilerplate of writing `impl std::fmt::Display for Chip` manually.

The `#[strum(serialize_all = "kebab-case")]` attribute configures proper string formatting:
* `Chip::Esp32c3` becomes `"esp32c3"`.
* `Chip::Esp32s3` becomes `"esp32s3"`.

```rust
println!("{}", Chip::Esp32c3); // Outputs: esp32c3
```

---

### 3. Parsing Strings into Enums (`strum::EnumString`)

The `EnumString` macro implements `std::str::FromStr`. This makes parsing string inputs straightforward:

```rust
use std::str::FromStr;

let chip = Chip::from_str("esp32c3")?; // Ok(Chip::Esp32c3)
// Or via standard parse() method:
let chip: Chip = "esp32c3".parse()?;
```

---

## Additional helpful Strum macros

The `strum` crate offers several other powerful derives:

* **`EnumCount`**: Provides a compile-time constant (e.g., `Chip::COUNT`) representing the total number of variants in the `enum`.
* **`VariantNames`**: Exposes a slice containing all variant names (e.g., `Chip::VARIANTS` — `["esp32", "esp32c2"]`).
* **`EnumProperty` / `EnumMessage`**: Allows attaching key-value metadata or human-readable descriptions directly to variants via attributes like `#[strum(message = "...")]`.

---

## Complete code example

```rust
use strum::{EnumIter, EnumString, Display, IntoEnumIterator};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, EnumIter, Display, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Chip {
    Esp32,
    Esp32c2,
    Esp32c3,
    Esp32c5,
    Esp32c6,
    Esp32c61,
    Esp32h2,
    Esp32s2,
    Esp32s3,
}

fn main() {
    // 1. Iterate over all enum variants
    println!("=== ESP Chips ===");
    for chip in Chip::iter() {
        println!("- {chip}");
    }

    // 2. Parse string back into enum
    let input = "esp32c3";
    match Chip::from_str(input) {
        Ok(chip) => println!("\nSuccessfully parsed '{input}' into variant: {:?}", chip),
        Err(_) => println!("\nUnknown chip!"),
    }
}
```

### Program Output:
```text
=== ESP Chips ===
- esp32
- esp32c2
- esp32c3
- esp32c5
- esp32c6
- esp32c61
- esp32h2
- esp32s2
- esp32s3

Successfully parsed 'esp32c3' into variant: Esp32c3
```

---

## 🎯 Test your knowledge!

Take this short quiz to reinforce your understanding of working with `strum`:


### 📝 Strum Crate in Rust


**1. Which strum macro adds the .iter() method for an enum?**

- [ ] strum::Display
- [ ] strum::EnumIter
- [ ] strum::EnumString
- [ ] strum::EnumCount

**2. How does #[strum(serialize_all = "kebab-case")] convert the Esp32c3 variant into a string?**

- [ ] Converts it to ESP32C3
- [ ] Converts it to esp32c3
- [ ] Converts it to esp_32_c3
- [ ] Leaves it unchanged

**3. What is the primary role of the derive macro strum::EnumString?**

- [ ] Prints the enum to console via std::fmt::Display
- [ ] Counts the number of elements in the enum
- [ ] Implements std::str::FromStr to parse strings into enum variants
- [ ] Creates a string array of all enum variant names

**4. What is the main advantage of strum::EnumIter over a manual array like variants: &[Chip]?**

- [ ] It only works at runtime
- [ ] Adding a new variant to the enum automatically includes it in all iteration loops
- [ ] It speeds up program compilation 2x
- [ ] It stores variants in memory as a linked list

---

## How to add to your project

Simply add the dependency to your `Cargo.toml`:

```toml
[dependencies]
strum = { version = "0.28", features = ["derive"] }
```

🔗 **GitHub Repository:** [Petrochev/strum](https://github.com/Petrochev/strum)  
📦 **Crates.io:** [crates.io/crates/strum](https://crates.io/crates/strum)  
🦀 **Real-world example from esp-generate:** [esp-rs/esp-generate](https://github.com/esp-rs/esp-generate)

