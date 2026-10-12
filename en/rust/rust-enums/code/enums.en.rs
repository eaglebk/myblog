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
// ! **Enum Variant Construction**
// ! - Enums can combine different data shapes under a single unified type.
// ! - Unit variants carry no associated data (`PageLoad`).
// ! - Tuple variants hold unnamed data inside parentheses (`KeyPress`).
// ! - Struct variants hold named fields inside curly braces (`Click`).

// ---

// Defining an uninhabited type (an enum without variants)
#[allow(dead_code)]
enum EmptyEnum {}

fn main() {
    // Creating an instance of EmptyEnum is impossible because it has no variants!
    println!("EmptyEnum is defined, but cannot be instantiated.");
}
// ! **Uninhabited Types**
// ! - An enum with no variants is called an uninhabited type.
// ! - Instantiating such a type in safe code is impossible.
// ! - Useful in the type system to represent impossible states (e.g., an error that can never happen).

// ---

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
// ! **Extracting Data with match**
// ! - The `match` expression guarantees safe access to data stored inside enum variants.
// ! - The Rust compiler enforces exhaustiveness: every possible variant must be handled.
// ! - Data is destructured directly inside the match arm pattern.
