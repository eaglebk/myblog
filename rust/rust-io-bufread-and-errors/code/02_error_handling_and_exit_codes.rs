// Шаг 1: Ранний возврат ошибок через оператор `?` и траектория `.lines()`

use std::io::{BufRead, Cursor, Result};

pub fn parse_config_values(reader: impl BufRead) -> Result<Vec<u32>> {
    let mut values = Vec::new();

    for line_result in reader.lines() {
        // Оператор `?` моментально прекращает выполнение при дисковой ошибке или сбое декодирования UTF-8:
        let line = line_result?;
        let trimmed = line.trim();

        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            if let Ok(num) = trimmed.parse::<u32>() {
                values.push(num);
            }
        }
    }

    Ok(values)
}

fn main() {
    let raw_config = "100\n# comment\n200\n300\n";
    let cursor = Cursor::new(raw_config);

    match parse_config_values(cursor) {
        Ok(parsed) => println!("Успешно распаршено чисел: {:?}", parsed),
        Err(err) => println!("Ошибка при чтении: {err}"),
    }
}

// ! **1. Безопасность итеррирования по .lines()**
// ! - Метод `.lines()` возвращает `Result<String>`, а не сырую строку `String`, так как чтение из I/O потока может сорваться в любой момент.
// ! - Оператор `?` обеспечивает безопасную проброску ошибки наверх без паники приложения.

// ---

// Шаг 2: Системный поток ошибок (stderr) и интеграция с Shell через exit codes

// ?hidden:start
use std::io::{BufRead, Cursor, Result};

pub fn parse_config_values(reader: impl BufRead) -> Result<Vec<u32>> {
    let mut values = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            if let Ok(num) = trimmed.parse::<u32>() {
                values.push(num);
            }
        }
    }
    Ok(values)
}
// ?hidden:end

use std::process;

fn run_cli_pipeline() -> Result<()> {
    let sample = Cursor::new("42\n84\n");
    let results = parse_config_values(sample)?;
    println!("Обработано элементов: {}", results.len());
    Ok(())
}

fn main() {
    // В консольных утилитах ошибки всегда пишутся в `stderr` через `eprintln!`,
    // а сбои завершаются с ненулевым кодом выхода для взаимодействия с Bash/CI:
    if let Err(err) = run_cli_pipeline() {
        eprintln!("Критическая ошибка выполнения утилиты: {err}");
        process::exit(1);
    }
}

// ! **2. Разделение stdout/stderr и коды завершения**
// ! - Вывод ошибок в `eprintln!` защищает пайплайны Shell (`cat data | my_tool > output.txt`): сообщения об ошибках не попадут в результирующий файл.
// ! - Вызов `process::exit(1)` позволяет скриптам сборки и CI/CD отслеживать успешность работы программы.
