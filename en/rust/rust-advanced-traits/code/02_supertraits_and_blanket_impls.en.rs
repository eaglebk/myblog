// Step 1: Supertraits
use std::fmt::Display;

// Supertrait Loggable requires any implementing type to ALSO implement Display
trait Loggable: Display {
    fn log_with_prefix(&self, prefix: &str) {
        // We can safely use {} formatting because Self: Display!
        println!("[{prefix}] {self}");
    }
}

struct User {
    username: String,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User({})", self.username)
    }
}

// Now we can implement Loggable for User:
impl Loggable for User {}

fn main() {
    let u = User { username: String::from("Alex") };
    u.log_with_prefix("INFO");
}

// ! **1. Supertraits**
// ! - The syntax `trait Loggable: Display` imposes a bound: the type must implement the base trait `Display`.
// ! - Allows using methods from the base trait inside the supertrait implementation.

// ---

// Step 2: Blanket Implementations
use std::fmt::Display as StandardDisplay;

trait Summary {
    fn print_summary(&self);
}

// Blanket Implementation: implements Summary for ALL types T that implement Display!
impl<T: StandardDisplay> Summary for T {
    fn print_summary(&self) {
        println!("Automatic summary output for type: {self}");
    }
}

fn main() {
    // All standard library and custom types with Display automatically receive print_summary():
    let num = 42;
    num.print_summary();
    "Hello, Rust!".print_summary();
    true.print_summary();
}

// ! **2. Blanket Implementations**
// ! - The syntax `impl<T: TraitA> TraitB for T` automatically implements `TraitB` for ALL types satisfying `TraitA`.
// ! - Widely used across the standard library (e.g., `ToString` is implemented for all types implementing `Display`).
