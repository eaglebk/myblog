// ?hidden:start
use tokio::sync::oneshot;
use tokio::time::{sleep, Duration};
// ?hidden:end

async fn compute_density(respond_to: oneshot::Sender<f64>) {
    println!("[ЯДРО] Вычисляем плотность атмосферы...");
    sleep(Duration::from_millis(200)).await;
    let result = 1.4159;
    // oneshot send() забирает владение tx и отправляет ровно ОДНО сообщение
    let _ = respond_to.send(result);
}

#[tokio::main]
async fn main() {
    // oneshot не требует указания capacity — вместимость всегда равна 1
    let (tx, rx) = oneshot::channel();

    tokio::spawn(compute_density(tx));

    println!("[МОСТИК] Запрос отправлен. Ожидаем результат...");

    match rx.await {
        Ok(density) => println!("[МОСТИК] Рассчитанная плотность: {:.4} кг/м³", density),
        Err(_) => println!("[МОСТИК] Задача была отменена до отправки ответа!"),
    }
}

// ! **Канал tokio::sync::oneshot (Запрос–Ответ)**
// ! - Предназначен для передачи ровно ОДНОГО сообщения между единичным отправителем и единичным получателем.
// ! - Идеально подходит для паттерна Request–Response (RPC): фоновая задача получает `oneshot::Sender` и возвращает результат вычислений.
// ! - Вызов `rx.await` возвращает `Err`, если отправитель завершился или был уничтожен без отправки ответа.
