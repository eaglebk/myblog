use tokio::time::{sleep, Duration};

async fn good_sleep() {
    // ! Хорошо: уступаем планировщику
    sleep(Duration::from_millis(200)).await;
    println!("Проснулся (планировщик выполнял другие задачи)");
}

// ---
#[tokio::main]
async fn main() {
    let t1 = good_sleep();
    let t2 = async {
        for i in 0..5 {
            println!("Параллельная задача {i}");
            tokio::task::yield_now().await;
        }
    };
    tokio::join!(t1, t2);
}
