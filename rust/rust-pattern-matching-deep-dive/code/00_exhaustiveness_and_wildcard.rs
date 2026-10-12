// ?compile_fail
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected { ip: String },
}

fn handle_connection(state: ConnectionState) {
    match state {
        ConnectionState::Disconnected => println!("Офлайн"),
        ConnectionState::Connected { ip } => println!("Подключено к {ip}"),
        // Ошибка компиляции: вариант `Connecting` не обработан!
    }
}

fn main() {
    let state = ConnectionState::Connecting;
    handle_connection(state);
}

// ! **1. Ошибка компиляции: неполный match**
// ! - Компилятор Rust строго требует исчерпывающей проверки всех вариантов (exhaustiveness).
// ! - Вариант `Connecting` пропущен, поэтому компилятор отклоняет сборку с ошибкой E0004.

// ---

enum ConnectionState {
    Disconnected,
    Connecting,
    Connected { ip: String },
}

fn handle_connection(state: ConnectionState) {
    match state {
        ConnectionState::Disconnected => println!("Офлайн"),
        ConnectionState::Connected { ip } => println!("Подключено к {ip}"),
        // Заглушка `_` перехватывает любые оставшиеся варианты:
        _ => println!("Прочие состояния"),
    }
}

fn main() {
    handle_connection(ConnectionState::Disconnected);
    handle_connection(ConnectionState::Connecting);
    handle_connection(ConnectionState::Connected {
        ip: String::from("192.168.1.1"),
    });
}

// ! **2. Исправление заглушкой `_` (wildcard)**
// ! - Шаблон `_` перехватывает любые необработанные варианты, удовлетворяя проверке exhaustiveness.
// ! - Хотя такой вариант компилируется, злоупотреблять им не рекомендуется: при добавлении новых вариантов в enum компилятор больше не предупредит об их пропуске.
