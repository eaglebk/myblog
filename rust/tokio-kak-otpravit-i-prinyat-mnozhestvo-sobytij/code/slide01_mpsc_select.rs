// ?hidden:start
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration, interval};

#[derive(Debug)]
enum Command {
    AdjustShields(u32),
    Evacuate,
}

#[derive(Debug)]
enum Telemetry {
    Temperature(f32),
    ReactorStatus(bool),
}
// ?hidden:end

#[tokio::main]
async fn main() {
    let (cmd_tx, mut cmd_rx) = mpsc::channel(10);
    let (telemetry_tx, mut telemetry_rx) = mpsc::channel(10);

    // Имитируем отправку команд в фоне
    tokio::spawn(async move {
        sleep(Duration::from_millis(150)).await;
        let _ = cmd_tx.send(Command::AdjustShields(80)).await;
        sleep(Duration::from_millis(250)).await;
        let _ = cmd_tx.send(Command::Evacuate).await;
    });

    // Имитируем отправку телеметрии в фоне
    tokio::spawn(async move {
        let _ = telemetry_tx.send(Telemetry::Temperature(36.6)).await;
        sleep(Duration::from_millis(200)).await;
        let _ = telemetry_tx.send(Telemetry::ReactorStatus(true)).await;
    });

    let mut heartbeat = interval(Duration::from_millis(100));
    // Пропустим первый мгновенный тик, чтобы не засорять вывод
    heartbeat.tick().await;

    loop {
        tokio::select! {
            // Читаем команды
            Some(cmd) = cmd_rx.recv() => {
                println!("[МОСТИК] Получена команда: {:?}", cmd);
                if let Command::Evacuate = cmd {
                    println!("[МОСТИК] Внимание! Начинаем эвакуацию!");
                    break;
                }
            }
            // Читаем телеметрию
            Some(tel) = telemetry_rx.recv() => {
                println!("[МОСТИК] Телеметрия: {:?}", tel);
            }
            // Периодический сигнал
            _ = heartbeat.tick() => {
                println!("[МОСТИК] Системы жизнеобеспечения в норме...");
            }
        }
    }
}

// ! **Мультиплексирование через tokio::select!**
// ! - Слушаем несколько источников событий одновременно.
// ! - Ветки выбираются по мере готовности данных.
// ! - Работает для каналов, таймеров и любых Future.
