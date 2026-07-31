// ?hidden:start
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};
// ?hidden:end

async fn subsystem_worker(name: &'static str, mut rx: broadcast::Receiver<String>) {
    while let Ok(alert) = rx.recv().await {
        println!("[{}] Сигнал тревоги принят: {}", name, alert);
    }
}

#[tokio::main]
async fn main() {
    // broadcast создается с обязательным указанием емкости буфера
    let (tx, _rx) = broadcast::channel(16);

    // Подписываем три независимые системы с помощью tx.subscribe()
    tokio::spawn(subsystem_worker("ДВИГАТЕЛИ", tx.subscribe()));
    tokio::spawn(subsystem_worker("ЩИТЫ", tx.subscribe()));
    tokio::spawn(subsystem_worker("ЖИЗНЕОБЕСПЕЧЕНИЕ", tx.subscribe()));

    sleep(Duration::from_millis(100)).await;

    println!("[ЦЕНТР] Рассылка экстренного уведомления...");
    // Каждое сообщение получат ВСЕ три подписчика одновременно!
    let _ = tx.send("ВНИМАНИЕ: Скачок гравитационного поля!".to_string());

    sleep(Duration::from_millis(200)).await;
}

// ! **Широковещательный канал tokio::sync::broadcast**
// ! - Модель Multi-producer Multi-consumer: каждое отправленное сообщение получают ВСЕ подписчики.
// ! - Новые подписчики подключаются вызовом `tx.subscribe()`.
// ! - Подходит для сигналов завершения работы (Graceful Shutdown), аварйных оповещений и шины событий.
