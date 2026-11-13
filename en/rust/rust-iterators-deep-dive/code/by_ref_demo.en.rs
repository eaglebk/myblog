fn main() {
    let mut it = [10, 20, 30, 40, 50].into_iter();
    let header: Vec<i32> = it.by_ref().take(2).collect();
    let rest: Vec<i32> = it.collect();

    println!("header = {:?}, rest = {:?}", header, rest);
}

// ! **Taking a prefix with .by_ref()**
// ! - `take(2)` borrows `it` mutably instead of consuming it, so `it` still yields `[30, 40, 50]` afterward.

