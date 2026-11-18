// Шаг 1: Ранний возврат ошибок через оператор `?` в `.lines()`

use std::io::{self, BufRead, Cursor, ErrorKind, Result};

pub fn parse_config_values(reader: impl BufRead) -> Result<Vec<u32>> {
    let mut values = Vec::new();

    for line_result in reader.lines() {
        // Оператор `?` моментально прекращает выполнение при сбое чтения или битом UTF-8:
        let line = line_result?;
        let trimmed = line.trim();

        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            let num = trimmed
                .parse::<u32>()
                .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
            values.push(num);
        }
    }

    Ok(values)
}

fn main() {
    let raw_config = "100\n# comment\n200\n300\n";
    let cursor = Cursor::new(raw_config);

    match parse_config_values(cursor) {
        Ok(parsed) => println!("Разобраны числа: {:?}", parsed),
        Err(err) => eprintln!("Ошибка при чтении: {err}"),
    }
}

// ! **1. Безопасный обход `.lines()`**
// ! - Метод `.lines()` возвращает `Result<String>`, а не сырую строку `String`, так как чтение потока может сорваться в любой момент.
// ! - Оператор `?` обеспечивает безопасный проброс ошибки наверх без паники приложения.

// ---

// Шаг 2: Системный поток ошибок (`stderr`) и `ExitCode` вместо `process::exit`

// ?hidden:start
use std::io::{self, BufRead, Cursor, ErrorKind, Result};

pub fn parse_config_values(reader: impl BufRead) -> Result<Vec<u32>> {
    let mut values = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            let num = trimmed
                .parse::<u32>()
                .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
            values.push(num);
        }
    }
    Ok(values)
}
// ?hidden:end

use std::process::ExitCode;

fn run_cli_pipeline() -> Result<()> {
    let sample = Cursor::new("42\n84\n");
    let results = parse_config_values(sample)?;
    println!("Обработано элементов: {}", results.len());
    Ok(())
}

fn main() -> ExitCode {
    // Ошибки всегда пишутся в `stderr` через `eprintln!`,
    // а из `main()` возвращается `ExitCode`, чтобы отработали все деструкторы `Drop`:
    match run_cli_pipeline() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Критическая ошибка выполнения утилиты: {err}");
            ExitCode::FAILURE
        }
    }
}

// ! **2. Разделение `stdout`/`stderr` и возврат `ExitCode`**
// ! - Вывод ошибок в `eprintln!` защищает пайплайны Shell (`cat data | my_tool > output.txt`): ошибки не попадут в результирующий файл.
// ! - Возврат `ExitCode::FAILURE` из `main()` сигнализирует о сбое скриптам и CI/CD, но при этом корректно сбрасывает буферы и вызывает `Drop`.
