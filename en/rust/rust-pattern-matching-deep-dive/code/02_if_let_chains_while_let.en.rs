// Step 1: Combining conditions with if-let chains (let_chains)
fn main() {
    let result: Option<Result<i32, &str>> = Some(Ok(42));

    // if-let chain combining pattern matching and boolean expressions with &&:
    if let Some(res) = result && let Ok(val) = res && val > 10 {
        println!("Successfully extracted value greater than 10: {val}");
    }
}

// ! **1. if-let chains (let_chains)**
// ! - In modern Rust, multiple `let` pattern bindings and boolean conditions can be chained using `&&`.
// ! - This eliminates deeply nested `if let` blocks and keeps logic flat and readable.

// ---

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

// ! **2. Concise while let loop**
// ! - The `while let` construct runs repeatedly as long as the expression matches the specified pattern.
// ! - Perfect for working with iterators, stack buffers, and data queues.
