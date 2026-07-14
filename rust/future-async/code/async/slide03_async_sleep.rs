use tokio::time::{sleep, Duration};

async fn query_sensor_async() -> u32 {
    sleep(Duration::from_millis(50)).await;
    return 42
}

async fn process_async(d: u32) -> u32 { d }

async fn calibrate_async() -> u32 {
    let d = query_sensor_async().await;
    sleep(Duration::from_secs(1)).await; // уступаем планировщику
    process_async(d).await
}

#[tokio::main]
async fn main() {
    println!("[МОСТИК] Запуск калибровки (async)...");
    let data = calibrate_async().await;
    println!("[МОСТИК] Калибровка завершена: {data}");
}

// ! Комментарий:
// ! Асинхронные точки ожидания .await — место где задача уступает поток выполнения рантайму (cooperative).
