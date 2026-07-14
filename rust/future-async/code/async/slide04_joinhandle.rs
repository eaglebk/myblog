use tokio::task::{self, JoinHandle};

fn not_an_async_function() -> JoinHandle<()> {
    task::spawn(async {
        println!("[ЭКИПАЖ] Второе сообщение из async-задачи");
    })
}

#[tokio::main]
async fn main() {
    println!("[МОСТИК] Первое сообщение");
    let _ = not_an_async_function().await;
}

// ! Комментарий:
// ! Возвращаем JoinHandle из sync-функции и ждём её выше (await).
