use std::sync::Arc;
use std::thread;

// Шаг 1: Почему Rc не является Send, а Arc — является
fn main() {
    // Arc<T> (Atomic Reference Counting) использует атомарные операции для счетчика ссылок,
    // поэтому Arc<T> реализует Send + Sync (при T: Send + Sync) и может передаваться между потоками!
    let shared_data = Arc::new(vec![1, 2, 3]);
    let data_clone = Arc::clone(&shared_data);

    let handle = thread::spawn(move || {
        println!("Атомарный Arc в фоновом потоке: {:?}", data_clone);
    });

    handle.join().unwrap();
    println!("Главный поток удерживает Arc, сильные ссылки: {}", Arc::strong_count(&shared_data));
}

// ! **1. Почему Rc не реализует Send**
// ! - `Rc<T>` изменяет счетчик ссылок через обычное неатомарное инкрементирование (`count += 1`).
// ! - Если бы `Rc<T>` передавался в другой поток, одновременный `clone` или `drop` привел бы к состояниям гонки (Data Race) и порче памяти.
// ! - Для многопоточного разделяемого владения строго используется `Arc<T>`.

// ---

use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;

// Шаг 2: Почему RefCell является Send, но НЕ является Sync
fn main() {
    // 1. RefCell<T> реализует Send (его можно полностью переместить в другой поток)
    let cell = RefCell::new(42);
    let handle = thread::spawn(move || {
        *cell.borrow_mut() += 10;
        println!("RefCell перемещен и изменен в другом потоке: {}", cell.borrow());
    });
    handle.join().unwrap();

    // 2. Для одновременного доступа к данным из НЕСКОЛЬКИХ потоков по &T используется Arc<Mutex<T>>
    let mutex_data = Arc::new(Mutex::new(vec!["данные"]));
    let mutex_clone = Arc::clone(&mutex_data);

    let thread_handle = thread::spawn(move || {
        let mut lock = mutex_clone.lock().unwrap();
        lock.push("из фонового потока");
    });

    thread_handle.join().unwrap();
    println!("Результирующий вектор под Mutex: {:?}", mutex_data.lock().unwrap());
}

// ! **2. Почему RefCell реализует Send, но НЕ реализует Sync**
// ! - `RefCell<T>` проверяет динамические заимствования (`borrow()` / `borrow_mut()`) через неатомарный счетчик во флаге.
// ! - Поэтому передать САМ `RefCell` в другой поток можно (`Send`), но передать ССЫЛКУ `&RefCell` в несколько потоков одновременно НЕЛЬЗЯ (`!Sync`).
// ! - Для безопасной внутренней мутабельности между потоками по ссылки используется `Mutex<T>` или `RwLock<T>`.
