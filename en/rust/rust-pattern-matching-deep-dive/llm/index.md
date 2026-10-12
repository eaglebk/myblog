# Rust: Deep Dive into Pattern Matching - Destructuring, Bindings, and Guards

> In-depth guide to pattern matching in Rust: exhaustiveness, destructuring, @-bindings in dupe-krill, slice patterns, match guards, and let chains.

- **URL:** http://localhost:1313/en/rust/rust-pattern-matching-deep-dive/
- **Date:** 2026-10-05

- **Tags:** rust, dev, pattern-matching, destructuring, match-guards, if-let, while-let

---
## Introduction

Pattern matching in Rust is often mistaken for a mere analog of `switch/case` statements found in other languages. In practice, it is one of the most expressive and powerful tools in the language. Rust provides `match`, `if let`, and `let` expressions that destructure structs on the fly, evaluate complex conditions without nested `if` blocks, and eliminate invalid states right at compile time.

In previous articles, I covered enums in [rust-enums](../rust-enums/) and control flow constructs in [rust-conditionals](../rust-conditionals/).

Today, let's move beyond the basics and explore how Pattern Matching works in real-world software: from `@`-bindings in CLI utilities to slice patterns and state machines in production open-source projects.

---

## 1. Exhaustiveness and destructuring

The primary guarantee the compiler enforces on any `match` is **exhaustiveness**. If a type can take three states, you must explicitly handle all three, or supply a `_` wildcard to catch everything else.

In the slider below, step 1 demonstrates how the compiler blocks an unhandled variant, while step 2 shows how the `_` wildcard resolves the issue (though relying on wildcards is discouraged when adding new states):


### 💻 Exhaustiveness and the _ wildcard


#### Step 1

```rust
// ?compile_fail
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected { ip: String },
}

fn handle_connection(state: ConnectionState) {
    match state {
        ConnectionState::Disconnected => println!("Offline"),
        ConnectionState::Connected { ip } => println!("Connected to {ip}"),
        // Compile error: `Connecting` variant is not covered!
    }
}

fn main() {
    let state = ConnectionState::Connecting;
    handle_connection(state);
}
```

**1. Compile Error: Non-exhaustive match**
- The Rust compiler strictly enforces exhaustiveness checks across all enum variants.
- The `Connecting` variant is missing, so compilation fails with error E0004.


#### Step 2

```rust
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected { ip: String },
}

fn handle_connection(state: ConnectionState) {
    match state {
        ConnectionState::Disconnected => println!("Offline"),
        ConnectionState::Connected { ip } => println!("Connected to {ip}"),
        // The `_` wildcard catches all remaining variants:
        _ => println!("Other state"),
    }
}

fn main() {
    handle_connection(ConnectionState::Disconnected);
    handle_connection(ConnectionState::Connecting);
    handle_connection(ConnectionState::Connected {
        ip: String::from("192.168.1.1"),
    });
}
```

**2. Fixing with the `_` wildcard**
- The `_` pattern catches any unhandled variants, satisfying exhaustiveness checks.
- Although this compiles, overusing `_` is discouraged: when new enum variants are added, the compiler will not warn you to handle them explicitly.


### Destructuring structs and tuples on the fly

Pattern matching works not only in `match`, but also in ordinary `let` statements. You can extract nested fields in a single step:

```rust
struct Address {
    street: String,
    city: String,
}

struct Person {
    name: String,
    address: Address,
}

// Extract the name and nested address fields in one step:
let Person { 
    name, 
    address: Address { street, city } 
} = person;
```

Unneeded fields can be discarded using `_` or the rest operator `..`:

```rust
let Person { name, address: Address { city, .. } } = person;
```

### Borrowing: Match Ergonomics and the legacy of `ref`

In modern Rust, matching on references feels natural thanks to **Match Ergonomics** (introduced in Rust 2018). When you need to match without consuming ownership (for non-`Copy` types), simply pass a reference into `match`:

```rust
let point = (String::from("sensor_1"), 42);

// Pass &point — the compiler automatically binds name as &String:
match &point {
    (name, 0) => println!("Sensor {name} at zero mark"),
    (name, val) if *val > 100 => println!("Threshold exceeded on sensor {name}: {val}"),
    _ => println!("Normal operation"),
}
// point remains accessible here because we matched on a reference
```

> [!NOTE]
> **Why is `ref` rarely seen in modern Rust code?**
>
> Prior to Match Ergonomics in Rust 2018, the compiler required explicitly declaring the binding mode in each pattern: `(ref name, 0)` or `ref mut x`.
>
> Today, explicit `ref` is considered legacy syntax and is almost never needed in everyday code. Its only remaining use case is **partial moves**, where one field of a struct is moved by value while another is borrowed by reference:
> ```rust
> struct Task {
>     id: String,
>     payload: Vec<u8>,
> }
> 
> // payload is moved into ownership, while id is borrowed:
> let Task { ref id, payload } = task;
> ```

### Multiple patterns with `|`

When several arms share the exact same logic, combine them using the pipe `|` operator:

