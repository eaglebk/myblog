use std::collections::HashMap;

fn main() {
    let mut smart_home = HashMap::new();
    smart_home.insert("гостиная_свет", "включен");

    // Метод get() возвращает Option<&V>
    let light_status = smart_home.get("гостиная_свет");
    let lock_status = smart_home.get("дверь_замок");

    println!("Свет в гостиной: {:?}", light_status);
    println!("Замок двери: {:?}", lock_status);
}
// ! **Поиск значений по ключу через .get()**
// ! - Метод `.get()` принимает ссылку на ключ и возвращает `Option<&V>`.
// ! - Если ключ найден, возвращается `Some(&value)`. Если ключа нет, возвращается `None`.
// ! - Это позволяет безопасно проверять наличие данных без риска паники во время работы.

// ---

use std::collections::HashMap;

fn main() {
    let mut smart_home = HashMap::new();
    smart_home.insert("гостиная_свет", "включен");

    // Получаем значение с дефолтом, если ключ отсутствует
    let default_status = "выключен";
    let light = smart_home.get("гостиная_свет").unwrap_or(&default_status);
    let lock = smart_home.get("дверь_замок").unwrap_or(&default_status);

    println!("Свет: {}", light);
    println!("Замок: {}", lock);
}
// ! **Безопасное извлечение с unwrap_or**
// ! - Метод `.unwrap_or()` позволяет задать запасное значение, которое будет использовано, если `.get()` вернул `None`.

// ---

use std::collections::HashMap;

fn main() {
    let mut smart_home = HashMap::new();
    smart_home.insert("гостиная_свет".to_string(), "включен".to_string());

    // or_insert добавляет значение, только если ключа нет
    smart_home.entry("кухня_чайник".to_string()).or_insert("выключен".to_string());
    smart_home.entry("гостиная_свет".to_string()).or_insert("включен".to_string());

    // and_modify изменяет значение, только если ключ есть
    smart_home.entry("гостиная_свет".to_string())
        .and_modify(|status| *status = "выключен".to_string());

    println!("Итоговое состояние: {:?}", smart_home);
}
// ! **Entry API: or_insert и and_modify**
// ! - Метод `.entry()` возвращает перечисление Entry, представляющее запись (присутствующую или отсутствующую).
// ! - Метод `.or_insert()` вставляет значение, только если ключа нет, и возвращает мутабельную ссылку на значение.
// ! - Метод `.and_modify()` позволяет выполнить замыкание для изменения существующего значения по ключу.
