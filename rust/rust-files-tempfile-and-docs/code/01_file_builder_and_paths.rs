// Шаг 1: Конфигурация дескрипторов файлов через File::options()

use std::fs::File;
use std::io::Write;
use anyhow::Result;

pub fn append_telemetry_log(file_path: &str, entry: &str) -> Result<()> {
    // Вызов File::options() является применением паттерна Builder для файловых флагов:
    let mut log_file = File::options()
        .create(true) // Создать файл, если он еще не существует
        .append(true) // Писать байты строго в конец файла без перезаписи
        .open(file_path)?;

    writeln!(log_file, "[LOG]: {entry}")?;
    Ok(())
}

fn main() {
    let result = append_telemetry_log("system_events.log", "Сервис авторизации успешно запущен");
    match result {
        Ok(()) => println!("Запись успешно дозаписана в файл лога"),
        Err(err) => println!("Ошибка доступа к файлу: {err}"),
    }
}

// ! **1. Паттерн Builder в File::options()**
// ! - Подробно паттерн Builder мы разбирали в статье [Паттерн Builder в Rust](../rust-builder/).
// ! - Стандартная библиотека использует этот паттерн в `File::options()` для наглядной настройки системных режимов открытия файлов (`read`, `write`, `append`, `create_new`).

// ---

// Шаг 2: Гибкие типы путей с impl AsRef<Path> и разница Path vs PathBuf

use std::fs;
use std::path::{Path, PathBuf};

// Использование `impl AsRef<Path>` позволяет функции принимать &str, String, Path, PathBuf без явного клонирования:
pub fn inspect_file_size(path: impl AsRef<Path>) -> std::io::Result<u64> {
    let path_ref: &Path = path.as_ref(); // Получаем иммутабельный срез пути (unsized Path)
    let metadata = fs::metadata(path_ref)?;
    Ok(metadata.len())
}

fn main() {
    // PathBuf — владеющий динамический путь в куче (аналог String).
    let mut owned_path = PathBuf::from("data");
    owned_path.push("config.json");

    // &str — обычная строковая ссылка.
    let str_path = "system_events.log";

    // Обе переменные разных типов передаются в одну и ту же функцию благодаря impl AsRef<Path>:
    if let Ok(size) = inspect_file_size(&owned_path) {
        println!("Размер {:?}: {size} байт", owned_path);
    }
    if let Ok(size) = inspect_file_size(str_path) {
        println!("Размер {:?}: {size} байт", str_path);
    }
}

// ! **2. Path vs PathBuf и трейт impl AsRef<Path>**
// ! - `Path` — невладеющий срез пути без фиксированного размера (unsized, аналогичен `&str`).
// ! - `PathBuf` — владеющий мутабельный путь в куче (аналог `String`).
// ! - `impl AsRef<Path>` предоставляет максимальную эргономику вызова для любого строкового типа.
