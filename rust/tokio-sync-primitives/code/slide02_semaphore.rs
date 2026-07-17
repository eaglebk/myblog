// ?hidden:start
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::sleep;
// ?hidden:end

async fn try_cling(id: u32, semaphore: Arc<Semaphore>) {
    println!("[Пчела {}] Ищет свободное место на теле Шершня...", id);
    
    // Получаем разрешение (permit) на посадку. Если мест нет — ждем.
    let _permit = semaphore.acquire().await.unwrap();
    
    println!("[Пчела {}] Зацепилась за панцирь! (Место занято)", id);
    sleep(Duration::from_millis(200)).await; // Удерживаем место на Шершне
    
    println!("[Пчела {}] Отпускает Шершня. (Место освободилось)", id);
    // _permit автоматически возвращается в семафор при выходе из области видимости
}

#[tokio::main]
async fn main() {
    // На теле Шершня одновременно могут закрепиться только 2 пчелы
    let hornet_surface = Arc::new(Semaphore::new(2));
    let mut tasks = vec![];

    for id in 1..=4 {
        let sem = hornet_surface.clone();
        tasks.push(tokio::spawn(try_cling(id, sem)));
    }

    for task in tasks {
        let _ = task.await;
    }
}

// ! **Семафор: ограничение доступа к ресурсу**
// ! - `tokio::sync::Semaphore` регулирует доступ к ограниченному ресурсу (2 места на теле Шершня).
// ! - Вызов `acquire().await` блокирует задачу асинхронно, если все разрешения (permits) заняты.
// ! - Разрешение автоматически освобождается при уничтожении `SemaphorePermit`.
