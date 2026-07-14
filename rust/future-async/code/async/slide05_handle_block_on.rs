use tokio::runtime::Handle;

fn not_an_async_function(handle: Handle) {
    handle.block_on(async {
        println!("[ЭКИПАЖ] Второе сообщение через block_on");
    })
}

#[tokio::main]
async fn main() {
    println!("[МОСТИК] Первое сообщение");
    let handle = Handle::current();
    std::thread::spawn(move || {
        not_an_async_function(handle);
    })
    .join()
    .unwrap();
}

// ! Мост между потоком операционной системы и Tokio — Handle::block_on.
