# Interior Mutability in Rust: Pattern Philosophy, Cell, and RefCell

> Exploring the Interior Mutability pattern in Rust: core mechanics, Cell and RefCell containers, inherited vs interior mutability, and practical code examples.

- **URL:** https://eagle2000.space/en/rust/rust-interior-mutability/
- **Date:** 2026-09-09

- **Tags:** rust, dev, interior-mutability, cell, refcell

---
## Introduction

Usually, the borrow checker acts strictly: if a variable is declared without `mut`, the compiler will never allow modifying its fields through an immutable reference `&T`. This is standard **Inherited Mutability**.

However, in real-world code, situations arise where a component needs to store a cached response, record an error log, or update an internal identifier while holding only an immutable reference `&self`.

For these cases, the Rust standard library provides the **Interior Mutability** pattern. In this article, I explore how `Cell` and `RefCell` work, where they apply in practice, and how their runtime checks operate.

---

## 1. The Cell<T> Container: Mutability Without References

The simplest tool for interior mutability is **`std::cell::Cell<T>`**.

The key property of `Cell<T>` is that it **never hands out direct references** `&T` or `&mut T` to the value inside it.

Instead of references, `Cell` operates by reading, writing, or moving values as a whole:
* `get()` — returns a copy of the inner value (requires `T: Copy`).
* `set(val)` — completely replaces the inner value with `val`.
* `replace(val)` — stores `val` inside the container and returns the previous value.
* `take()` — extracts the value, leaving `Default::default()` in its place.

```rust
use std::cell::Cell;

struct Counter {
    count: Cell<u32>,
}

fn main() {
    let c = Counter { count: Cell::new(0) };
    
    // The variable 'c' is NOT mutable! But we can freely modify count:
    c.count.set(c.count.get() + 1);
    
    println!("Current count: {}", c.count.get()); // Prints 1
}
```

> [!NOTE]
> Note: For non-`Copy` types (like `String` or `Vec`), calling `.get()` will fail to compile. However, non-`Copy` types in `Cell` can still be updated via `.set()`, `.replace()`, and `.take()`.

> [!TIP]
> Because `Cell<T>` never issues references to its inner data, it **does not need to track borrow counts**. It has zero runtime overhead (Zero-cost abstraction) and zero risk of panicking! It is the ideal choice for flags, counters, and `Copy` types.

---

## 2. The RefCell<T> Container: Dynamic Borrow Checking

When storing a complex data structure (such as `Vec<String>`, `HashMap`, or a custom object) inside a container, copying it as a whole via `Cell::get()` is not an option.

For complex types, Rust provides **`std::cell::RefCell<T>`**.

`RefCell<T>` lets you request references to the inner value:
* `.borrow()` — returns a `Ref<T>` wrapper that acts like an immutable reference `&T`.
* `.borrow_mut()` — returns a `RefMut<T>` wrapper that acts like a mutable reference `&mut T`.
* `.try_borrow()` / `.try_borrow_mut()` — non-panicking variants returning `Result`, allowing safe checks when borrow conflicts might occur.

### Runtime Borrow Checking

The standard Rust compiler enforces the rule *"either multiple `&` or exactly one `&mut`"* at compile time.

`RefCell` defers this borrow counter check to **runtime**.

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3]);

    let mut b1 = data.borrow_mut();
    b1.push(4);

    // WARNING: b1 is still active! Attempting a second borrow_mut will PANIC:
    // let mut b2 = data.borrow_mut(); // ALREADY BORROWED PANIC!
}
```

Thanks to the RAII idiom, the `Ref` and `RefMut` smart pointers automatically decrement the borrow counter of `RefCell` in their `Drop` destructor when going out of scope. Therefore, calling `drop(b1)` explicitly allows the next `data.borrow_mut()` call to succeed smoothly.

> [!CAUTION]
> Remember: `RefCell` does not bypass Rust borrow rules; it merely defers their enforcement to runtime. Architectural design flaws result in runtime panics rather than compile errors!

---

## 3. Real-World Use Cases

Now that we understand how `Cell` and `RefCell` operate, let's explore why changing data through an immutable reference `&self` is necessary when `&mut self` isn't feasible.

In real-world code, using `&mut self` is not always an option. Here are three common scenarios:

### 1. Button Click Counters
Consider a standard UI button component. Let's try updating a click counter inside a `&self` method:


### 💻 Comparison: Compilation Error vs Solution with Cell


#### Step 1

```rust
// ?compile_fail

struct Button {
    label: String,
    click_count: u32,
}

impl Button {
    fn on_click(&self) {
        println!("Button '{}' clicked!", self.label);
        self.click_count += 1;
    }
}

fn main() {
    let btn = Button {
        label: String::from("Submit"),
        click_count: 0,
    };
    btn.on_click();
}
```

**Without Cell: Compilation Error**
- The compiler prohibits modifying `click_count` through an immutable reference `&self`.
- Declaring the method as `&mut self` is often impossible in UI components, as buttons are accessed from multiple places simultaneously.


#### Step 2

```rust
use std::cell::Cell;

