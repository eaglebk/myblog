// ?hidden:start
use strum::{EnumIter, EnumString, Display, IntoEnumIterator};
use std::str::FromStr;
// ?hidden:end

#[derive(Debug, PartialEq, Eq, EnumIter, Display, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum Chip {
    Esp32,
    Esp32c2,
    Esp32c3,
    Esp32c5,
    Esp32c6,
    Esp32c61,
    Esp32h2,
    Esp32s2,
    Esp32s3,
}

fn main() {
    // 1. Итерирование по всем вариантам перечисления
    println!("=== Поддерживаемые микроконтроллеры ESP ===");
    for chip in Chip::iter() {
        println!("- {chip}");
    }

    // 2. Парсинг из строки обратно в enum
    let input = "esp32c3";
    match Chip::from_str(input) {
        Ok(chip) => println!("\nУспешно спарсили '{input}' в вариант: {:?}", chip),
        Err(_) => println!("\nНеизвестный чип!"),
    }
}

// ! **Итерирование по enum и парсинг строк в Rust с помощью strum**
// ! - **EnumIter**: Позволяет вызвать `Chip::iter()` и пройтись по всем вариантам без дублирования кода.
// ! - **Display**: Автоматически форматирует вариант в строку с учетом `serialize_all = "kebab-case"`.
// ! - **EnumString**: Позволяет спарсить строку `"esp32c3"` обратно в `Chip::Esp32c3`.
