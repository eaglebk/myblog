use tokio::time::{sleep, Duration};

async fn slow(id: u32) {
    sleep(Duration::from_secs(1)).await;
    println!("Готово {id}");
}

#[tokio::main]
async fn main() {
    let handles = (1..=3)
        .map(|i| tokio::spawn(slow(i)))
        .collect::<Vec<_>>();

    for h in handles {
        let _ = h.await;
    }
    println!("Все задачи завершены");
}

// ! tokio::spawn возвращает JoinHandle; собираем и ждём всех.
