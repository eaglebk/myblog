use tokio::task;

async fn looper(name: &str) {
    for i in 0..3 {
        println!("{name}-{i}");
        task::yield_now().await;
    }
}

#[tokio::main]
async fn main() {
    tokio::join!(looper("A"), looper("B"));
}

// ! Кооперативность: yield_now явно отдаёт квант выполнения другим задачам.
