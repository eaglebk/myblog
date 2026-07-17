// ?hidden:start
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Barrier;
use tokio::time::sleep;
// ?hidden:end

async fn attack_with_barrier(id: u32, barrier: Arc<Barrier>) {
    println!("[Пчела {}] Зацепилась и ждет остальных...", id);
    
    // Ждем, пока все участники группы (3 пчелы) достигнут этой точки
    barrier.wait().await;
    
    // Все 3 пчелы на месте — запускаем нагрев одновременно!
    println!("[Пчела {}] Вспышка! Запускаем термо-нагрев!", id);
    sleep(Duration::from_millis(50)).await;
}

#[tokio::main]
async fn main() {
    // Барьер ожидает ровно 3 участников для проведения атаки
    let attack_barrier = Arc::new(Barrier::new(3));
    let mut tasks = vec![];

    for id in 1..=3 {
        let barrier = attack_barrier.clone();
        tasks.push(tokio::spawn(attack_with_barrier(id, barrier)));
    }

    for task in tasks {
        let _ = task.await;
    }
    println!("[ИТОГ] Шершень побежден тепловым шаром!");
}

// ! **Барьер: одновременный запуск группы задач**
// ! - `tokio::sync::Barrier` приостанавливает задачи до тех пор, пока нужное число участников не вызовет `wait()`.
// ! - Как только пришел последний участник, все задачи возобновляют работу одновременно.
// ! - Это гарантирует синхронный разогрев всей группы пчел в один момент.
