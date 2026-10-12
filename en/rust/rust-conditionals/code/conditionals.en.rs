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
// ! **Code Blocks**
// ! - A code block is enclosed in curly braces `{}`.
// ! - If the final expression of a block does not end with `;`, its value is returned from the block.
// ! - A block ending with `;` or an empty block returns the unit type `()`.

// ---

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
// ! **Conditional if-else Expressions**
// ! - Parentheses around the condition are unnecessary (e.g., `if x > 100` instead of `if (x > 100)`).
// ! - The condition must strictly evaluate to the boolean type `bool`.
// ! - Both `if` and `else` branches must return identical types. Rust has no ternary operator; `if-else` serves this purpose.

// ---

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
// ! **if let and Chained Conditions**
// ! - `if let` provides a concise alternative to `match` when testing a single pattern.
// ! - Variables bound inside the pattern are scoped strictly to the `if let` block.
// ! - Modern Rust allows combining multiple `let` pattern bindings and boolean expressions using `&&`.
