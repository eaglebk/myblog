use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

// Шаг 1: Атомарный счетчик AtomicUsize без мьютекса
fn main() {
    // AtomicUsize изменяется по разделяемой иммутабельной ссылке (&self) без блокировки Mutex!
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                // fetch_add атомарно прибавляет 1 и возвращает предыдущее значение
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Итоговое значение атомарного счетчика: {}", counter.load(Ordering::Relaxed));
}

// ! **1. Атомарные скалярные примитивы**
// ! - Типы `AtomicUsize`, `AtomicBool`, `AtomicI32` позволяют изменять значения по иммутабельной ссылке `&self` (Interior Mutability на уровне железа).
// ! - Операции `fetch_add`, `fetch_sub`, `store` и `load` выполняются одной непрерывной процессорной инструкцией.

// ---

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// Шаг 2: Флаг останова фонового потока на основе AtomicBool
fn main() {
    let running_flag = Arc::new(AtomicBool::new(true));
    let flag_clone = Arc::clone(&running_flag);

    let handle = thread::spawn(move || {
        let mut iterations = 0;
        while flag_clone.load(Ordering::Relaxed) {
            iterations += 1;
            thread::sleep(Duration::from_millis(10));
        }
        println!("Фоновый поток завершен после {} итераций", iterations);
    });

    thread::sleep(Duration::from_millis(50));
    // Сигнализируем фоновому потоку о необходимости останова
    running_flag.store(false, Ordering::Relaxed);

    handle.join().unwrap();
}

// ! **2. Атомарные флаги остановки (Lock-Free Signals)**
// ! - Использование `AtomicBool` для передачи сигналов остановки между потоками не требует тяжелой блокировки мьютекса.
