# Trait Objects: How Dynamic Polymorphism Works in Rust

> Learn how trait objects (dyn Trait) work in Rust in simple terms: fat pointers, vtables, memory alignment, and object safety rules in practice.

- **URL:** https://eagle2000.space/en/rust/rust-dyn-traits/
- **Date:** 2026-08-14

- **Tags:** rust, dev, traits, dyn, vtable, middle

---
## Introduction

In our previous articles: [RAII, Box, and Deref](../rust-raii-box-deref) and [Introduction to Traits](../rust-traits), we explored basic traits and heap allocations via `Box<T>`. By default, Rust strives to make your code as fast as possible: when using generic types, the compiler performs **monomorphization**. It takes a function and generates a concrete copy for every single type used, producing direct, zero-cost function calls.

However, real-world software engineering often presents tasks where data types are not known upfront at compile time.

Imagine we are designing a notification or background task system. We have different task structs: `NetworkTask` (HTTP requests), `DbTask` (database writes), and `TimerTask` (timer triggers). All of them implement a shared `Task` trait with an `execute()` method. We want to store all these tasks in a single list (`Vec`) and process them inside a loop.

How do we store different types inside a single `Vec`? A vector in Rust requires every element to have the exact same type and byte size in memory.

This is where **trait objects** and the `dyn` keyword come to the rescue. In this article, we will examine how dynamic method dispatch works in Rust, where "fat" pointers come from, and how the compiler keeps everything memory-safe.

---

## 1. Static vs. Dynamic Dispatch

To choose the right tool for the job, let's compare these two approaches:

| Feature | Static Polymorphism (Generics) | Dynamic Polymorphism (`dyn Trait`) |
| --- | --- | --- |
| **Method Resolution** | Compile time | Runtime |
| **Binary Size** | Slightly larger (code duplicated per type) | Smaller (single function instance for all types) |
| **Collections in Memory** | Single type per vector (e.g., strictly `Vec<i32>`) | Heterogeneous types in one vector (`Vec<Box<dyn Task>>`) |
| **Execution Speed** | Direct call (maximum performance) | Indirect call via method table (small runtime overhead) |

### Heterogeneous Collections Example

To store different types inside a single vector, we wrap them in a `Box` pointer and specify the `dyn` keyword:

```rust
trait Task { 
    fn execute(&self); 
}

struct NetworkTask;
struct DbTask;

// Implementing Task for these structs...

// Storing different types in a single vector
let tasks: Vec<Box<dyn Task>> = vec![
    Box::new(NetworkTask),
    Box::new(DbTask),
];

for task in tasks {
    task.execute(); // Which exact function executes is resolved at runtime
}
```

> [!NOTE]
> If the exhaustive list of all possible types is known upfront and won't expand dynamically, a standard `enum` serves as a great alternative to `dyn Trait`.

---

## 2. Type Sizes and `?Sized`

In Rust, almost every type has a fixed size in bytes known to the compiler at compile time. These are called `Sized` types. For example, `i64` always takes 8 bytes, and a struct of two `i32` fields takes 8 bytes. The compiler knows exactly how much stack space to allocate.

However, some types have a size that can only be determined at runtime. These are called **DSTs (Dynamically Sized Types)**:
- String slices `str`
- Array slices `[T]`
- Trait objects like `dyn Task`

> [!CAUTION]
> Variables of DST types cannot be stored directly on the stack or passed by value. A variable declaration like `let x: dyn Task` will fail to compile because Rust does not know how many bytes to allocate on the stack. Dynamic types must always live behind pointers or references: `&str`, `&[T]`, `Box<dyn Task>`.

By default, the compiler assumes every generic parameter must have a fixed size (`T: Sized`). If we want to write a function that works with dynamic types by reference, we must explicitly relax this constraint using the `?Sized` bound ("size may be unknown"):

```rust
// T can be either fixed-size or dynamically-sized (e.g., &str or &dyn Debug)
fn print_val<T: ?Sized + std::fmt::Debug>(val: &T) {
    println!("{:?}", val);
}
```

---

## 3. Fat Pointers and the Method Table (vtable)

A standard reference on a 64-bit architecture (like `&i32` or `Box<u64>`) takes **8 bytes** (one machine word) in memory. It is simply a memory address.

However, a reference to a dynamic type like `&dyn Task` or `Box<dyn Task>` takes **16 bytes (two machine words)**. Because of this doubled size, they are called **fat pointers**.

What do these 16 bytes contain?
1. **First 8 bytes:** Address of the actual data (where the struct lives on the heap or stack).
2. **Second 8 bytes:** Address of the virtual method table (`vtable`) for that concrete type.

