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

// ! **1. Destructuring and @-bindings**
// ! - Patterns allow dissecting nested structs and enums in a single expression without intermediate variables.
// ! - The `@` operator binds the matched value to a variable while simultaneously checking a range or pattern.

// ---

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

// ! **2. Match Guards**
// ! - A Match Guard (`if CONDITION`) adds an arbitrary runtime condition to a `match` arm.
// ! - The compiler evaluates arms sequentially: place more specific guarded arms before general ones.
