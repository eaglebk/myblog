fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // No work done yet! This only builds a Map<Iter<'_, i32>, ...> struct on the stack
    let doubled = numbers.iter().map(|x| {
        println!("Processing {x}");
        x * 2
    });

    println!("Stack size: {} bytes", std::mem::size_of_val(&doubled));
    println!("Actual type: {}", std::any::type_name_of_val(&doubled));
}

// ! **Checking laziness and stack size**
// ! - Run the code: no "Processing ..." lines are printed because no consuming method is called.
// ! - `doubled` takes 16 bytes on the stack, and its type reveals the nested `Map<Iter<'_, i32>, ...>` wrapper.

