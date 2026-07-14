use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

async fn unit(i: u32) -> u32 {
    sleep(Duration::from_millis(30 * i as u64)).await;
    i * 100
}

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    for i in 1..=5 {
        set.spawn(unit(i));
    }

    while let Some(res) = set.join_next().await {
        match res {
            Ok(v) => println!("Готов блок: {v}"),
            Err(e) => eprintln!("Ошибка задачи: {e}"),
        }
    }
    println!("JoinSet завершил все задачи");
}

// ! JoinSet позволяет получать результаты по мере готовности (в любом порядке).
