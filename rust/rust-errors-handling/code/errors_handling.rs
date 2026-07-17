fn main() {
    let raw_input: Option<&str> = Some("42");
    
    // Преобразуем строку в число и умножаем на 2
    let processed = raw_input
        .and_then(|s| s.parse::<i32>().ok())
        .map(|num| num * 2);
        
    println!("Обработанное значение: {:?}", processed);
}

// ! **Комбинаторы Option и Result**
// ! - Вместо матчинга можно лаконично преобразовывать данные с помощью `map` и `and_then`.
// ! - Метод `map` применяет функцию к значению внутри `Some` или `Ok`.
// ! - Метод `and_then` принимает замыкание, возвращающее новый `Option`/`Result` — удобно для цепочки зависимых операций.

// ---

// ?hidden:start
use std::num::ParseIntError;

fn parse_and_double(val_str: &str) -> Result<i32, ParseIntError> {
    // Если parse вернет Err, выполнение прервется и вернется Err
    let parsed = val_str.parse::<i32>()?;
    Ok(parsed * 2)
}
// ?hidden:end

fn main() {
    match parse_and_double("21") {
        Ok(res) => println!("Успех: {}", res),
        Err(e) => println!("Ошибка разбора: {}", e),
    }
    
    if parse_and_double("invalid").is_err() {
        println!("Функция корректно вернула ошибку при невалидном вводе.");
    }
}

// ! **Оператор проброса ошибок `?`**
// ! - Оператор `?` разворачивает `Ok(T)` или немедленно возвращает `Err(E)` из текущей функции.
// ! - Он также автоматически приводит типы ошибок с помощью типажа `From` (`From::from`).
// ! - Это позволяет прозрачно трансформировать локальные ошибки в общую ошибку функции.

// ---

// ?hidden:start
use thiserror::Error;

#[derive(Error, Debug)]
enum DataError {
    #[error("Ошибка ввода-вывода при обращении к файлу: {0}")]
    Disconnect(#[from] std::io::Error),
    
    #[error("Некорректный формат данных в строке: {0}")]
    InvalidFormat(String),
}

fn read_data() -> Result<String, DataError> {
    // Симулируем чтение с ошибкой формата
    Err(DataError::InvalidFormat("Не хватает заголовка".to_string()))
}
// ?hidden:end

fn main() {
    match read_data() {
        Ok(data) => println!("Данные: {}", data),
        Err(e) => println!("Произошла ошибка: {}", e),
    }
}

// ! **Кастомные ошибки с thiserror**
// ! - Крейт `thiserror` генерирует реализации трейта `Error` и форматирования `Display`.
// ! - Мы аннотируем варианты макросом `#[error(...)]` для пользовательского вывода.
// ! - Атрибут `#[from]` автоматически реализует типаж `From` для конвертации ошибок.
