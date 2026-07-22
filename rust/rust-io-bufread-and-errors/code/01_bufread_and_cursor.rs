// Шаг 1: Подход Magic Function и идеальная тестируемость с `impl BufRead` и `Cursor`

use std::io::{BufRead, Cursor, Result};

pub struct LogMetrics {
    pub total_lines: usize,
    pub error_lines: usize,
}

// Принимаем обобщенный `impl BufRead` вместо жестко привязанного `StdinLock` или `BufReader<File>`:
pub fn analyze_log_stream(input: impl BufRead) -> Result<LogMetrics> {
    let mut total_lines = 0;
    let mut error_lines = 0;

    for line in input.lines() {
        let text = line?; // Обработка возможных ошибок чтения каждого блока
        total_lines += 1;
        if text.contains("[ERROR]") || text.contains("[FATAL]") {
            error_lines += 1;
        }
    }

    Ok(LogMetrics {
        total_lines,
        error_lines,
    })
}

fn main() {
    // В тестах мы создаем фейковый поток ввода с помощью std::io::Cursor без обращения к файловой системе:
    let fake_log_data = "[INFO] System start\n[ERROR] Connection timeout\n[INFO] Retrying\n";
    let cursor = Cursor::new(fake_log_data);

    let metrics = analyze_log_stream(cursor).expect("Ошибка анализа лог-потока");
    println!(
        "Проанализировано строк: {}, ошибок найдено: {}",
        metrics.total_lines, metrics.error_lines
    );
}

// ! **1. Подход Magic Function и BufRead**
// ! - Сначала проектируем функцию от удобного вызова, подбирая минимально достаточный типаж `impl BufRead`.
// ! - Использование `std::io::Cursor` позволяет тестировать потоки чтения прямо в оперативной памяти без диск-И/О.

// ---

// Шаг 2: Эффективная блокировка stdin().lock() для высокопроизводительного ввода
// ?ignore

use std::io::{stdin, BufRead};

fn process_user_stream(input: impl BufRead) -> usize {
    input.lines().map_while(Result::ok).count()
}

fn main() {
    // Вызов stdin().lock() блокирует глобальный поток ввода на время работы,
    // избавляя от повторных захватов Mutex при чтении каждой строки:
    let locked_stdin = stdin().lock();

    // Передаем заблокированный stdin в ту же универсальную функцию:
    let count = process_user_stream(locked_stdin);
    println!("Получено строк из стандартного ввода: {count}");
}

// ! **2. Захват блокировки через `stdin().lock()`**
// ! - Стандартный `stdin()` защищен внутренним Mutex. Повторный вызов `.read_line()` на `stdin()` захватывает lock каждый раз.
// ! - Метод `stdin().lock()` возвращает `StdinLock`, который реализует `BufRead` и кардинально ускоряет чтение.
