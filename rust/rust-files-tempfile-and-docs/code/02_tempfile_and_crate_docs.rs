// Шаг 1: Изолированное тестирование файловой системы через tempfile

use std::fs;
use std::path::Path;
use anyhow::Result;

pub fn write_header_if_missing(path: impl AsRef<Path>) -> Result<()> {
    let p = path.as_ref();
    if !p.exists() {
        fs::write(p, "# HEADER: Log Version 1.0\n")?;
    }
    Ok(())
}

fn main() {
    println!("Модуль файловой утилиты готов к тестированию.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn write_header_creates_file_in_isolated_tempdir() {
        // tempdir() создает временную папку в системном temp каталог:
        let dir = tempdir().expect("Не удалось создать временную директорию");
        let file_path = dir.path().join("audit.log");

        write_header_if_missing(&file_path).unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "# HEADER: Log Version 1.0\n");
    } // Переменная dir выходит из области видимости и срабатывает Drop — вся временная папка удаляется с диска!
}

// ! **1. Безопасное тестирование ФС с tempfile**
// ! - Библиотека `tempfile::tempdir()` создаёт изолированный каталог в `/tmp`.
// ! - При выходе из области видимости срабатывает `Drop`, и весь каталог вместе с созданными тестовыми файлами автоматически стирается с диска.

// ---

//! # Модуль безопасного журналирования
//!
//! Данный модуль предоставляет высокоуровневый интерфейс для работы с файлами логов.
//! Смотрите также функцию [`std::fs::File::open`] и типаж [`std::io::BufRead`].

use std::fs;
use std::path::Path;

/// Выполняет чтение первой строки из указанного пути файла.
///
/// # Ошибки
/// Возвращает [`anyhow::Error`], если файл не существует или недоступен для чтения.
pub fn read_first_line(path: impl AsRef<Path>) -> Result<String, String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let first = text.lines().next().unwrap_or("").to_string();
    Ok(first)
}

fn main() {
    println!("Интра-документационные ссылки откомпилированы успешно.");
}

// ! **2. Стандарты документации Rust**
// ! - `//!` описывает весь модуль или крейт целиком на самом верхнем уровне файла.
// ! - `///` документирует отдельные публичные функции, структуры или поля.
// ! - Синтаксис `[`std::fs::File::open`]` автоматически создает кликабельные внутридокументационные ссылки в rustdoc.
