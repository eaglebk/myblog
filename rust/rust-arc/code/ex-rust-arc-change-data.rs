use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_cloned = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_cloned.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
// ! **Разберем подробнее что же тут происходит.**
// ! _Это пример демонстрации безопасного изменения общего значения из нескольких потоков:_
// ! 1. `counter` — это общая переменная, защищённая `Mutex` и обёрнутая в `Arc` (как мы уже знаем он позволяет безопасно делиться данными между потоками).
// ! 2. В цикле создаётся 10 потоков с `thread::spawn` каждый из которых:
// ! - получает свою копию `Arc` для `counter`
// ! - блокирует `Mutex` для получения доступа к данным
// ! - увеличивает значение на 1
// ! 3. `handle.join()` ждёт завершения всех потоков.

