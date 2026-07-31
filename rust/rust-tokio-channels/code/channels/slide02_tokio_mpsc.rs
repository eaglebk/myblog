// ?hidden:start
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
// ?hidden:end

#[tokio::main]
async fn main() {
    // Создаем асинхронный mpsc-канал с емкостью буфера на 10 элементов
    let (tx, mut rx) = mpsc::channel(10);

    // Запускаем асинхронную задачу передачи телеметрии
    tokio::spawn(async move {
        println!("[ЗОНД] Начинаем передачу показаний...");
        sleep(Duration::from_millis(300)).await;
        let _ = tx.send("Атмосферное давление: 120 кПа".to_string()).await;
    });

    println!("[МОСТИК] Ожидаем данные (неблокирующий await)...");

    // recv() возвращает Future. Метод .await приостанавливает ТОЛЬКО текущую задачу,
    // освобождая поток воркера Tokio для выполнения других задач!
    if let Some(msg) = rx.recv().await {
        println!("[МОСТИК] Принято: {}", msg);
    }

    println!("[МОСТИК] Сбор данных завершен.");
}

// ! **Асинхронный канал tokio::sync::mpsc**
// ! - Создается с помощью `mpsc::channel(capacity)`. Емкость буфера обязательна для защиты от переполнения памяти (Backpressure).
// ! - Методы `send().await` и `recv().await` не блокируют ОС-поток, а приостанавливают текущую задачу (task yield).
