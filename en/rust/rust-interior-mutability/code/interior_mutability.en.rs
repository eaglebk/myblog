use std::cell::Cell;

struct Logger {
    log_count: Cell<usize>,
}

impl Logger {
    fn new() -> Self {
        Logger {
            log_count: Cell::new(0),
        }
    }

    // Note: The method takes &self (immutable reference!), but mutates log_count!
    fn log(&self, message: &str) {
        println!("[LOG]: {message}");
        let current = self.log_count.get();
        self.log_count.set(current + 1);
    }
}

fn main() {
    let logger = Logger::new();

    logger.log("First system log message");
    logger.log("Second system log message");

    println!("Total log entries: {}", logger.log_count.get());

    // The replace method replaces the value and returns the old one
    let old_val = logger.log_count.replace(100);
    println!("Old count: {old_val}, new count: {}", logger.log_count.get());
}

// ! **Interior Mutability with Cell<T>**
// ! - `Cell<T>` allows modifying values inside an object holding only an immutable reference `&self`.
// ! - Writing via `.set()` and reading via `.get()` works by copying or moving values without issuing direct references `&T`.
// ! - Ideal for simple scalar (`Copy`) types, flags, and counters.

// ---

use std::cell::RefCell;

struct UserCache {
    cache: RefCell<Vec<String>>,
}

impl UserCache {
    fn new() -> Self {
        UserCache {
            cache: RefCell::new(Vec::new()),
        }
    }

    fn add_user(&self, name: String) {
        // Request a mutable borrow at runtime
        self.cache.borrow_mut().push(name);
    }

    fn print_users(&self) {
        // Request an immutable borrow
        let users = self.cache.borrow();
        println!("Cached users: {:?}", *users);
    }
}

fn main() {
    let cache = UserCache::new();

    cache.add_user("Alice".to_string());
    cache.add_user("Bob".to_string());

    cache.print_users();

    // Check cache size
    println!("Total users count: {}", cache.cache.borrow().len());
}

// ! **Dynamic Borrow Checking with RefCell<T>**
// ! - `RefCell<T>` allows mutating complex structures on the heap via `borrow_mut()` through `&self` references.
// ! - Unlike `Cell`, `RefCell` hands out temporary smart references `Ref` and `RefMut`.
// ! - Borrow checks are deferred to runtime. Attempting to hold two `borrow_mut()` handles simultaneously triggers a panic.
