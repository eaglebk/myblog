# Closures in Rust: Functions with their own Memory

> A simple analysis of function pointers and closures in Rust. Learn how closures capture environment, the difference between Fn, FnMut, and FnOnce, and how they are structured under the hood.

- **URL:** http://localhost:1314/en/rust/rust-closures/
- **Date:** 2026-07-16

- **Tags:** rust, dev, closures, advanced

---## Introduction

Closures (or lambda expressions) are anonymous functions that we can create on the fly and pass to other functions. But their main superpower is the ability to **capture environment**, meaning they remember variables from the scope where they were declared.

In programming languages with automatic garbage collection (GC), developers don't even think about how closures store these variables. But in Rust, where memory is managed by the compiler, the rules for closures are governed by strict ownership and borrowing semantics.

In this article, we will break down how closures work in Rust, how they differ from ordinary functions, how they are divided into `Fn`, `FnMut`, and `FnOnce` traits, and how the compiler generates them "under the hood."

---

## 1. Function Pointers (`fn`)

Before we talk about closures, let's recall ordinary functions.

In Rust, a function's name can act as a pointer to it. The type of such a pointer is written in lowercase: for example, `fn(i32) -> i32`.

* **How it is structured in memory:**
  * The `fn` pointer is a **"thin" pointer**. It stores only one address — the address of the first instruction of the compiled code in memory.
  * Such functions **have no state**. They cannot look at outer variables or read anything that wasn't passed to them in the arguments.

---

## 2. Closures: Functions with a backpack

A closure is an anonymous function that has an invisible "backpack" (context) behind its back, where it can put external variables.

```rust
let coefficient = 10;
let multiply = |x| x * coefficient; // The closure has captured coefficient!
```

Closure arguments are written inside two vertical bars `|...|`, followed by the body. Curly braces are only needed for multi-line code.

### Type Inference

The Rust compiler automatically infers what data types the closure accepts and returns, analyzing the place of its first invocation. However, if you want, you can specify the types manually:

```rust
let multiply = |x: i32| -> i32 { x * coefficient };
```

### Capturing Environment and the `move` keyword

* **By default (by reference):** The closure tries to be as efficient as possible. It doesn't copy variables; it borrows them. The compiler decides what kind of reference is needed: a shared reference for reading (`&T`) or a mutable reference for writing (`&mut T`).
* **Forced move (`move`):** If the closure must outlive the surrounding code (for example, we return it from a function or pass it to another thread), we need to move ownership of the variables inside the closure. For this, write the keyword `move` before the vertical bars:
  ```rust
  let print_data = move || println!("Data: {}", data);
  ```

---

## 3. Three Closure Traits: Fn, FnMut, and FnOnce

Depending on what exactly the closure does with the variables in its "backpack," it automatically implements one of three traits:

<div style="background-color:white; padding: 20px">
{{< mermaid >}}
graph TD
    FnOnce --> FnMut
    FnMut --> Fn
    
    style FnOnce fill:#f9f,stroke:#333,stroke-width:2px,color:#000
    style FnMut fill:#ccf,stroke:#333,stroke-width:1px,color:#000
    style Fn fill:#cfc,stroke:#333,stroke-width:1px,color:#000
{{< /mermaid >}}
</div>

1. **`FnOnce` (The Consumer)**
   * **What it does:** Takes captured variables by value and consumes them (destroys or passes them on).
   * **Limitation:** Since the data is consumed on the first call, you can call such a closure **only once**.
2. **`FnMut` (The Mutator)**
   * **What it does:** Modifies captured variables (uses an exclusive reference `&mut self`).
   * **Limitation:** Can be called any number of times, but only sequentially (cannot run concurrently from different threads without synchronization).
3. **`Fn` (The Reader)**
   * **What it does:** Only reads data (uses a shared reference `&self`).
   * **Limitation:** No limitations! Can be called repeatedly and concurrently from different threads.

### Trait Hierarchy

They form a nested structure:
* Any closure of type `Fn` automatically knows how to work as `FnMut` and `FnOnce`.
* Regular function pointers (`fn`) have no state, so they always implement the top trait `Fn`.

---

## 4. How Closures Work in Code

### Passing to Functions

Since the exact type of a closure is anonymous, we use generics or `impl Trait`:

```rust
fn execute(mut action: impl FnMut()) {
    action();
}
```

### Returning from Functions

You cannot return a closure directly — the compiler doesn't know its exact size. Therefore, we return it via `impl Trait` with the mandatory `move` keyword (to take the captured variables with us):

```rust
fn create_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
```

### Storing in Structs

If a struct needs to store a closure, we have two ways:
1. **Generics (fast):** The struct is statically bound to the closure type at compile time. No overhead.
   ```rust
   struct Runner<F: Fn()> { action: F }
   ```
2. **Box (flexible):** The closure is hidden in the heap under a dynamic pointer. Allows storing different closures in the same struct type.
   ```rust
   struct Runner { action: Box<dyn Fn()> }
   ```

---

## 5. What happens under the hood?

Closures in Rust are just syntactic sugar. Under the hood, the compiler turns each closure into a regular structure:

