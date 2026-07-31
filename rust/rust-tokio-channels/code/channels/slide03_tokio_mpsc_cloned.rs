// ?hidden:start
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
// ?hidden:end

async fn sensor_task(id: u32, tx: mpsc::Sender<String>) {
    for i in 1..=2 {
        sleep(Duration::from_millis(50 * id as u64)).await;
        let msg = format!("Сенсор #{} -> Показание #{}", id, i);
        let _ = tx.send(msg).await;
    }
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);

    // Запускаем два независимых сенсора, клонируя Sender
    tokio::spawn(sensor_task(1, tx.clone()));
    tokio::spawn(sensor_task(2, tx.clone()));

    // Обязательно удаляем оригинальный Sender в main!
    // Иначе канал никогда не закроется и rx.recv().await зависнет.
    drop(tx);

    println!("[ЦЕНТР] Ожидаем показания всех сенсоров...");

    // Цикл закроется автоматически, как только ВСЕ клоны Sender будут удалены из памяти
    while let Some(msg) = rx.recv().await {
        println!("[ЦЕНТР] Получено: {}", msg);
    }

    println!("[ЦЕНТР] Все сенсоры завершили работу.");
}

// ! **Паттерн Multi-producer Single-consumer (MPSC)**
// ! - Передатчик `Sender` можно клонировать через `tx.clone()` и передавать в разные фоновые задачи.
// ! - Получатель `Receiver` единственный (Single Consumer).
// ! - Чтобы цикл чтения `while let Some(...)` корректно завершился, оригинальный `tx` в main обязательно удаляется через `drop(tx)`.
