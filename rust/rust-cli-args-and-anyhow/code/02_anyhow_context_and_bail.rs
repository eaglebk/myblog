// Шаг 1: Обогащение контекста ошибок в anyhow: .context() vs .with_context()

use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_first_config_line(path: &str) -> Result<String> {
    let file = File::open(path)
        // Замыкание в with_context вычисляется ТОЛЬКО ПРИ ОШИБКЕ, не тратя ресурсы на форматирование в happy path:
        .with_context(|| format!("Не удалось открыть конфигурационный файл по пути: '{path}'"))?;

    let mut reader = BufReader::new(file);
    let mut first_line = String::new();

    reader
        .read_line(&mut first_line)
        .context("Ошибка при чтении первой строки из файла")?;

    Ok(first_line.trim().to_string())
}

fn main() {
    let result = read_first_config_line("/non_existent_directory/config.toml");
    if let Err(err) = result {
        println!("Перехваченная цепочка контекста ошибок:");
        println!("{err:?}");
    }
}

// ! **1. Обогащение ошибок с anyhow**
// ! - `.context("static msg")` добавляет статический контекст к цепочке вызовов ошибки.
// ! - `.with_context(|| format!(...))` использует замыкание (lazy evaluation), предотвращая лишние аллокации строк при успешном выполнении.

// ---

// Шаг 2: Ранний возврат и валидация через anyhow::bail! и anyhow::ensure!

use anyhow::{bail, ensure, Result};

pub fn process_user_tokens(token: &str, min_length: usize) -> Result<usize> {
    // ensure! проверяет предикат и делает моментальный bail! с сообщением, если условие ложно:
    ensure!(
        !token.is_empty(),
        "Токен аутентификации не может быть пустым"
    );

    if token.len() < min_length {
        // bail! — макрос быстрый возврат ошибки anyhow::Error:
        bail!(
            "Длина токена слишком мала: получено {}, требуется минимум {}",
            token.len(),
            min_length
        );
    }

    Ok(token.len())
}

fn main() {
    match process_user_tokens("abc", 8) {
        Ok(len) => println!("Валидный токен длиной {len}"),
        Err(err) => println!("Ошибка валидации: {err}"),
    }
}

// ! **2. Макросы bail! и ensure!**
// ! - Макрос `bail!("msg")` моментально прерывает функцию и возвращает `Err(anyhow::anyhow!("msg"))`.
// ! - Макрос `ensure!(condition, "msg")` является краткой записью для `if !condition { bail!("msg"); }`.
