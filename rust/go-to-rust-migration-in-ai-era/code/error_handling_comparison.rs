#[derive(Debug)]
enum DatabaseError {
    NotFound,
    ConnectionFailed,
}

#[derive(Debug)]
enum UserError {
    Db(DatabaseError),
    InvalidInput(String),
}

impl From<DatabaseError> for UserError {
    fn from(err: DatabaseError) -> Self {
        UserError::Db(err)
    }
}

fn fetch_user_id(username: &str) -> Result<u64, DatabaseError> {
    if username == "unknown" {
        Err(DatabaseError::NotFound)
    } else {
        Ok(42)
    }
}

fn validate_and_fetch(username: &str) -> Result<u64, UserError> {
    if username.is_empty() {
        return Err(UserError::InvalidInput("Username is empty".to_string()));
    }
    // Оператор ? автоматически преобразует DatabaseError в UserError через From
    let id = fetch_user_id(username)?;
    Ok(id)
}

fn main() {
    match validate_and_fetch("alice") {
        Ok(id) => println!("Найден пользователь с ID: {}", id),
        Err(e) => println!("Ошибка: {:?}", e),
    }

    match validate_and_fetch("") {
        Ok(id) => println!("Найден пользователь с ID: {}", id),
        Err(e) => println!("Ошибка: {:?}", e),
    }
}
// ! **Элегантная обработка ошибок через Result и оператор ?**
// ! - В отличии от Go с проверкой `if err != nil` на каждом шаге, Rust использует оператор `?`.
// ! - Оператор `?` разворачивает `Ok(val)` или досрочно возвращает `Err(e)`, автоматически вызывая `.into()`.
// ! - Кастомный `enum` дает статическую гарантию обработки всех возможных сценариев сбоя.

// ---

fn parse_port(port_str: &str) -> Result<u16, String> {
    port_str
        .parse::<u16>()
        .map_err(|e| format!("Некорректный порт: {}", e))
}

fn connect_service(host: &str, port_str: &str) -> Result<String, String> {
    let port = parse_port(port_str)?;
    Ok(format!("Подключено к {}:{}", host, port))
}

fn main() {
    let ok = connect_service("localhost", "8080");
    let err = connect_service("localhost", "invalid");

    println!("Результат 1: {:?}", ok);
    println!("Результат 2: {:?}", err);
}
// ! **Управление контекстом ошибок без внешних библиотек**
// ! - Метод `.map_err()` позволяет мгновенно обогатить ошибку контекстом до ее возврата через `?`.
// ! - Такой подход не требует тяжёлых стэктрейсов и работает со 100% производительностью без аллокаций в хэше.
// ! - В продуктовой разработке для аналогичных целей используются `thiserror` (для библиотек) и `anyhow` (для бинарников).