struct Button {
    label: String,
    click_count: Cell<u32>,
}

impl Button {
    fn on_click(&self) {
        println!("Button '{}' clicked!", self.label);
        let count = self.click_count.get();
        self.click_count.set(count + 1);
    }
}

fn main() {
    let btn = Button {
        label: String::from("Submit"),
        click_count: Cell::new(0),
    };

    btn.on_click();
    btn.on_click();
    println!("Total button clicks: {}", btn.click_count.get());
}
```

**Solution using Cell<T>: Compiles Successfully**
- Wrap the counter in `Cell<u32>`.
- The `on_click(&self)` method successfully updates the inner value via `.set()` while preserving the `&self` reference.


In UI frameworks, event handler methods usually accept `&self` because button instances are invoked from multiple parts of the application. `Cell` enables updating the internal click counter directly through `&self` without propagating `&mut self` across the entire call chain.

### 2. Log History Accumulation in Services
Another scenario involves a shared application context `App`, accessed across components via `&App` to read settings. With `RefCell`, any function can record warnings into a shared log vector on the fly:

```rust
struct App {
    name: String,
    // Log history stored inside the application struct
    logs: RefCell<Vec<String>>,
}

impl App {
    fn log_error(&self, msg: &str) {
        // Appends an entry to the vector holding only &self
        self.logs.borrow_mut().push(msg.to_string());
    }
}
```

### 3. State Flags ("Already Initialized")
The same pattern applies to initialization flags in background audio players or loaders:

```rust
struct Player {
    is_playing: Cell<bool>,
}

impl Player {
    fn play(&self) {
        if !self.is_playing.get() {
            println!("Starting playback...");
            self.is_playing.set(true);
        }
    }
}
```

---

## 4. Multithreading Constraints

An essential caveat: `Cell` and `RefCell` containers are designed **strictly for single-threaded execution**.

Neither type implements the `Sync` marker trait, meaning the compiler prevents passing references to them across thread boundaries at build time.

A detailed walkthrough of multithreaded interior mutability (`Mutex`, `RwLock`) and the `Send` / `Sync` traits will be covered in a separate dedicated article.

---

| Type | Issues References? | Borrow Checking | Type & Trait Requirements | Overhead |
| :--- | :---: | :---: | :---: | :--- |
| **`Cell<T>`** | No (get/set only) | Not required | Requires `T: Copy` for `.get()` | Zero-cost |
| **`RefCell<T>`** | Yes (`Ref` / `RefMut`) | Runtime (Panics on violation) | Works with any `T` | Small runtime counter |

Let's explore how these types operate in practical interactive code slides:


### 💻 Comparing Cell<T> and RefCell<T> Behavior


#### Step 1

```rust
use std::cell::Cell;

struct Logger {
    log_count: Cell<usize>,
}

impl Logger {
    fn new() -> Self {
        Logger {
            log_count: Cell::new(0),
        }
    }

    // Note: The method takes &self (immutable reference!), but mutates log_count!
    fn log(&self, message: &str) {
        println!("[LOG]: {message}");
        let current = self.log_count.get();
        self.log_count.set(current + 1);
    }
}

fn main() {
    let logger = Logger::new();

    logger.log("First system log message");
    logger.log("Second system log message");

    println!("Total log entries: {}", logger.log_count.get());

    // The replace method replaces the value and returns the old one
    let old_val = logger.log_count.replace(100);
    println!("Old count: {old_val}, new count: {}", logger.log_count.get());
}
```

**Interior Mutability with Cell<T>**
- `Cell<T>` allows modifying values inside an object holding only an immutable reference `&self`.
- Writing via `.set()` and reading via `.get()` works by copying or moving values without issuing direct references `&T`.
- Ideal for simple scalar (`Copy`) types, flags, and counters.


#### Step 2

```rust
use std::cell::RefCell;

struct UserCache {
    cache: RefCell<Vec<String>>,
}

impl UserCache {
    fn new() -> Self {
        UserCache {
            cache: RefCell::new(Vec::new()),
        }
    }

    fn add_user(&self, name: String) {
        // Request a mutable borrow at runtime
        self.cache.borrow_mut().push(name);
    }

    fn print_users(&self) {
        // Request an immutable borrow
        let users = self.cache.borrow();
        println!("Cached users: {:?}", *users);
    }
}

