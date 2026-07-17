// ?compile_fail
// ?hidden:start
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;
// ?hidden:end

async fn deal_damage_bad(id: u32, hornet_hp: Arc<Mutex<i32>>) {
    // Получаем блокировку стандартного std::sync::Mutex
    let mut hp = hornet_hp.lock().unwrap();
    *hp -= 10;
    println!("[Пчела {}] Разогрела панцирь. Текущее HP Шершня: {}", id, *hp);
    
    // Пытаемся уснуть, удерживая std::sync::MutexGuard.
    // ОШИБКА: MutexGuard не реализует Send, и Tokio не сможет перенести
    // эту задачу на другой поток после .await!
    sleep(Duration::from_millis(10)).await; 
}

#[tokio::main]
async fn main() {
    let hornet_hp = Arc::new(Mutex::new(100));
    // Этот код не скомпилируется!
    tokio::spawn(deal_damage_bad(1, hornet_hp.clone())).await.unwrap();
}

// ! **Ошибка: удержание std::sync::MutexGuard через .await**
// ! - Стандартный `std::sync::MutexGuard` не является `Send`, так как привязан к конкретному потоку ОС.
// ! - Если удерживать его во время вызова `.await`, компилятор выдаст ошибку, так как Tokio не сможет безопасно переносить задачу между потоками во время ожидания.

// ---

// ?hidden:start
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;
// ?hidden:end

async fn deal_damage_good(id: u32, hornet_hp: Arc<Mutex<i32>>) {
    // Асинхронный Tokio MutexGuard безопасен для удерживания через .await
    let mut hp = hornet_hp.lock().await;
    *hp -= 10;
    println!("[Пчела {}] Разогрела панцирь. Текущее HP Шершня: {}", id, *hp);
    
    // Спим, удерживая блокировку. Это работает, но блокирует остальных!
    sleep(Duration::from_millis(10)).await;
}

#[tokio::main]
async fn main() {
    let hornet_hp = Arc::new(Mutex::new(100));
    let mut tasks = vec![];
    for id in 1..=3 {
        let hp = hornet_hp.clone();
        tasks.push(tokio::spawn(deal_damage_good(id, hp)));
    }
    for task in tasks {
        let _ = task.await;
    }
    println!("[ИТОГ] Здоровье Шершня успешно уменьшено до 70.");
}

// ! **Решение: Использование асинхронного tokio::sync::Mutex**
// ! - `tokio::sync::Mutex` возвращает guard, который реализует `Send` и может безопасно удерживаться через точки `.await`.
// ! - Альтернатива для производительности — использовать `std::sync::Mutex`, но ограничить область его видимости фигурными скобками `{}` так, чтобы блокировка снималась ДО вызова `.await`.
