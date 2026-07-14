use std::collections::HashMap;

fn main() {
    // Создаем изменяемый HashMap
    let mut smart_home = HashMap::new();

    // Добавляем устройства и их состояние
    smart_home.insert("гостиная_свет".to_string(), "включен".to_string());
    smart_home.insert("спальня_термостат".to_string(), "22°C".to_string());

    println!("Устройства: {:?}", smart_home);
}
// ! **Создание HashMap и добавление элементов**
// ! - Для использования `HashMap` импортируем его из `std::collections`.
// ! - Чтобы наполнить коллекцию данными, она должна быть объявлена изменяемой (`mut`).
// ! - Метод `insert(key, value)` добавляет новую пару ключ-значение в мапу.

// ---

use std::collections::HashMap;

fn main() {
    let mut smart_home = HashMap::new();
    smart_home.insert("гостиная_свет".to_string(), "включен".to_string());
    smart_home.insert("спальня_термостат".to_string(), "22°C".to_string());

    // Обновляем значение по существующему ключу
    smart_home.insert("гостиная_свет".to_string(), "выключен".to_string());

    println!("Обновленные устройства: {:?}", smart_home);
}
// ! **Обновление значения по ключу**
// ! - Если мы вызываем `insert` с ключом, который уже существует в `HashMap`, старое значение перезаписывается новым.
