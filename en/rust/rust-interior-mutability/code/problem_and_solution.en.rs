// ?compile_fail

struct Button {
    label: String,
    click_count: u32,
}

impl Button {
    fn on_click(&self) {
        println!("Button '{}' clicked!", self.label);
        self.click_count += 1;
    }
}

fn main() {
    let btn = Button {
        label: String::from("Submit"),
        click_count: 0,
    };
    btn.on_click();
}

// ! **Without Cell: Compilation Error**
// ! - The compiler prohibits modifying `click_count` through an immutable reference `&self`.
// ! - Declaring the method as `&mut self` is often impossible in UI components, as buttons are accessed from multiple places simultaneously.

// ---

use std::cell::Cell;

struct Button {
    label: String,
    click_count: Cell<u32>,
}

impl Button {
    fn on_click(&self) {
        println!("Button '{}' clicked!", self.label);
        let count = self.click_count.get();
        self.click_count.set(count + 1);
    }
}

fn main() {
    let btn = Button {
        label: String::from("Submit"),
        click_count: Cell::new(0),
    };

    btn.on_click();
    btn.on_click();
    println!("Total button clicks: {}", btn.click_count.get());
}

// ! **Solution using Cell<T>: Compiles Successfully**
// ! - Wrap the counter in `Cell<u32>`.
// ! - The `on_click(&self)` method successfully updates the inner value via `.set()` while preserving the `&self` reference.