1. An **anonymous struct** is created, whose fields are the captured variables.
2. The implementation of the call (methods of traits `call`, `call_mut`, or `call_once`) is generated for this structure.
3. At the site of the closure call, the compiler simply substitutes a regular method call of this structure.

---

## 6. Lifetimes on the fly (HRTBs)

Sometimes a complex situation arises: a closure accepts a reference to an object, but the lifetime of this reference is born and dies right inside the calling function. In such cases, the closure must be able to work with references of **any** lifetime.

For this, Higher-Rank Trait Bounds (or HRTBs) syntax is used via the `for<'a>` keyword:

```rust
fn run_with_ref<F>(f: F)
where F: for<'a> Fn(&'a str) 
{
    let local = String::from("data");
    f(&local); // The lifetime of the reference is limited only to this function!
}
```

The construction `for<'a> Fn(&'a str)` tells the compiler: "This closure knows how to accept a string reference with absolutely any lifetime `'a` that we give it at the moment of call."

Let's see how closures of different types work in the interactive examples below:


### 💻 Closures in Rust


#### Step 1

```rust
fn call_with_five(f: fn(i32) -> i32) {
    println!("Fn pointer call: {}", f(5));
}

fn main() {
    // 1. Function pointer to a standard function
    fn add_one(x: i32) -> i32 { x + 1 }
    call_with_five(add_one);
    
    // 2. Closure capturing environment
    let coefficient = 10;
    let closure = |x| x * coefficient;
    
    // call_with_five(closure); // Error! Closure with state cannot be coerced to a plain fn pointer.
    println!("Closure call: {}", closure(5));
}
```

**Function Pointers and Closures**
- Function pointer `fn` is a "thin" pointer: it stores only the machine code memory address.
- Closure (anonymous function) can capture external variables and carry payload state.
- Closures receive a unique anonymous type generated by the compiler.


#### Step 2

```rust
fn call_fn<F: Fn()>(f: F) { f(); }
fn call_fn_mut<F: FnMut()>(mut f: F) { f(); }
fn call_fn_once<F: FnOnce()>(f: F) { f(); }

fn main() {
    let mut greeting = "Hello".to_string();
    
    // Fn: only reads environment
    let closure_fn = || println!("Fn: {}", greeting);
    call_fn(closure_fn);
    
    // FnMut: mutates environment
    let closure_mut = || {
        greeting.push_str(" World");
        println!("FnMut: {}", greeting);
    };
    call_fn_mut(closure_mut);
    
    // FnOnce: consumes (moves) ownership of greeting
    let closure_once = move || {
        let consumed = greeting;
        println!("FnOnce: {}", consumed);
    };
    call_fn_once(closure_once);
    // call_fn_once(closure_once); // Error! Variable closure_once was already consumed.
}
```

**Traits Fn, FnMut, and FnOnce**
- `Fn`: captures via shared reference `&self`. Allows concurrent multi-threaded execution.
- `FnMut`: captures via exclusive reference `&mut self`. Allows mutating captured state.
- `FnOnce`: consumes captured state. Can be called exactly once.


#### Step 3

```rust
fn get_counter(mut start: u32) -> impl FnMut() -> u32 {
    move || {
        start += 1;
        start
    }
}

fn main() {
    let mut next = get_counter(100);
    println!("Counter: {}", next());
    println!("Counter: {}", next());
}
```

**Returning Closures from Functions**
- Because closure types are anonymous, we use `impl Trait` return position syntax.
- We must use the `move` keyword to transfer ownership of captured variables.
- Otherwise the returned closure would reference local variables destroyed upon function exit.


#### Step 4

```rust
fn process_borrowed<F>(f: F)
where
    F: for<'a> Fn(&'a str),
{
    let local_string = String::from("Local Data");
    f(&local_string); // Reference lives strictly inside function
}
fn main() {
    // Closure accepts a string slice whose lifetime is determined inside process_borrowed
    let print_len = |s: &str| println!("String length: {}", s.len());
    process_borrowed(print_len);
}
```




---

## Check Your Knowledge!

Take a short interactive test to reinforce your understanding of closures in Rust.


### 📝 Closures in Rust


**1. What is the main low-level difference between a function pointer `fn` and a closure?**

- [ ] The `fn` pointer runs slower than closures
- [ ] The `fn` pointer is a thin pointer and cannot contain external state (context), whereas a closure can capture variables from the environment
- [ ] Closures are always allocated on the heap, while `fn` pointers are on the stack

**2. What is the inheritance (subtype) hierarchy between closure traits?**

- [ ] FnOnce is a subtype of FnMut, which is a subtype of Fn
- [ ] Fn is a subtype of FnMut, which is a subtype of FnOnce
- [ ] These traits are independent and not related to each other

**3. What happens if you apply the `move` keyword to a closure?**

- [ ] The closure will be moved to another execution thread
- [ ] All external variables mentioned in the closure will be forced to be captured by value (ownership will move inside the closure)
- [ ] The compiler will forbid calling the closure more than once

**4. Why are Higher-Rank Trait Bounds (HRTBs, `for<'a>` syntax) used in closure signatures?**

- [ ] To execute the closure concurrently in multiple threads
- [ ] To remove restrictions on recursive closure calls
- [ ] To bind lifetimes so that the closure can accept references with absolutely any lifetime determined at runtime at the moment of invocation