<div style="background-color:white; padding: 20px">
{{< mermaid >}}
graph TD
    subgraph FatPointer["Fat Pointer (16 bytes)"]
        direction LR
        DataPtr["1. Data Pointer (8 bytes)"]
        VtablePtr["2. vtable Pointer (8 bytes)"]
    end

    DataMemory["<b>Data in Memory</b><br/>──────────────────────<br/>NetworkTask<br/><i>(object payload)</i>"]
    VtableMemory["<b>Virtual Method Table (vtable)</b><br/>──────────────────────<br/>vtable for NetworkTask<br/>• Destructor function (Drop)<br/>• Object size in bytes (size)<br/>• Memory alignment (align)<br/>• Pointer to execute() method"]

    DataPtr --> DataMemory
    VtablePtr --> VtableMemory

    style FatPointer fill:#f4f4f6,stroke:#666,stroke-width:2px,color:#000
    style DataPtr fill:#fff,stroke:#333,color:#000
    style VtablePtr fill:#fff,stroke:#333,color:#000
    style DataMemory fill:#e8f4f8,stroke:#0288d1,stroke-width:2px,color:#000
    style VtableMemory fill:#efebe9,stroke:#5d4037,stroke-width:2px,color:#000
{{< /mermaid >}}
</div>

### Virtual Method Table (vtable) in Practice

For every type implementing a trait, the compiler constructs a small static `vtable` during compilation. It contains:
- Address of the cleanup function (`Drop`).
- Object size and alignment requirements (so `Box` knows how many bytes to return to the OS on deallocation).
- Addresses of all methods declared in the trait.

When we invoke a method on a trait object:

```rust
let task: Box<dyn Task> = Box::new(NetworkTask);
task.execute();
```

The following steps occur under the hood:
1. The program reads the second pointer from the fat pointer to locate the `vtable`.
2. It looks up the function address for `NetworkTask::execute` from the table slot.
3. It calls the function, passing the data address (from the first half of the fat pointer) as the first `&self` argument.

---

## 4. Combining Traits and Why `dyn (Fly + Swim)` Is Disallowed

In Rust, you cannot directly construct a trait object from two independent traits:

```rust
// Compilation error!
fn handle(obj: &(dyn Fly + Swim)) {}
```

The reason is simple: supporting this would require a "three-word" fat pointer (data address + vtable1 + vtable2), adding memory overhead and complexity to pointer layouts.

### How to Achieve This in Code?

We create a single supertrait that inherits the required traits, and then provide a **Blanket Implementation**:

```rust
trait Fly { 
    fn fly(&self) -> String; 
}
trait Swim { 
    fn swim(&self) -> String; 
}

// Combine constraints into a single trait
trait Duck: Fly + Swim {}

// Blanket implementation for any type implementing both Fly and Swim
impl<T: Fly + Swim> Duck for T {}

// Now dyn Duck is completely valid and takes standard 16 bytes!
fn handle_duck(duck: &dyn Duck) {
    println!("Duck: {}, {}", duck.fly(), duck.swim());
}
```

---

## 5. Object Safety Rules

Not every trait can be converted into a `dyn Trait` object. For a trait to support dynamic polymorphism, it must adhere to **Object Safety** rules.

These rules become intuitive once you remember how `vtable` works: a virtual table can only store methods that have a fixed function pointer signature and receive a pointer to data.

### Why Do Certain Methods Break Object Safety?

1. **Method takes `self` by value (`fn run(self)`):** Passing by value requires placing the object on the stack, but the byte size of `dyn Trait` is not known at compile time.
2. **Method uses generics (`fn process<T>(&self, val: T)`):** The compiler generates a distinct function instance for every `T`. Storing infinite function variants in a fixed `vtable` is impossible.
3. **Method lacks a `self` parameter (`fn new() -> Self`):** Without a `self` argument, the program cannot dispatch to a specific vtable instance because there is no object instance to inspect.
4. **Method returns or takes `Self`:** At runtime, different concrete structs may hide behind `dyn Trait`, so Rust cannot guarantee type safety when comparing or combining them.

### Practical Solution with `where Self: Sized`

If you need constructors or generic methods for static dispatch, but still want to use the trait via `dyn Trait`, add a `where Self: Sized` bound to problematic methods:

```rust
trait SafeTrait {
    fn name(&self) -> String;
    
    // This method takes Self by value and would break Object Safety.
    // The `where Self: Sized` bound excludes it from the vtable!
    fn copy_by_val(self) -> Self where Self: Sized {
        self
    }
}
```

With `where Self: Sized`, the compiler knows that this method is only available during static typing, so it simply omits it from the `dyn SafeTrait` vtable. The trait remains object-safe!

Let's see how all these mechanisms work in practice in our interactive code slides:


### 💻 Trait Objects and Dynamic Polymorphism


#### Step 1

```rust
trait Task {
    fn execute(&self) -> String;
}

struct NetworkTask;
impl Task for NetworkTask {
    fn execute(&self) -> String { "Network".to_string() }
}

struct DbTask;
impl Task for DbTask {
    fn execute(&self) -> String { "Database".to_string() }
}

fn main() {
    // Vector holds different types under a common trait object
    let tasks: Vec<Box<dyn Task>> = vec![
        Box::new(NetworkTask),
        Box::new(DbTask),
    ];
    for t in tasks {
        println!("Executing task: {}", t.execute());
    }
}
```

