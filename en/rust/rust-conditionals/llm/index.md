# Conditionals and Control Flow in Rust

> Master branching and control flow in Rust: code blocks, if-else expressions, single-pattern matching with if let, and let-chains.

- **URL:** http://localhost:1314/en/rust/rust-conditionals/
- **Date:** 2026-07-31

- **Tags:** rust, dev

---
## Introduction: Code Blocks and Expressions

In Rust, a code block is a fundamental syntactic unit. It does not merely group statements together; it is an expression that yields a value.

* **Syntax:** Enclosed by curly braces `{ ... }`. Inside, statements typically terminate with a semicolon `;`.
* **Return Value:** 
  * If the final statement of the block is an expression without a trailing semicolon, its evaluated value becomes the return value of the entire block.
  * If the final statement ends with a semicolon `;` (or if the block is empty), the block evaluates to the unit type **`()`** (an empty tuple).

Let's explore code blocks, `if-else` expressions, and pattern matching in the interactive slides below:


### 💻 Branching and Code Blocks


#### Step 1

```rust
fn main() {
    // A code block evaluates to the value of its final expression
    let y = {
        let x = 1;
        let z = 2;
        x + z // No semicolon -> returns 3
    };

    // A code block ending with a semicolon returns the unit type ()
    let u = {
        let x = 10;
        println!("x = {}", x);
    };

    println!("y = {}, u = {:?}", y, u);
}
```

**Code Blocks**
- A code block is enclosed in curly braces `{}`.
- If the final expression of a block does not end with `;`, its value is returned from the block.
- A block ending with `;` or an empty block returns the unit type `()`.


#### Step 2

```rust
fn main() {
    let x = 150;

    // In Rust, if-else is an expression that produces a value
    let y = if x > 100 {
        let val = 2024;
        val
    } else {
        let val = 2025;
        val
    };

    println!("y = {}", y);

    // COMPILE ERROR: Both if-else branches must evaluate to the same type!
    // let z = if x > 100 { 2024 } else { "2025" };
}
```

**Conditional if-else Expressions**
- Parentheses around the condition are unnecessary (e.g., `if x > 100` instead of `if (x > 100)`).
- The condition must strictly evaluate to the boolean type `bool`.
- Both `if` and `else` branches must return identical types. Rust has no ternary operator; `if-else` serves this purpose.


#### Step 3

```rust
fn main() {
    let some_val: Option<i32> = Some(150);

    // Using if let for concise single-pattern matching
    if let Some(x) = some_val {
        println!("Matched! x = {}", x);
    }

    let another_val: Option<i32> = Some(50);

    // Starting with Rust 1.88, chained let patterns with && (let_chains) are supported
    if let Some(x) = some_val && let Some(y) = another_val && x > y {
        println!("Both values extracted, and x ({}) > y ({})", x, y);
    }
}
```

**if let and Chained Conditions**
- `if let` provides a concise alternative to `match` when testing a single pattern.
- Variables bound inside the pattern are scoped strictly to the `if let` block.
- Modern Rust allows combining multiple `let` pattern bindings and boolean expressions using `&&`.


---

## Conditionals and Branching

### Basic `if` and `else`

The `if` construct checks a boolean predicate and executes the corresponding branch.

* **Syntax Rules:**
  * The condition **must not** be wrapped in parentheses `( )`. Adding parentheses around the condition (e.g., `if (a > 5)`) is considered non-idiomatic and triggers compiler linter warnings.
  * The condition expression must strictly evaluate to the **`bool`** type. Using integers (like in C or C++) triggers a compilation error.

```rust
// COMPILE ERROR: count is not a boolean
if count { ... } 

// CORRECT: expressions evaluate to bool
if count > 0 { ... }
if count != 0 { ... }
```

### Comparison and Logical Operators

Used to construct conditions:
* `>` (greater than), `>=` (greater than or equal to)
* `<` (less than), `<=` (less than or equal to)
* `==` (equality), `!=` (inequality)
* `&&` (logical AND), `||` (logical OR)

### Assigning from `if` Expressions

In Rust, `if-else` is an expression, which means it returns a value that can be directly bound to a variable.

* **Critical Type Rule:** All branches in an `if-else` expression **must evaluate to the exact same type**.
* Rust does not have a traditional ternary operator (`? :`). Instead, `if-else` serves this role cleanly.

```rust
// Correct: both branches produce i32
let y = if x > 100 { 2024 } else { 2025 }; 

// COMPILE ERROR: mismatched branch types (i32 vs &str):
let y = if x > 100 { 2024 } else { "2025" }; 
```

If the `else` branch is omitted, the `if` expression must return the unit type `()`, since evaluating to false produces `()` by default:

```rust
let u = if x > 100 { foo() }; // Valid only if foo() returns ()
```

---

## Pattern Matching with `if let`

### Basic `if let`

`if let` provides a concise alternative to `match` when you only care about handling a single specific pattern.

```rust
// For Option
if let Some(x) = foo_opt() {
    bar(x); // x is scoped exclusively to this block
} else {
    // Executed if the result is None
}
```

### Chained `if let` (let-chains)

Starting with modern Rust versions, chaining multiple `let` pattern bindings and boolean conditions using the `&&` operator is supported within a single `if` statement.

```rust
// Pattern matching combined with an additional predicate on x
if let Some(x) = foo() && x > 100 {
    todo!()
}

// Multiple pattern matches in a single condition
if let Some(x) = foo() && let Some(y) = bar() && x > y {
    todo!()
}
```

---

## Test Your Knowledge

Take a quick interactive quiz to solidify your understanding of code blocks and branching in Rust:


### 📝 Rust Conditionals


**1. What does the code block `{ let x = 5; x * 2; }` evaluate to?**

- [ ] 10
- [ ] 5
- [ ] The unit type `()` (empty tuple)
- [ ] A compiler error due to missing explicit return

**2. Which of the following conditions in an `if` expression will cause a compile error?**

- [ ] if x != 0 { ... }
- [ ] if x { ... } (where x is 1 of type i32)
- [ ] if x > 5 && y < 10 { ... }
- [ ] if true { ... }

**3. What is the critical requirement for `if-else` branches when used as an expression to assign a value?**

- [ ] They must return an integer type
- [ ] They must be enclosed in parentheses
- [ ] Both branches (if and else) must evaluate to identical types
- [ ] The else branch must be omitted

**4. What does `let u = if x > 100 { foo() };` evaluate to if foo() returns i32 and x is 50?**

- [ ] 0
- [ ] Option::None
- [ ] A compilation error (type mismatch between i32 and ())
- [ ] i32

**5. Which logical operator combines multiple `let` pattern bindings and conditions in an `if let` chain?**

- [ ] || (OR)
- [ ] && (AND)
- [ ] , (comma)
- [ ] and

