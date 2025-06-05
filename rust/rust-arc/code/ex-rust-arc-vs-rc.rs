use std::rc::Rc;
use std::thread;

fn main() {
    let data = Rc::new(vec![1, 2, 3]);

    let handle = thread::spawn(move || {
        println!("{:?}", data);
    });

    handle.join().unwrap();
}
// ! **1. Этот код не скомпилируется.**
// ! - Ошибка: `Rc<Vec<i32>> cannot be sent between threads safely`

// ---

use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);
    let data_cloned = Arc::clone(&data);

    let handle = thread::spawn(move || {
        println!("{:?}", data_cloned);
    });

    handle.join().unwrap();
}
// ! **2. Теперь всё работает корректно.**