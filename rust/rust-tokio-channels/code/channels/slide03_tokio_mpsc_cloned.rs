// ?hidden:start
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
// ?hidden:end

async fn sensor_task(id: u32, tx: mpsc::Sender<String>) {
    for i in 1..=3 {
        sleep(Duration::from_millis(50 * id as u64)).await;
        let msg = format!("Датчик {} -> Значение {}", id, i);
        let _ = tx.send(msg).await;
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);

    // Запускаем два параллельных датчика, каждый со своей копией Sender
    tokio::spawn(sensor_task(1, tx.clone()));
    tokio::spawn(sensor_task(2, tx.clone()));

    // Важно: удаляем исходный Sender, иначе цикл rx.recv() никогда не завершится,
    // так как канал будет считаться открытым
    drop(tx);

    println!("[ЦЕНТР] Ожидаем показания датчиков...");

    // Канал закроется автоматически, когда все отправители (Sender) будут удалены
    while let Some(msg) = rx.recv().await {
        println!("[ЦЕНТР] Получено: {}", msg);
    }

    println!("[ЦЕНТР] Все датчики отключены.");
}

// ! **Паттерн Multi-producer Single-consumer (MPSC)**
// ! - Мы можем клонировать `Sender` с помощью `tx.clone()` и передавать его в разные асинхронные задачи.
// ! - Получатель `Receiver` существует только в одном экземпляре.
// ! - Чтобы цикл чтения завершился при выходе всех задач, исходный `tx` на мостике должен быть явно удален через `drop(tx)`.
