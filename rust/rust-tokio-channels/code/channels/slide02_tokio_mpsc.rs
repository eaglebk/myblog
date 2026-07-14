// ?hidden:start
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
// ?hidden:end

#[tokio::main]
async fn main() {
    // Создаем асинхронный mpsc-канал с емкостью буфера на 10 элементов
    let (tx, mut rx) = mpsc::channel(10);

    // Запускаем асинхронную задачу в Tokio
    tokio::spawn(async move {
        println!("[НАВИГАЦИЯ] Начинаем расчет...");
        sleep(Duration::from_millis(300)).await;
        let _ = tx.send("Траектория рассчитана асинхронно!".to_string()).await;
    });

    println!("[МОСТИК] Ожидаем данные (неблокирующий await)...");

    // recv() возвращает Future. Она приостанавливает выполнение нашей задачи,
    // но воркер-поток Tokio продолжает выполнять другие задачи в это время!
    if let Some(msg) = rx.recv().await {
        println!("[МОСТИК] Принято: {}", msg);
    }

    println!("[МОСТИК] Сближение завершено.");
}

// ! **Асинхронный канал tokio::sync::mpsc**
// ! - Создается с помощью `mpsc::channel(capacity)`. Емкость буфера обязательна для защиты от переполнения памяти (backpressure).
// ! - Методы `send().await` и `recv().await` не блокируют ОС-поток, а приостанавливают выполнение текущей задачи (task yield).
