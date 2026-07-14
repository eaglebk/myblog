// ?hidden:start
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};
// ?hidden:end

#[tokio::main]
async fn main() {
    // Создаем broadcast-канал с буфером на 2 сообщения
    let (tx, mut rx1) = broadcast::channel::<String>(2);
    let mut rx2 = tx.subscribe();

    // Первый получатель: Модуль жизнеобеспечения (быстрый)
    tokio::spawn(async move {
        loop {
            match rx1.recv().await {
                Ok(msg) => println!("[ЖИЗНЕОБЕСПЕЧЕНИЕ] Получено: {}", msg),
                Err(broadcast::error::RecvError::Closed) => break,
                Err(e) => println!("[ЖИЗНЕОБЕСПЕЧЕНИЕ] Ошибка: {:?}", e),
            }
        }
    });

    // Второй получатель: Двигательный отсек (медленный)
    tokio::spawn(async move {
        loop {
            // Имитируем долгую обработку, из-за чего буфер переполнится
            sleep(Duration::from_millis(300)).await;
            match rx2.recv().await {
                Ok(msg) => println!("[ДВИГАТЕЛИ] Получено: {}", msg),
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    println!("[ДВИГАТЕЛИ] Отстали! Пропущено {} сообщений.", n);
                }
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    });

    // Отправляем сообщения с мостика
    sleep(Duration::from_millis(50)).await;
    let _ = tx.send("Сигнал 1: Прогрев систем".to_string());
    let _ = tx.send("Сигнал 2: Калибровка щитов".to_string());
    let _ = tx.send("Сигнал 3: Запуск реактора".to_string());
    let _ = tx.send("Сигнал 4: Экстренный старт!".to_string());

    // Даем время задачам завершиться
    sleep(Duration::from_millis(1000)).await;
}

// ! **Канал tokio::sync::broadcast**
// ! - Отправка сообщений по схеме "многие ко многим" (Multi-producer Multi-consumer).
// ! - Если буфер переполняется из-за медленного получателя, Tokio выбрасывает `RecvError::Lagged`.
// ! - Идеально подходит для общих событий, таких как сигналы остановки или глобальные уведомления.