fn main() {
    let cache = UserCache::new();

    cache.add_user("Alice".to_string());
    cache.add_user("Bob".to_string());

    cache.print_users();

    // Check cache size
    println!("Total users count: {}", cache.cache.borrow().len());
}
```

**Dynamic Borrow Checking with RefCell<T>**
- `RefCell<T>` allows mutating complex structures on the heap via `borrow_mut()` through `&self` references.
- Unlike `Cell`, `RefCell` hands out temporary smart references `Ref` and `RefMut`.
- Borrow checks are deferred to runtime. Attempting to hold two `borrow_mut()` handles simultaneously triggers a panic.


---

## Test Your Knowledge!

Take a short interactive quiz on interior mutability:


### 📝 Interior Mutability in Rust


**1. What is the core purpose of the Interior Mutability pattern in Rust?**

- [ ] Automatically copying all local variables onto the heap
- [ ] Mutating inner data of a structure through an immutable &self reference
- [ ] Bypassing compile-time type checking completely
- [ ] Increasing stack allocation limits for large structs

**2. Which `Cell<T>` method extracts the inner value while replacing it with `T::default()`?**

- [ ] Cell::take()
- [ ] Cell::get()
- [ ] Cell::replace()
- [ ] Cell::drop()

**3. What is the primary distinction between `RefCell<T>` and `Cell<T>`?**

- [ ] Cell compiles directly into unsafe unverified code
- [ ] RefCell issues temporary smart references (Ref/RefMut) to complex inner objects, whereas Cell copies or moves values as a whole
- [ ] RefCell cannot hold standard data structures
- [ ] Cell can only be used with String vectors

**4. What happens at runtime if `.borrow_mut()` is called on a `RefCell` while a previous mutable borrow is still active?**

- [ ] The compiler fails the build
- [ ] The thread blocks until the reference is released
- [ ] The program panics at runtime (Already borrowed panic)
- [ ] Data is automatically cloned in memory

**5. What constraint must type `T` satisfy to invoke the `.get()` method on `Cell<T>`?**

- [ ] Type T must implement the Copy trait
- [ ] Type T must be wrapped inside an Option
- [ ] Type T must implement the Display trait
- [ ] Type T must be stored exclusively on the heap

**6. When is `Cell<u32>` preferable over `RefCell<u32>`?**

- [ ] When long-lived references to inner elements are needed
- [ ] When a counter or flag is needed with zero runtime overhead and guaranteed panic-free execution
- [ ] When data must be shared across threads
- [ ] Never, RefCell is always superior

**7. What type does the `.borrow_mut()` method on `RefCell<T>` return?**

- [ ] A raw mutable reference &mut T
- [ ] A RefMut<T> smart wrapper that decrements the borrow counter when dropped
- [ ] A raw pointer *mut T
- [ ] A new RefCell<T> instance

**8. How can you attempt to borrow from a `RefCell<T>` safely without risking a runtime panic?**

- [ ] Use try_borrow() and try_borrow_mut() methods which return a Result
- [ ] Wrap the call inside an unsafe block
- [ ] Invoke std::mem::forget on the handle
- [ ] Use a panic_catch! macro

**9. Which `Cell<T>` method inserts a new value into the container and returns the previously stored value?**

- [ ] Cell::set()
- [ ] Cell::get()
- [ ] Cell::replace()
- [ ] Cell::take()

**10. What happens if `.borrow_mut()` is requested while an immutable reference from `.borrow()` is still active on `RefCell`?**

- [ ] The compiler prevents the build
- [ ] A runtime panic occurs because Rust borrow rules prohibit aliasing & and &mut
- [ ] The request is silently ignored
- [ ] The existing immutable reference is automatically invalidated

**11. What is the performance overhead of `Cell<T>` compared to a plain struct field?**

- [ ] Significant overhead due to OS mutex locks
- [ ] Zero-cost — in compiled code, Cell is represented as a plain value in memory
- [ ] Continuous heap allocation overhead
- [ ] 50% performance degradation on every access

**12. What happens to a `RefCell` borrow counter when a `Ref` or `RefMut` object goes out of scope?**

- [ ] The counter instantly resets to 0
- [ ] Its Drop implementation automatically decrements the active borrow counter
- [ ] The counter remains unchanged until process exit
- [ ] The counter increments by 1

**13. Can `Cell<String>` be used with non-`Copy` types like `String`?**

- [ ] Yes, but .get() is unavailable; mutation works via set(), replace(), and take()
- [ ] No, Cell only works with f32 and u32 types
- [ ] Yes, but only within unsafe blocks
- [ ] No, the compiler rejects Cell<String> definitions

**14. Why does calling `let b1 = data.borrow_mut(); drop(b1); let b2 = data.borrow_mut();` NOT panic?**

- [ ] The first borrow b1 ended and was released before b2 was requested
- [ ] RefCell allows unlimited simultaneous mutable references
- [ ] drop() disables borrow checking entirely
- [ ] The compiler merges both borrows into one

**15. Which scenario is `RefCell<Vec<String>>` best suited for?**

- [ ] A multithreaded server handling 100 concurrent threads
- [ ] Computing compile-time numerical constants
- [ ] A single-threaded service struct passed by &self that needs to append log entries to a vector
- [ ] Creating global immutable constants

**16. What is the result of invoking `Cell::set(new_val)` on a `Cell<u32>`?**

- [ ] The previous value is overwritten with new_val
- [ ] The method returns a Result error
- [ ] The value is appended to an internal queue
- [ ] Subsequent calls to the struct are blocked

**17. Which fundamental Rust borrowing rule does `RefCell` defer to runtime?**

- [ ] Either one mutable reference &mut T OR any number of immutable references &T
- [ ] Variable names must be in uppercase
- [ ] All custom structs must implement Copy
- [ ] Stack variables are automatically moved to the heap

