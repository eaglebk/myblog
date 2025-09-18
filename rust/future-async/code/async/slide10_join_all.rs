// ?hidden:start
use tokio::time::{sleep, Duration};
use futures::future::join_all;
// ?hidden:end
async fn work(i: u32) -> u32 {
    sleep(Duration::from_millis(50 * i as u64)).await;
    i * 2
}

#[tokio::main]
async fn main() {
    let futs = vec![work(1), work(2), work(3)];
    let results = join_all(futs).await;
    println!("Результаты: {:?}", results); // [2,4,6]
}
// ! Используем join_all из пакета futures
// ! - Он ждёт коллекцию Future и возвращает Vec результатов.
