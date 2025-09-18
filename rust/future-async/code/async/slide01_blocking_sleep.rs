// ?hidden:start
fn query_sensor() -> u32 { 42 }
fn process(d: u32) -> u32 { d }

use std::{thread, time::Duration};
// ?hidden:end
fn calibrate() -> u32 {
    let d = query_sensor();
    thread::sleep(Duration::from_secs(5)); 
    process(d)
}

fn main() {
    println!("[МОСТИК] Запуск калибровки...");
    let data = calibrate();
    println!("[МОСТИК] Калибровка завершена: {data}");
}

// ! Проблема:
// ! - thread::sleep блокирует ОС-поток целиком.
// ! - Если так "усыпить" воркер рантайма, другие async-задачи не выполнятся.
