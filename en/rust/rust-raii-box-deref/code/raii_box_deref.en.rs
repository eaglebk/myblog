struct SensorData {
    name: String,
}

impl Drop for SensorData {
    fn drop(&mut self) {
        println!("Releasing sensor resource: {}", self.name);
    }
}

fn main() {
    {
        let sensor = SensorData {
            name: "Kitchen Temperature".to_string(),
        };
        println!("Working with sensor: {}", sensor.name);
    } // Scope ends here — Drop is automatically called!
    println!("Section completed.");
}

// ! **The RAII Pattern and the Drop Trait**
// ! - The RAII concept guarantees: resources are allocated upon creation and automatically deallocated when exiting scope.
// ! - The `Drop` trait allows overriding cleanup logic (closing files, releasing memory, logging).
// ! - The `drop` method is called automatically in reverse order of variable declaration.

// ---

struct Config {
    port: u16,
}

fn main() {
    // 1. Allocate object on the heap using Box
    let mut heap_config = Box::new(Config { port: 8080 });
    heap_config.port = 9090; // Mutable access to heap data
    println!("Heap config port: {}", heap_config.port);

    // 2. Intentionally "leak" memory to obtain a 'static reference
    let static_config: &'static mut Config = Box::leak(heap_config);
    static_config.port = 3000;
    println!("Static port after Box::leak: {}", static_config.port);
}

// ! **Smart Pointer Box<T> and Box::leak**
// ! - `Box<T>` moves an object from stack to heap, retaining ownership over it.
// ! - `Box::leak` cancels automatic `Drop` invocation and returns a mutable reference with a `'static` lifetime.
// ! - This is useful for creating global configurations or long-lived singletons at runtime.

// ---

use std::ops::{Deref, DerefMut};

struct SmartWrapper<T> {
    value: T,
}

impl<T> SmartWrapper<T> {
    fn new(value: T) -> Self {
        SmartWrapper { value }
    }
}

impl<T> Deref for SmartWrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for SmartWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    let mut wrapped_num = SmartWrapper::new(42);
    
    // Dereference operator overloading with *
    assert_eq!(*wrapped_num, 42);
    
    *wrapped_num += 10;
    println!("Updated value: {}", *wrapped_num);
}

// ! **Dereference Operator Overloading: Deref and DerefMut**
// ! - Implementing `Deref` enables using the dereference operator `*` on custom smart pointers.
// ! - `DerefMut` extends dereferencing for mutable references (`*x = new_val`).
// ! - Associated type `type Target = T` specifies what type dereferencing yields.

// ---

// Function expects a standard string slice &str
fn print_message(msg: &str) {
    println!("Message: {msg}");
}

fn main() {
    let boxed_string: Box<String> = Box::new(String::from("Hello from Deref!"));
    
    // Deref Coercion chain: &Box<String> -> &String -> &str
    print_message(&boxed_string);
    
    // Direct invocation of inner type methods via smart pointer:
    println!("String length: {}", boxed_string.len());
}

// ! **Implicit Reference Coercion (Deref Coercion)**
// ! - The compiler automatically coerces smart pointer references into inner type references: `&Box<String>` -> `&String` -> `&str`.
// ! - This applies across chains of types implementing `Deref`.
// ! - Enables calling inner type methods directly through the smart pointer (e.g., `boxed_string.len()`).
