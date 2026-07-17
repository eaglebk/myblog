// ?hidden:start
use std::time::Duration;
use tokio::time::sleep;
// ?hidden:end

async fn attack_individually(id: u32) {
    println!("[Пчела {}] Лечу атаковать Шершня в одиночку!", id);
    sleep(Duration::from_millis(50)).await;
    // Без сбора группы тепло одной пчелы мгновенно рассеивается
    println!("[Пчела {}] Меня смахнули! Мое тепло рассеялось, я погибла.", id);
}

#[tokio::main]
async fn main() {
    let mut tasks = vec![];
    for id in 1..=5 {
        tasks.push(tokio::spawn(attack_individually(id)));
    }
    for task in tasks {
        let _ = task.await;
    }
    println!("[ИТОГ] Шершень не заметил атаки. Все пчелы погибли.");
}

// ! **Хаотичная атака: без координации**
// ! - Каждая задача запускается независимо и пытается нагреться сразу же.
// ! - Без синхронизации пчелы действуют вразнобой, тепло рассеивается, и шершень легко побеждает их поодиночке.