```rust
match status {
    "queued" | "pending" => println!("In processing queue"),
    "running" | "retrying" => println!("Running"),
    _ => println!("Completed or failed"),
}
```

---

## 2. `@`-bindings: A real-world example from dupe-krill

A common problem: check whether a number or object falls within a range, and **immediately use** that value in the arm body. Without `@`-bindings, you would either need nested `if` statements or duplicate range checks.

The `@` operator solves this cleanly: it tests the subpattern and binds the matched value to a variable at the same time.

An excellent production example comes from the file duplicate finder [dupe-krill (by Kornel Lesiński)](https://github.com/kornelski/dupe-krill/blob/0e813b83c08543d4052523e05ca170bb1d155e91/src/ui.rs#L50), which formats scan durations into human-readable strings using range matching:

```rust
// Source code from dupe-krill (src/ui.rs):
let nice_duration = match scan_duration.as_secs() {
    x @ 0..=5 => format!(
        "{:.1}s", 
        (x * 1_000_000_000 + u64::from(scan_duration.subsec_nanos())) as f64 / 1_000_000_000f64
    ),
    x @ 5..=59 => format!("{x}s"),
    x => format!("{}m{}s", x / 60, x % 60),
};
```

Notice how this works:
* In `x @ 0..=5`, the compiler checks that seconds fall in `0..=5` and binds that number to `x` to format fractional seconds with nanoseconds (`1.4s`).
* In `x @ 5..=59`, the number is printed as integer seconds (`42s`).
* The final arm binds any remaining values to `x` and splits them into minutes and seconds (`2m15s`).

The code reads sequentially from top to bottom with zero boilerplate variables.

---

## 3. Slice Patterns and Guards: 3D file parser in normalize_obj

**Match Guards** add an arbitrary `if` condition to a match arm. The arm matches only if both the pattern and the guard condition evaluate to true.

Guards become especially expressive when paired with **Slice Patterns**.

Consider the OFF 3D model parser from [normalize_obj (Julian Knodt)](https://github.com/JulianKnodt/normalize_obj/blob/a43e1c71a2f2d5c336ba72d840b9293a8496c66c/src/off.rs#L35). The parser splits lines by whitespace and matches on token slices:

```rust
// Parser excerpt from normalize_obj (src/off.rs):
match &tokens.as_slice() {
    // 1. Ignore comments: the first token starts with '#'
    [v, ..] if v.starts_with('#') => continue,

    // 2. Read format header:
    [off] if off.to_lowercase() == "off" => {
        curr = State::Counts;
    }

    // 3. Exact 3-element slice: vertex coordinates (x, y, z)
    [x, y, z] if curr == State::Verts => {
        let x = x.parse::<f64>()?;
        let y = y.parse::<f64>()?;
        let z = z.parse::<f64>()?;
        verts.push([x, y, z]);
    }

    // 4. Capture head and tail slice using `@ ..`:
    [cnt, r @ ..] if curr == State::Faces => {
        let _cnt = cnt.parse::<u32>()?;
        let v_idxs = r.iter().map(|vi| vi.parse::<usize>().unwrap()).collect();
        faces.push(v_idxs);
    }

    _ => panic!("Unexpected line in file"),
}
```

Key highlights:
1. `[v, ..]` verifies that the line contains at least one word `v`, ignoring the rest `..`. The `if v.starts_with('#')` guard filters out comments.
2. `[x, y, z]` strictly requires **exactly three** elements in the slice.
3. `[cnt, r @ ..]` extracts the first element as face count `cnt` and binds the rest of the slice to `r`.

An entire parser state machine fits into a single, clean `match` without manual `tokens.len() == 3` checks.

---

## 4. Interactive examples: Destructuring and guards

Explore and run the interactive examples with destructuring and guards below:


### 💻 Destructuring, @-bindings, and Match Guards


#### Step 1

```rust
// Step 1: Deep struct destructuring and binding patterns (@)
struct Address {
    street: String,
    city: String,
}

struct Person {
    name: String,
    address: Address,
}

fn main() {
    let person = Person {
        name: String::from("Alex"),
        address: Address {
            street: String::from("Broadway"),
            city: String::from("New York"),
        },
    };

    // 1. Extracting fields from nested structs in a single step:
    let Person { name, address: Address { street, city } } = person;
    println!("Name: {name}, City: {city}, Street: {street}");

    // 2. Using @-binding to keep the value while checking a range:
    let val = 15;
    match val {
        num @ 1..=10 => println!("Number {num} in range 1..=10"),
        num @ 11..=20 => println!("Number {num} in range 11..=20"),
        _ => println!("Other number"),
    }
}
```

**1. Destructuring and @-bindings**
- Patterns allow dissecting nested structs and enums in a single expression without intermediate variables.
- The `@` operator binds the matched value to a variable while simultaneously checking a range or pattern.


#### Step 2

```rust
enum Signal {
    Quit,
    Move { x: i32, y: i32 },
    Echo(String),
}

// Step 2: Match Guards in enums
fn main() {
    let signal = Signal::Move { x: 20, y: 20 };

    match signal {
        Signal::Quit => println!("Quit command"),
        // Match Guard with coordinate equality condition:
        Signal::Move { x, y } if x == y => {
            println!("Diagonal move to point ({x}, {y})");
        }
        Signal::Move { x, y } => {
            println!("Standard move to point ({x}, {y})");
        }
        Signal::Echo(text) => println!("Signal text: {text}"),
    }
}
```

**2. Match Guards**
- A Match Guard (`if CONDITION`) adds an arbitrary runtime condition to a `match` arm.
- The compiler evaluates arms sequentially: place more specific guarded arms before general ones.


---

## 5. `if let` chains (`let_chains`) and `while let`

When a full `match` is excessive (for instance, when only checking a single positive case), `if let` comes in handy.

Modern Rust allows chaining multiple patterns with boolean conditions using `&&` (**let chains**):

```rust
let result: Option<Result<i32, &str>> = Some(Ok(42));

// Unpack nested Option and Result types in a single condition:
if let Some(res) = result && let Ok(val) = res && val > 10 {
    println!("Extracted valid value: {val}");
}
```

For consuming queues, iterators, or Tokio channels, the **`while let`** loop is ideal:

```rust
let mut stack = vec![1, 2, 3];

// Loops as long as pop() returns Some(item):
while let Some(top) = stack.pop() {
    println!("Popped from stack: {top}");
}
```

Try the interactive example with chained conditions:


### 💻 if let chains and while let loop


#### Step 1

```rust
// Step 1: Combining conditions with if-let chains (let_chains)
fn main() {
    let result: Option<Result<i32, &str>> = Some(Ok(42));

    // if-let chain combining pattern matching and boolean expressions with &&:
    if let Some(res) = result && let Ok(val) = res && val > 10 {
        println!("Successfully extracted value greater than 10: {val}");
    }
}
```

**1. if-let chains (let_chains)**
- In modern Rust, multiple `let` pattern bindings and boolean conditions can be chained using `&&`.
- This eliminates deeply nested `if let` blocks and keeps logic flat and readable.


#### Step 2

```rust
// Step 2: Using while let to consume iterators
fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
    let mut iter = numbers.into_iter();

    // while let loops while iter.next() returns Some(val)
    while let Some(val) = iter.next() {
        println!("Processed element from iterator: {val}");
    }

    println!("Iterator finished!");
}
```

**2. Concise while let loop**
- The `while let` construct runs repeatedly as long as the expression matches the specified pattern.
- Perfect for working with iterators, stack buffers, and data queues.


---

## Test your knowledge

Test your understanding of advanced Pattern Matching in Rust:


### 📝 Advanced Pattern Matching


**1. What is the purpose of the `@` operator in pattern matching?**

- [ ] Binds the matched value to a variable while simultaneously testing it against a range or subpattern
- [ ] Forces immediate deallocation of the object from memory
- [ ] Converts a string slice into a byte array

**2. What is a Match Guard in a `match` expression?**

- [ ] A specialized destructor for memory cleanup
- [ ] An additional `if` condition on a match arm that must evaluate to `true` for the arm to be selected
- [ ] A compiler protection against overwriting configuration files

**3. What is the primary benefit of `if let` chains (let_chains) with the `&&` operator?**

- [ ] They disable compiler lifetime checks
- [ ] They increase floating-point calculation precision
- [ ] They allow extracting values from multiple patterns and checking conditions sequentially without deep nesting

**4. Which scenario is the `while let` construct best suited for?**

- [ ] Looping repeatedly as long as an expression matches a pattern (such as successive `iter.next()` calls)
- [ ] Defining declarative `macro_rules!` macros
- [ ] Invoking C functions via FFI

**5. What happens during compilation if an enum variant is added but not handled in any `match` arm and there is no wildcard `_`?**

- [ ] The code compiles successfully, but panics at runtime if that variant is hit
- [ ] Compilation error: Rust strictly enforces exhaustiveness and will not compile code with uncovered variants
- [ ] The compiler automatically inserts a no-op default handler

**6. How does Match Ergonomics (introduced in Rust 2018) work when matching on a reference (`match &point`)?**

- [ ] It automatically clones the structure via `.clone()` on each arm
- [ ] It casts the reference to a raw pointer (`*const T`)
- [ ] The compiler automatically propagates reference binding mode, binding variables as references without requiring the `ref` keyword

**7. In which rare scenario is the `ref` keyword still necessary in modern Rust?**

- [ ] During a partial move, when one field is moved by value while another field is borrowed by reference
- [ ] Every time a reference field is declared inside a `struct`
- [ ] When passing arguments to asynchronous Tokio tasks

**8. What does the pattern `[first, rest @ ..]` do when matching a slice (Slice Patterns)?**

- [ ] Copies the slice into a `VecDeque` ring buffer
- [ ] Extracts the first element into `first` and binds the remaining tail slice to the `rest` variable
- [ ] Compares the first and last elements of the array for equality

