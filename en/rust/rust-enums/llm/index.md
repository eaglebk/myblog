# Enums in Rust: From Syntax to Pattern Matching

> In-depth overview of Rust enums: data-carrying variants, uninhabited types, the Option pattern, and match pattern matching with hands-on examples.

- **URL:** http://localhost:1314/en/rust/rust-enums/
- **Date:** 2026-08-01

- **Tags:** rust, dev

---
## Introduction: What is an Enum?

**Enums (Enumerations)** in Rust are custom data types that allow you to represent a value that can be one of several possible variants.

If you have prior experience with Java or C++, you probably used enums to define simple sets of constants (such as days of the week). In Rust, however, enums are vastly more capable: each variant can hold heterogeneous data of different shapes and types, making your code safe, expressive, and concise.

## Constructing Variants

Unlike many languages where enums are simply named integer constants under the hood, Rust enum variants can store rich data structures. A single `enum` can hold three distinct variant flavors:

1. **Unit variants:** without associated data (similar to unit structs).
2. **Tuple variants:** with unnamed payload data in parentheses (similar to tuple structs).
3. **Struct variants:** with named fields inside curly braces `{}` (similar to traditional structs).

Let's explore how variants are declared, what an uninhabited type represents, and how data is extracted using `match` in the interactive slides below:


### 💻 Working with Enums in Rust


#### Step 1

```rust
#[allow(dead_code)]
#[derive(Debug)]
enum WebEvent {
    // Unit variant (no associated data)
    PageLoad,
    // Tuple variant (with unnamed associated data)
    KeyPress(char),
    // Struct variant (with named fields)
    Click { x: i64, y: i64 },
}

fn main() {
    let load = WebEvent::PageLoad;
    let press = WebEvent::KeyPress('q');
    let click = WebEvent::Click { x: 320, y: 240 };

    println!("load: {:?}", load);
    println!("press: {:?}", press);
    println!("click: {:?}", click);
}
```

**Enum Variant Construction**
- Enums can combine different data shapes under a single unified type.
- Unit variants carry no associated data (`PageLoad`).
- Tuple variants hold unnamed data inside parentheses (`KeyPress`).
- Struct variants hold named fields inside curly braces (`Click`).


#### Step 2

```rust
// Defining an uninhabited type (an enum without variants)
#[allow(dead_code)]
enum EmptyEnum {}

fn main() {
    // Creating an instance of EmptyEnum is impossible because it has no variants!
    println!("EmptyEnum is defined, but cannot be instantiated.");
}
```

**Uninhabited Types**
- An enum with no variants is called an uninhabited type.
- Instantiating such a type in safe code is impossible.
- Useful in the type system to represent impossible states (e.g., an error that can never happen).


#### Step 3

```rust
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Echo(String),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };

    // Extracting data via pattern matching
    match msg {
        Message::Quit => {
            println!("Quit event");
        }
        Message::Move { x, y } => {
            println!("Move to coordinates x = {}, y = {}", x, y);
        }
        Message::Echo(text) => {
            println!("Message text: {}", text);
        }
    }
}
```

**Extracting Data with match**
- The `match` expression guarantees safe access to data stored inside enum variants.
- The Rust compiler enforces exhaustiveness: every possible variant must be handled.
- Data is destructured directly inside the match arm pattern.


### Implementation Details:

* **Step 1. Variant Flavors:** The `WebEvent` enum demonstrates all three variant forms. An important detail: all fields and payloads defined within enum variants are **automatically public**; you do not need to prefix them with `pub`.
* **Step 2. Empty Enums (Uninhabited Types):** Rust allows defining an `enum` without any variants at all (e.g., `enum EmptyEnum {}`). It is impossible to instantiate this type in safe code. Such types help represent states that should never occur, allowing the compiler to optimize unreachable execution paths.
* **Step 3. Extracting Data (Pattern Matching):** Because different variants can hold entirely different data types, the compiler will not allow direct access without inspection. We must determine which variant is currently active. The most expressive way to do this is using the `match` expression.

> **Important:** The `match` expression in Rust is exhaustive. The compiler will produce an error if even a single enum variant is left unhandled.

---

## Test Your Knowledge

Take a quick interactive quiz to solidify your understanding of Rust enums:


### 📝 Rust Enums


**1. What are Enums commonly referred to as in type theory?**

- [ ] Product type
- [ ] Sum type or tagged union
- [ ] Dynamic interface
- [ ] Abstract class

**2. What is the default visibility of fields (associated data) within enum variants?**

- [ ] They are automatically private
- [ ] They are automatically public
- [ ] Their visibility matches the module visibility
- [ ] You must explicitly add `pub` before each variant field

**3. Which variant in `enum Foo { A, B(u32), C { val: bool } }` is a struct-like variant?**

- [ ] A
- [ ] B(u32)
- [ ] C { val: bool }
- [ ] None of the above

**4. What constitutes an uninhabited type when declared as an enum?**

- [ ] An enum where all variants are marked as deprecated
- [ ] An enum that contains zero variants
- [ ] An enum containing only empty unit variants
- [ ] The special `Option::None` variant

**5. How do you safely extract associated data from an enum variant?**

- [ ] Using the `as` typecast operator
- [ ] Using pattern matching (e.g., via `match` or `if let`)
- [ ] Using dot notation to access the variant index (e.g., `event.KeyPress.0`)
- [ ] By binding the data to a global variable

