
// ?hidden:start
use std::time::Duration;
use std::thread;
use tokio::task;

fn query_sensor() -> u32 { 42 }
fn process(d: u32) -> u32 { d }

fn calibrate_blocking() -> u32 {
    let d = query_sensor();
    thread::sleep(Duration::from_secs(5)); // блокируем, но в отдельном потоке
    process(d)
}

// ?hidden:end

#[tokio::main]
async fn main() {
    println!("[МОСТИК] Запуск калибровки (spawn_blocking)...");
    let data = task::spawn_blocking(|| calibrate_blocking())
        .await
        .expect("join error");
    println!("[МОСТИК] Калибровка завершена: {data}");
}

// ! Комментарий
// ! - spawn_blocking уводит блокирующую работу в пул блокирующих потоков.
// ! - Это спасает рантайм, но каждый поток — требует дополнительную память; используйте осторожно.
