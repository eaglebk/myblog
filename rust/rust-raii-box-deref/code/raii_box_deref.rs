struct SensorData {
    name: String,
}

impl Drop for SensorData {
    fn drop(&mut self) {
        println!("Очистка ресурса датчика: {}", self.name);
    }
}

fn main() {
    {
        let sensor = SensorData {
            name: "Температура Кухни".to_string(),
        };
        println!("Работаем с датчиком: {}", sensor.name);
    } // Здесь область видимости заканчивается — автоматически вызывается Drop!
    println!("Секция завершена.");
}

// ! **Идиома RAII и трейт Drop**
// ! - Концепция RAII гарантирует: ресурс выделяется при создании и автоматически деаллоцируется при выходе из области видимости.
// ! - Трейт `Drop` позволяет переопределить логику очистки (закрытие файлов, освобождение памяти, логирование).
// ! - Вызов метода `drop` происходит автоматически в порядке, обратном объявлению переменных.

// ---

struct Config {
    port: u16,
}

fn main() {
    // 1. Создаем объект в куче через Box
    let mut heap_config = Box::new(Config { port: 8080 });
    heap_config.port = 9090; // Мутабельный доступ к данным в куче
    println!("Порт конфигурации в куче: {}", heap_config.port);

    // 2. Намеренно «утекаем» память для получения 'static ссылки
    let static_config: &'static mut Config = Box::leak(heap_config);
    static_config.port = 3000;
    println!("Статический порт после Box::leak: {}", static_config.port);
}

// ! **Умный указатель Box<T> и Box::leak**
// ! - `Box<T>` перемещает объект из стека в кучу (heap), сохраняя владение над ним.
// ! - `Box::leak` отменяет автоматический вызов `Drop` и возвращает мутабельную ссылку со статическим временем жизни `'static`.
// ! - Это полезно для создания глобальных конфигураций или долгоживущих синглтонов в рантайме.

// ---

use std::ops::{Deref, DerefMut};

struct SmartWrapper<T> {
    value: T,
}

impl<T> SmartWrapper<T> {
    fn new(value: T) -> Self {
        SmartWrapper { value }
    }
}

impl<T> Deref for SmartWrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for SmartWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    let mut wrapped_num = SmartWrapper::new(42);
    
    // Перегрузка оператора разыменования *
    assert_eq!(*wrapped_num, 42);
    
    *wrapped_num += 10;
    println!("Обновленное значение: {}", *wrapped_num);
}

// ! **Перегрузка разыменования: Deref и DerefMut**
// ! - Реализация трейта `Deref` позволяет использовать оператор разыменования `*` для кастомных умных указателей.
// ! - `DerefMut` расширяет разыменование для изменяемых ссылок (`*x = new_val`).
// ! - Ассоциированный тип `type Target = T` указывает, к какому типу приводит разыменование.

// ---

// Функция ожидает обычный строковый срез &str
fn print_message(msg: &str) {
    println!("Сообщение: {msg}");
}

fn main() {
    let boxed_string: Box<String> = Box::new(String::from("Привет от Deref!"));
    
    // Цепочка приведений Deref Coercion: &Box<String> -> &String -> &str
    print_message(&boxed_string);
    
    // Проверка методов целевого типа прямо через умный указатель:
    println!("Длина строки: {}", boxed_string.len());
}

// ! **Неявное приведение ссылок (Deref Coercion)**
// ! - Компилятор автоматически преобразует ссылки на умный указатель в ссылку на внутренний тип: `&Box<String>` -> `&String` -> `&str`.
// ! - Это работает для цепочек приведений типов, реализующих `Deref`.
// ! - Позволяет вызывать методы внутреннего типа напрямую через указатель (например, `boxed_string.len()`).