**Dynamic Polymorphism and dyn Trait**
- Trait object `dyn Task` enables working with heterogeneous collections where types are known only at runtime.
- For this reason, objects must live behind a pointer (e.g., `Box<dyn Task>` or `&dyn Task`).
- Invoked method resolution occurs at runtime via the virtual method table (vtable).


#### Step 2

```rust
// By default all generics have an implicit T: Sized bound.
// We relax it with ?Sized so the function can accept slices or dyn Trait.
fn print_debug<T: ?Sized + std::fmt::Debug>(val: &T) {
    println!("Value: {:?}", val);
}

fn main() {
    let s: &str = "Hello, world!"; // str is a DST (Dynamically Sized Type)
    print_debug(s);
}
```

**Implicit Sized bound and ?Sized**
- Most types have a size known at compile time (Sized) and can live on the stack.
- Dynamically Sized Types (DSTs), such as `str`, `[T]`, or `dyn Trait`, do not have a fixed size.
- Syntax `?Sized` relaxes the implicit fixed-size requirement, allowing DSTs to be passed by reference.


#### Step 3

```rust
trait Fly {
    fn fly(&self) -> String;
}

trait Swim {
    fn swim(&self) -> String;
}

// Rust forbids &(dyn Fly + Swim) directly.
// We create a combined supertrait:
trait Duck: Fly + Swim {}

// And write a Blanket Implementation for all matching types:
impl<T: Fly + Swim> Duck for T {}

struct Mallard;
impl Fly for Mallard {
    fn fly(&self) -> String { "Flying".to_string() }
}
impl Swim for Mallard {
    fn swim(&self) -> String { "Swimming".to_string() }
}

fn handle_duck(duck: &dyn Duck) {
    println!("Duck: {}, {}", duck.fly(), duck.swim());
}

fn main() {
    let d = Mallard;
    handle_duck(&d);
}
```

**Multi-Trait Objects**
- In Rust, you cannot directly combine multiple independent traits into a single trait object: `dyn Trait1 + Trait2` will not compile.
- As a workaround, we construct a supertrait `Duck` inheriting from both required traits.
- Blanket implementation automatically makes any matching type implement `Duck`.


#### Step 4

```rust
// Trait contains a `copy_by_val` method returning Self by value.
// This breaks Object Safety (dynamic compatibility).
// But we isolate it using `where Self: Sized`!
trait SafeTrait {
    fn name(&self) -> String;
    
    // This method will not be included in the vtable and won't break dyn SafeTrait
    fn copy_by_val(self) -> Self where Self: Sized {
        self
    }
}

struct Item;
impl SafeTrait for Item {
    fn name(&self) -> String { "Item".to_string() }
}

fn print_name(obj: &dyn SafeTrait) {
    println!("Name: {}", obj.name());
    // obj.copy_by_val(); // Error! Method unavailable for trait object.
}

fn main() {
    let item = Item;
    print_name(&item);
}
```

**Object Safety**
- Trait objects can only be created for Object-Safe traits (no Self by value, no generic methods, etc.).
- The `where Self: Sized` constraint allows excluding problematic methods from the vtable.
- The trait remains object-safe, while "unsafe" methods can only be invoked on static types.


---

## Test Your Knowledge!

Take a short quiz to test your understanding of trait objects, fat pointers, and dynamic polymorphism in Rust.


### 📝 Trait Objects and Dynamic Polymorphism


**1. What is the key difference between DSTs (Dynamically Sized Types) and regular Sized types?**

- [ ] DST types can only be located on the stack, while Sized types are on the heap
- [ ] DSTs have a size unknown at compile time. They cannot be stored directly on the stack or passed by value; they must be accessed behind pointers
- [ ] The compiler generates separate copies of functions for DST types during monomorphization

**2. What does a Fat Pointer to a trait object consist of in Rust?**

- [ ] A pointer to the object's data in memory and a pointer to the virtual method table (vtable)
- [ ] A pointer to the data and the length of the object (number of elements)
- [ ] A pointer to the vtable and the number of methods in the trait

**3. Which information is NOT stored in the vtable for a trait object?**

- [ ] A pointer to the destructor (Drop) for cleaning up memory
- [ ] The size and alignment requirements of the concrete type in memory
- [ ] String names of methods for dynamic lookup by name at runtime
- [ ] Addresses of compiled methods implementing the trait

**4. Which trait property violates Object Safety and prevents creating a `dyn Trait`?**

- [ ] Having methods that accept a mutable reference `&mut self`
- [ ] Having associated types
- [ ] Having static methods (without a `self` argument) that lack a `where Self: Sized` bound

**5. How does the compiler know how many bytes of memory to free in the allocator when dropping `Box<dyn Trait>`?**

- [ ] The size is calculated by scanning the heap memory bytes
- [ ] The allocator tracks all allocation sizes without Rust's involvement
- [ ] The compiler reads the size and alignment of the concrete type from the object's vtable

