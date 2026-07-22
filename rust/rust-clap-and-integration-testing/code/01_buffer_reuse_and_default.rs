// Шаг 1: Эффективная обработка с переиспользованием единственного буфера памяти

use std::io::{BufRead, Cursor, Result};

#[derive(Default, Debug, PartialEq)]
pub struct LineStats {
    pub total_lines: usize,
    pub total_words: usize,
    pub total_bytes: usize,
}

pub fn count_stats(mut reader: impl BufRead) -> Result<LineStats> {
    let mut stats = LineStats::default();
    let mut buffer = String::new(); // Единственная аллокация памяти в куче!

    // Пока чтение возвращает больше 0 байт, переиспользуем выделенный буфер:
    while reader.read_line(&mut buffer)? > 0 {
        stats.total_lines += 1;
        stats.total_words += buffer.split_whitespace().count();
        stats.total_bytes += buffer.len();

        buffer.clear(); // Очищаем строку без освобождения выделенного объема памяти
    }

    Ok(stats)
}

fn main() {
    let data = "Rust — язык системного программирования\nВторая строка файла\nТретья строка\n";
    let cursor = Cursor::new(data);

    let stats = count_stats(cursor).expect("Ошибка сбора статистики");
    println!("Результат подсчета: {:?}", stats);
}

// ! **1. Переиспользование буфера (Zero-Reallocation)**
// ! - Использование `.lines()` создает новый объект `String` на каждом шаге итератора, загружая аллокатор памяти.
// ! - Метод `read_line(&mut buffer)` в сочетании с `buffer.clear()` сводит миллионы выделений памяти к ОДНОМУ.

// ---

// Шаг 2: Автоматическая инициализация через #[derive(Default)]

#[derive(Default, Debug)]
pub struct AppConfig {
    pub max_retries: u32,
    pub timeout_sec: u64,
    pub verbose_logging: bool,
    pub output_directory: String,
}

fn main() {
    // Трейт Default автоматически заполняет все поля нулевыми/пустыми значениями по умолчанию:
    let default_config = AppConfig::default();
    println!("Конфигурация по умолчанию: {:?}", default_config);

    // Удобный синтаксис кастомизации отдельных полей (Struct Update Syntax):
    let custom_config = AppConfig {
        max_retries: 5,
        verbose_logging: true,
        ..AppConfig::default()
    };
    println!("Пользовательская конфигурация: {:?}", custom_config);
}

// ! **2. Трейт Default**
// ! - Атрибут `#[derive(Default)]` избавляет от написания рутинных конструкторов со стандартными нулями.
// ! - Позволяет легко совмещать значения по умолчанию с кастомными настройками через `..Default::default()`.
