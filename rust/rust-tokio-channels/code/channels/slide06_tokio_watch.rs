// ?hidden:start
use tokio::sync::watch;
use tokio::time::{sleep, Duration};
// ?hidden:end

async fn display_panel(id: u32, mut rx: watch::Receiver<u32>) {
    // rx.changed().await ожидает изменения значения в канале
    while rx.changed().await.is_ok() {
        let val = *rx.borrow();
        println!("[ПАНЕЛЬ #{}] Текущий уровень энергии: {}%", id, val);
    }
}

#[tokio::main]
async fn main() {
    // watch создается с начальным значением
    let (tx, rx) = watch::channel(100);

    // Подключаем панели, клонируя получатель Receiver
    tokio::spawn(display_panel(1, rx.clone()));
    tokio::spawn(display_panel(2, rx.clone()));

    sleep(Duration::from_millis(50)).await;

    println!("[РЕАКТОР] Изменение уровня энергии -> 85%");
    let _ = tx.send(85);

    sleep(Duration::from_millis(50)).await;

    println!("[РЕАКТОР] Изменение уровня энергии -> 40%");
    let _ = tx.send(40);

    sleep(Duration::from_millis(100)).await;
}

// ! **Канал состояния tokio::sync::watch**
// ! - Предназначен для наблюдения за одним текущим значением (Single-producer, Multi-consumer).
// ! - Подписчики всегда видят самое последнее состояние (`rx.borrow()`). Промежуточные значения могут пропускаться, если новое значение записано до чтения старого.
// ! - Идеален для передачи флагов конфигурации, статусов сети и параметров системы.
