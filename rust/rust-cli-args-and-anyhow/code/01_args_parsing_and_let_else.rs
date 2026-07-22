// Шаг 1: Чтение CLI аргументов с std::env::args() и конструкция let-else

use std::env;

pub struct CliConfig {
    pub file_path: String,
    pub mode: String,
}

pub fn parse_cli_args(args: impl Iterator<Item = String>) -> Result<CliConfig, String> {
    let mut args_iter = args.skip(1); // Пропускаем имя бинарного файла

    // Лаконичный деструктурирующий парсинг без громоздких match (Rust 2021+):
    let Some(file_path) = args_iter.next() else {
        return Err("Использование: log_analyzer <FILE_PATH> [MODE]".to_string());
    };

    let mode = args_iter.next().unwrap_or_else(|| "summary".to_string());

    Ok(CliConfig { file_path, mode })
}

fn main() {
    let mock_args = vec![
        "log_analyzer".to_string(),
        "/var/log/syslog.log".to_string(),
        "verbose".to_string(),
    ];

    match parse_cli_args(mock_args.into_iter()) {
        Ok(config) => println!("Файл: {}, Режим: {}", config.file_path, config.mode),
        Err(err) => println!("Ошибка: {err}"),
    }
}

// ! **1. Парсинг CLI и синтаксис let-else**
// ! - `std::env::args()` создает итератор параметров. Первый элемент `.skip(1)` всегда содержит имя бинарного файла.
// ! - Конструкция `let Some(val) = iter.next() else { return ...; }` позволяет ранний возврат при отсутствии обязательных параметров.

// ---

// Шаг 2: Возврат Result<()> из функции main()

// ?hidden:start
pub struct CliConfig {
    pub file_path: String,
    pub mode: String,
}

pub fn parse_cli_args(args: impl Iterator<Item = String>) -> Result<CliConfig, String> {
    let mut args_iter = args.skip(1);
    let Some(file_path) = args_iter.next() else {
        return Err("Ошибка: не указан обязательный путь к файлу логов".to_string());
    };
    let mode = args_iter.next().unwrap_or_else(|| "default".to_string());
    Ok(CliConfig { file_path, mode })
}
// ?hidden:end

fn main() -> Result<(), String> {
    let mock_args = vec!["my_utility".to_string(), "/var/log/syslog.log".to_string()];
    
    // При возврате Result из main() оператор ? автоматически пробрасывает ошибки:
    let config = parse_cli_args(mock_args.into_iter())?;
    println!("Утилита успешно запущена для файла: {}", config.file_path);

    Ok(())
}

// ! **2. Возврат Result<()> из main()**
// ! - В Rust функция `main()` может возвращать `Result<(), E>`, где `E: Debug`.
// ! - При возникновении ошибки рантайм автоматически отпечатает отладочное сообщение и завершит процесс с кодом 1.
