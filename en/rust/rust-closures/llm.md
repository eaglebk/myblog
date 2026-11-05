# Closures in Rust: Functions with Their Own Memory

> A practical breakdown of function pointers and closures in Rust: environment capture, how Fn, FnMut, and FnOnce map to struct methods under the hood, and when you need for<'a>.

- **URL:** https://eagle2000.space/en/rust/rust-closures/
- **Date:** 2026-11-05

- **Tags:** rust, dev, closures, advanced

---
In everyday Rust code, closures show up everywhere, from simple iterator chains like `.map(|x| x * 2)` and `.filter(|item| item.id == target_id)` to spawning background work in `tokio::spawn(async move { ... })`. At first glance, they look just like lambdas in a dozen other languages: write arguments between vertical bars `|...|`, reference whatever outer variables you need, and move on.

Things get interesting the moment you try to store a closure in a struct, return it from a function, or send it across threads.

In garbage-collected languages (Go, JavaScript, Python, C#), the runtime handles outer local variables automatically: if a closure touches a local variable, that variable is quietly promoted from the stack to the heap and stays alive as long as anything references it. Rust has no garbage collector. When a closure touches a stack variable, the compiler has to figure out at compile time whether the closure borrows `&T`, mutably borrows `&mut T`, or takes ownership (`T`), who drops the memory, and how long that closure can live.

That is why Rust does not have a single universal "function type." Instead, it separates plain function pointers (`fn`) from three closure traits: `FnOnce`, `FnMut`, and `Fn`. Here is how that machinery works under the hood.

---

## 1. Plain Function Pointers (`fn`)

Start with the simplest case, where a function needs no external state at all.

Every compiled function in a binary has a fixed address in the `.text` code segment where its first machine instruction lives. In Rust, a function's name can act as a pointer to that address, and its type is written in lowercase, such as `fn(i32) -> i32`.

An `fn` pointer is a thin pointer: just a single machine word (8 bytes on a 64-bit target) holding a code address. It has no state and cannot read local variables from the caller's scope; it only works with the arguments explicitly passed to it.

Suppose I write a simple `map` function for slices that accepts a function pointer `fn(T) -> U`:

```rust
fn map<T, U>(slice: &[T], func: fn(T) -> U) -> Vec<U>
where
    T: Copy,
{
    let mut result = Vec::with_capacity(slice.len());
    for value in slice {
        // value is &T, so we dereference it (*value) before calling func
        result.push(func(*value));
    }
    result
}
```

As long as I pass a regular function or a stateless closure like `|x| x * 2`, this compiles cleanly. In fact, any closure that captures nothing from its surroundings automatically coerces to a plain `fn` pointer.

The moment I declare `let factor = 10;` outside and try to pass `|x| x * factor` into `func: fn(i32) -> i32`, compilation fails. The reason is purely physical: an `fn` pointer only has 8 bytes for the instruction address. There is nowhere to store `factor` or a reference to it.

---

## 2. How Closures Capture Variables and Why `move` Exists

For a function to remember outer variables, it needs its own block of data memory alongside the code address:

```rust
let coefficient = 10;
let multiply = |x| x * coefficient;
```

You rarely need to write parameter or return types inside `|...|`. The compiler inspects the first call to `multiply(5)` and infers `|x: i32| -> i32` on its own, though you can annotate types explicitly if you want.

What matters more is how `multiply` captures `coefficient`. The Rust compiler inspects the closure body and picks the least restrictive capture mode it can get away with:

1. If the variable is only read inside the closure (printed, added, passed as `&T`), the closure stores a shared reference `&coefficient`.
2. If the variable is modified inside (for instance, calling `list.push(x)` or incrementing a counter), the closure captures an exclusive mutable reference `&mut list`. While the closure exists, nothing else can access `list`.
3. If the variable is consumed or moved out by value (such as calling `drop(data)` or `into_iter()`), references are no longer enough, so the closure takes full ownership of `data`.

If the compiler already knows when to borrow and when to take ownership, why does Rust have a separate `move` keyword?

You need `move` when the closure body only reads the data (so the compiler would normally borrow `&T`), but the closure itself must **outlive the scope** where it was created. That happens whenever you return a closure from a function or hand it to a background thread via `std::thread::spawn`:

```rust
let data = vec![1, 2, 3];

std::thread::spawn(move || {
    // We only read data here, so without `move` the compiler would borrow &data.
    // The thread might outlive the current function, so a stack reference is illegal.
    println!("Data in thread: {:?}", data);
});
```

Placing `move` before the vertical bars disables automatic reference borrowing and forces the closure to move ownership of all referenced variables inside itself.

> [!NOTE]
> One subtle detail with `move` often trips people up in interviews: if a captured variable implements `Copy` (like `i32` or `bool`), `move` copies the bits into the closure. The original variable in the outer scope remains valid and usable afterward.

---

## 3. What the Compiler Generates Under the Hood

Before looking at `Fn`, `FnMut`, and `FnOnce`, it helps to see what `rustc` turns a closure into at compile time. There is no special runtime magic.

When you write:

```rust
let factor = 10;
let multiply = |x: i32| x * factor;
```

the compiler generates an anonymous struct whose fields hold the captured variables, and turns the closure body into a method on that struct:

```rust
// Pseudocode of what rustc generates:
struct ClosureEnvironment<'a> {
    factor: &'a i32, // or plain i32 if `move` was used
}

impl<'a> ClosureEnvironment<'a> {
    fn call(&self, x: i32) -> i32 {
        x * (*self.factor)
    }
}
```

Almost every closure rule in Rust follows directly from this desugaring:

- Every closure expression in your source code gets its own unique, anonymous struct type. Even if you write two identical closures `|x| x + 1` on adjacent lines, the compiler treats them as two distinct struct types.
- A closure's size on the stack is the sum of the sizes of the variables (or references) it captured. If a closure captures nothing, its generated struct is a zero-sized type (0 bytes).

---

## 4. The Three Traits: FnOnce, FnMut, and Fn

Look back at the `call(&self, ...)` method on the generated struct above. Like any Rust method, it can take `self` in one of three ways: by value (`self`), by mutable reference (`&mut self`), or by shared reference (`&self`).

That receiver type is the exact difference between the three standard closure traits:

### `FnOnce` (takes `self`)
The base trait. Calling it consumes the closure struct by value. If your closure moves a captured `String` or `Vec` somewhere else (or drops it), you cannot run it a second time because the field inside the closure struct has already been moved out. `FnOnce` closures can only be called once.

### `FnMut` (takes `&mut self`)
Implemented by closures that mutate their captured fields without destroying them. You can call an `FnMut` closure as many times as you like (for example, a stateful counter or accumulator), but calling it requires mutable access to the closure variable itself, and you cannot invoke it concurrently across threads without synchronization.

### `Fn` (takes `&self`)
Implemented by closures that only need a shared reference to their state, or have no state at all. They can be called repeatedly and shared across threads simultaneously.

### Choosing the Right Bound in `where` Clauses

In the standard library, these traits form a subtrait hierarchy: `Fn` is a subtrait of `FnMut`, and `FnMut` is a subtrait of `FnOnce`.

When writing generic functions, it is easiest to think about this from the caller's perspective:

- Asking for **`FnOnce`** (like `Option::unwrap_or_else` does) is the most permissive bound for callers. Callers can pass **any closure at all** (`Fn`, `FnMut`, or `FnOnce`). After all, if a closure can be called many times (`Fn`), it can certainly be called once.
- Asking for **`FnMut`** (like `Iterator::map` or `Vec::retain`) accepts both `FnMut` and `Fn` closures, rejecting only one-shot `FnOnce` closures.
- Asking for **`Fn`** is the strictest requirement: only pure readers and stateless `fn` pointers qualify.

---

## 5. Passing, Returning, and Storing Closures in Structs

Because every closure has an unnamed struct type, you cannot spell its type directly in a function signature. Instead, you accept closures via generics or `impl Trait`:

```rust
fn execute(mut action: impl FnMut()) {
    action();
}
```

Returning a closure from a function also uses `impl Trait`, almost always paired with `move` so the closure owns its captured data instead of holding a dangling reference to the function's stack frame:

```rust
fn create_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
```

> [!WARNING]
> **Return-position `impl Trait` limitation:** `-> impl Fn(...)` uses static dispatch, meaning the function must return a single concrete closure struct type. Returning different closures from `if / else` branches will not compile:
> 
> ```rust
> fn returns_closure(cond: bool) -> impl Fn(i32) -> i32 {
>     if cond {
>         |x| x
>     } else {
>         |x| x * 3 // ERROR: the two branches produce different anonymous types!
>     }
> }
> ```
> 
> When the returned closure depends on a runtime condition, box it into a trait object `Box<dyn Fn(...)>` (or coerce it to `fn(i32) -> i32` if neither branch captures variables):
> 
> ```rust
> fn returns_closure(cond: bool) -> Box<dyn Fn(i32) -> i32> {
>     if cond {
>         Box::new(|x| x)
>     } else {
>         Box::new(|x| x * 3)
>     }
> }
> ```

You face the same trade-off when storing a callback inside a struct:

1. **Generic field (`struct Runner<F: Fn()> { action: F }`):** The closure struct is stored inline inside `Runner` with zero heap allocation, and the compiler can inline the call. The trade-off is that each `Runner<F>` instance is tied to one specific closure type.
2. **Boxed trait object (`struct Runner { action: Box<dyn Fn()> }`):** The closure is allocated on the heap behind a fat pointer (data address + vtable). This adds a small indirection cost, but lets you store different callbacks in a single `Vec<Box<dyn Fn()>>`.

---

## 6. Lifetimes on the Fly (`for<'a>` / HRTB)

One last case comes up when closures accept references created inside the function that calls them.

Imagine a function that allocates a temporary `String` on its own stack, passes a `&str` slice into a callback, and then drops the `String` before returning. If you try to put a lifetime parameter `'a` on the outer function (`fn run_with_ref<'a, F>(f: F)`), the compiler rejects it: `'a` is chosen by the caller, and no caller lifetime can describe a variable that only exists inside `run_with_ref`.

To express *"this closure can accept a reference with any lifetime `'a` chosen at the call site"*, Rust uses Higher-Rank Trait Bounds (HRTBs) with the `for<'a>` syntax:

```rust
fn run_with_ref<F>(f: F)
where
    F: for<'a> Fn(&'a str),
{
    let local = String::from("local buffer");
    f(&local); // the borrow lives only inside this function
}
```

For simple bounds like `F: Fn(&str)`, the compiler quietly expands the signature to `F: for<'a> Fn(&'a str)` for you. You only need to write `for<'a>` explicitly in more complex cases, such as when a closure takes a reference and returns another reference tied to that same local lifetime (`for<'a> Fn(&'a str) -> &'a str`).

Try running and modifying these examples in the interactive slides below:


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

## Check Your Knowledge

Take a short quiz to test how well you know `fn` pointers, `move` captures, and the `Fn` trait family:


### 📝 Closures in Rust


**1. Why can the closure `|x| x * 2` be passed to a parameter of type `fn(i32) -> i32`, whereas `|x| x * factor` (where `factor` is an outer variable) fails to compile?**

- [ ] An `fn` pointer only works with named functions declared via the `fn` keyword
- [ ] An `fn` pointer is 8 bytes and only stores a code address, leaving nowhere to store the captured `factor` variable or a reference to it
- [ ] Multiplication with local variables is forbidden inside `fn` pointers

**2. What does the compiler do by default (without `move`) if a closure calls `list.push(42)` on an outer vector?**

- [ ] It clones the entire `list` vector into the closure
- [ ] It moves ownership of `list` into the closure and drops it after the first call
- [ ] It captures a mutable reference `&mut list`, preventing any other access to `list` while the closure is alive

**3. Why do you need the `move` keyword before a closure when the closure body only reads an outer variable (for example, when passing it to `std::thread::spawn`)?**

- [ ] To force the closure to move ownership of the variable inside itself; otherwise it would borrow `&T` from a stack frame that may die before the thread finishes
- [ ] To speed up reads by disabling bounds checking
- [ ] Because `move` automatically wraps all captured variables in `Arc` and `Mutex`

**4. What happens to `let count: i32 = 10;` if it is used inside a `move` closure: `let f = move || println!("{}", count);`?**

- [ ] Using `count` in the outer scope after creating the closure becomes a compile error
- [ ] A bitwise copy of the integer is stored inside the closure, and `count` remains valid and usable in the outer scope
- [ ] The code fails to compile because `move` cannot be used with primitive types

**5. What does the Rust compiler turn a closure into at compile time, and what is its size on the stack?**

- [ ] A heap-allocated dynamic object whose size is always 16 bytes
- [ ] An anonymous struct whose fields hold the captured variables (or references to them), with a size equal to the sum of those fields
- [ ] A global function whose variables are placed in the `.bss` static segment

**6. How do the methods of `FnOnce`, `FnMut`, and `Fn` take the closure struct receiver (`self`)?**

- [ ] `FnOnce` takes `self` by value, `FnMut` takes `&mut self`, and `Fn` takes `&self`
- [ ] `FnOnce` takes `&self`, `FnMut` takes `&mut self`, and `Fn` takes `self` by value
- [ ] All three traits take `&self` and check mutability at runtime

**7. If a function is declared as `fn run<F: FnOnce()>(action: F)`, which closures can a caller pass to it?**

- [ ] Strictly closures that drop or move their captured variables when called
- [ ] Only `FnOnce` and `FnMut` closures, but not `Fn`
- [ ] Any closure at all (`FnOnce`, `FnMut`, `Fn`) as well as plain `fn` pointers

**8. Why does `fn make_op(flag: bool) -> impl Fn(i32) -> i32` fail to compile if the `if` branch returns `|x| x + 1` and the `else` branch returns `|x| x * 2`?**

- [ ] The `impl` keyword cannot be used in a return type position
- [ ] Every closure gets its own unique anonymous struct type, and `-> impl Trait` requires returning a single concrete type across all branches
- [ ] Both closures must capture the `flag` variable to have the same type

**9. What is the main trade-off between storing a closure in a struct via a generic parameter `struct Runner<F: Fn()> { f: F }` versus `Box<dyn Fn()>`?**

- [ ] A generic field stores the closure inline without heap allocation and allows call inlining, while `Box<dyn Fn()>` allocates on the heap but lets you store different closures in the same struct or collection
- [ ] The generic version lets you swap out the closure for a different one at runtime, while `Box<dyn Fn()>` does not
- [ ] A generic struct field can only store stateless `fn` pointers

**10. Why is the `for<'a>` syntax (Higher-Rank Trait Bounds) used in `where F: for<'a> Fn(&'a str)`?**

- [ ] It runs the closure in a loop for every substring
- [ ] It requires the string slice passed to the closure to have a `'static` lifetime
- [ ] It tells the compiler that the closure can accept a reference with any lifetime created right inside the calling function at the moment of invocation

