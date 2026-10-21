// Step 1: &'static str reference vs T: 'static trait bound
fn print_static(msg: &'static str) {
    println!("String literal embedded in the binary data section: {msg}");
}

fn spawn_worker<T: 'static>(val: T) {
    // T: 'static means: type T either owns its data without references (String, Vec, u32),
    // or contains only 'static references.
    println!("Value is self-contained and can live for an arbitrary duration.");
}

fn main() {
    print_static("Hello from the binary!");
    
    let owned_string = String::from("Dynamically allocated owned String");
    spawn_worker(owned_string);
}

// ! **1. Static Memory 'static**
// ! - `&'static str` points to data embedded in the binary that persists for the entire program execution.
// ! - The `T: 'static` bound does NOT mean the value lives forever; it ensures `T` contains no short-lived references.

// ---

// Step 2: Higher-Rank Trait Bounds (for<'a>)

// Who determines the reference lifetime?
// If we declared 'a on the outer function: fn process<'a, F>(f: F) where F: Fn(&'a str),
// the caller would have to provide the reference beforehand. We could not pass local data!
// With `for<'a>`, the closure promises to accept a reference of ANY lifetime at call time.
fn process_incoming_event<F>(f: F)
where
    F: for<'a> Fn(&'a str) -> String,
{
    // Data is created locally inside the function (e.g. read from a network socket):
    let local_payload = String::from("EVENT_ID_42: telemetry data");
    
    // Pass a reference to the local variable into the closure:
    let summary = f(&local_payload);
    println!("Processing result: {summary}");
}

fn main() {
    // Pass a closure capable of working with any temporary slice:
    process_incoming_event(|raw_data| {
        format!("Processed {} characters", raw_data.len())
    });
}

// ! **2. Lifetimes on the fly (for<'a>)**
// ! - Who chooses lifetime length: with 'a on the outer function, the caller fixes the lifetime before the call.
// ! - The `for<'a>` syntax defers the decision: the closure accepts references to local stack data created inside the function.
// ! - This pattern underpins callbacks in async frameworks (Tokio, Actix, Axum).

// ---

// Step 3: Variance — Covariance and Invariance
fn inspect_slice<'a>(slice: &'a str) {
    println!("Inspecting slice: {slice}");
}

fn main() {
    // 1. Covariance (&'a T):
    // A reference with a longer lifetime ('static) can be passed where a shorter lifetime ('a) is expected.
    let static_literal: &'static str = "Static data";
    inspect_slice(static_literal);

    // 2. Invariance (&'a mut T):
    // For mutable references, lifetime substitution is strictly forbidden!
    let mut data: &'static str = "Initial text";
    {
        let short_lived = String::from("Short string");
        // If Rust allowed assigning &mut data a reference to short_lived,
        // once short_lived was dropped, data would hold a dangling pointer!
        let _ref_mut: &mut &'static str = &mut data;
    }
    println!("Invariance prevents mutable references from dangling mutations: {data}");
}

// ! **3. Variance**
// ! - Covariance (`&'a T`): Rust safely shortens longer-lived references (such as `'static`) to match a local `'a`.
// ! - Invariance (`&'a mut T`): For mutable references, lifetime substitution is forbidden to prevent writing dangling pointers.
