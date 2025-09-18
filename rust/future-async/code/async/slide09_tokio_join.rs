use tokio::time::{sleep, Duration};

async fn job(id: u32) -> u32 {
    sleep(Duration::from_millis(60 * id as u64)).await;
    println!("Задача {id} завершена");
    id * 10
}

#[tokio::main]
async fn main() {
    let (a, b) = tokio::join!(job(1), job(2));
    println!("Результаты: {a}, {b}");
}

// ! Для фиксированного набора задач используйте tokio::join!
