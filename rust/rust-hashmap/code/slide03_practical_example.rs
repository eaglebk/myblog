use std::collections::HashMap;

fn main() {
    // Создаем HashMap для хранения статусов устройств
    let mut home_device_status: HashMap<String, String> = HashMap::new();

    // Добавляем статусы для некоторых устройств
    home_device_status.insert("гостиная_свет".to_string(), "включен".to_string());
    home_device_status.insert("спальня_термостат".to_string(), "22°C".to_string());

    // Список интересующих нас приборов
    let devices = vec!["гостиная_свет", "спальня_термостат", "кухня_кондиционер"];
    let default_device_status = "статус неизвестен".to_string();

    // Перебираем все устройства и выводим их статус
    for device in devices {
        let status = home_device_status.get(device).unwrap_or(&default_device_status);
        println!("{}: {}", device, status);
    }
}
// ! **Практический пример: Мониторинг умного дома**
// ! - Мы обходим вектор интересующих нас приборов и ищем состояние каждого в `HashMap`.
// ! - Для прибора `кухня_кондиционер`, которого нет в мапе, метод `.unwrap_or()` подставляет запасное значение `статус неизвестен`.
