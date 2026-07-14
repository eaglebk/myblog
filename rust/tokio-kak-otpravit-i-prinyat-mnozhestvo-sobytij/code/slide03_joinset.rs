// ?hidden:start
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};
// ?hidden:end

async fn scan_sector(sector_id: u32) -> (u32, &'static str) {
    // Разное время сканирования секторов для имитации асинхронности
    let delay = match sector_id {
        1 => 150,
        2 => 50,
        3 => 250,
        _ => 100,
    };
    sleep(Duration::from_millis(delay)).await;
    
    match sector_id {
        2 => (sector_id, "Обнаружен спасательный челнок доктора!"),
        _ => (sector_id, "Сектор пуст"),
    }
}

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();

    // Запускаем 3 задачи сканирования секторов динамически
    for sector_id in 1..=3 {
        set.spawn(scan_sector(sector_id));
    }

    // Обрабатываем результаты по мере их завершения
    while let Some(res) = set.join_next().await {
        match res {
            Ok((sector, status)) => {
                println!("[ЗОНД] Сектор {} завершил сканирование: {}", sector, status);
                if status.contains("челнок") {
                    println!("[МОСТИК] Цель найдена! Прекращаем сканирование остальных секторов.");
                    set.abort_all(); // Отменяем все оставшиеся задачи
                    break;
                }
            }
            Err(e) => println!("[ОШИБКА] Задача прервана: {:?}", e),
        }
    }
}

// ! **Менеджер задач tokio::task::JoinSet**
// ! - Динамический пул конкурентных задач, возвращающий результаты по мере их готовности.
// ! - Позволяет легко прервать все оставшиеся задачи через `set.abort_all()`.
// ! - Решает проблему утечки ресурсов при отмене и упрощает сбор результатов по сравнению с `join_all`.
