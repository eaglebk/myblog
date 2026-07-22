use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;
use std::thread;

// Шаг 1: Синхронизация данных через Acquire/Release барьер памяти
fn main() {
    let data = Arc::new(AtomicU8::new(0));
    let ready_flag = Arc::new(AtomicBool::new(false));

    let data_writer = Arc::clone(&data);
    let flag_writer = Arc::clone(&ready_flag);

    // Поток-Писатель публикует данные
    thread::spawn(move || {
        data_writer.store(42, Ordering::Relaxed);
        // Ordering::Release гарантирует, что ВСЕ предыдущие записи видны после этого момента
        flag_writer.store(true, Ordering::Release);
    });

    let data_reader = Arc::clone(&data);
    let flag_reader = Arc::clone(&ready_flag);

    // Поток-Читатель ожидает флаг
    let handle = thread::spawn(move || {
        // Ordering::Acquire создает барьер чтения
        while !flag_reader.load(Ordering::Acquire) {
            thread::yield_now();
        }
        // Защищено барьером: data_reader гарантированно видит значение 42!
        println!("Прочитанные данные через Acquire/Release: {}", data_reader.load(Ordering::Relaxed));
    });

    handle.join().unwrap();
}

// ! **1. Барьеры памяти Acquire и Release**
// ! - `Ordering::Release` создаёт барьер при записи: гарантирует, что все операции с памятью ДО записи не будут переупорядочены ПОСЛЕ неё.
// ! - `Ordering::Acquire` создаёт барьер при чтении: гарантирует, что операции ПОСЛЕ чтения не будут выполнены ДО него.
// ! - Пара `Release` + `Acquire` синхронизирует состояние между потоками без блокировок.

// ---

use std::sync::atomic::{AtomicUsize, Ordering};

// Шаг 2: Атомарная операция Compare-And-Swap (CAS) через compare_exchange
fn main() {
    let current_val = AtomicUsize::new(10);

    // Сравниваем: если текущее значение равно 10, заменяем его на 20:
    let result = current_val.compare_exchange(
        10,                // Ожидаемое значение
        20,                // Новое значение
        Ordering::SeqCst,  // Порядок при успехе
        Ordering::Relaxed  // Порядок при ошибке
    );

    match result {
        Ok(prev) => println!("Успешный CAS: предыдущее значение было {prev}, стало {}", current_val.load(Ordering::Relaxed)),
        Err(actual) => println!("CAS провален: фактическое значение {actual}"),
    }
}

// ! **2. Атомарный Compare-And-Swap (CAS)**
// ! - Функция `compare_exchange` является фундаментом всех lock-free структур данных (очередей, стеков, каналов).
// ! - Она проверяет ожидаемое значение и обновляет его ровно за одну атомарную операцию процессора.
