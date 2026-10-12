# Крейт дня: Strum - автоматический перебор и конвертация enum в Rust на примере esp-generate

> Разбираем популярный крейт strum: как итерироваться по вариантам enum, конвертировать enum в строки и обратно на реальном примере из инструмента esp-generate.

- **URL:** https://eagle2000.space/rust/crate-of-the-day-strum/
- **Date:** 2026-08-16

- **Tags:** rust, crate-of-the-day, enums, macros, esp32

---
{{< lead >}}
«Крейт дня» - краткий разбор популярных и практичных библиотек экосистемы Rust, которые избавляют от написания рутинного кода и делают архитектуру чище.
{{< /lead >}}

Сегодня на рассмотрении - **`strum`** (набор процедурных макросов для работы с `enum` в Rust).

---

## Проблема: enum в Rust не имеет итератора

Сам `enum` широко используется в языке, он отлично подходит для описания состояний или вариантов. Однако из коробки в языке **нет встроенного способа пройтись циклом по всему Enum**:

```rust
// Хотим пройтись по всем входящим в enum вариантам в цикле:
for color in Color::all_variants() { ... } // ❌ Ошибка! Такого метода нет
```

Если вы разрабатываете CLI-утилиту, генератор аналогичный `esp generator` или веб-сервис, вам часто требуется:
1. Вывести список всех поддерживаемых опций в `--help` или консольный меню-селектор.
2. Проверить входящую строку от пользователя или из файла конфигурации и распарсить её в `enum`.
3. Преобразовать имя enum в удобный строковый вид (Например для enum Color: `"black"` или `"white"`).

Без сторонних библиотек разработчикам приходится писать все вручную чтобы поддерживать массив `const ALL_COLORS: &[Color] = &[...]`. 

> **Главный риск такого подхода:** как только в проект добавляется новый вариант для enum, например `GREY` кто-то обязательно забудет дописать его в массив.

---

## Решение от сообщества: реальный пример из проекта `esp-generate`

Официальный генератор проектов для микроконтроллеров Espressif — **`esp-generate`** (от команды `esp-rs`) решает эту задачу с помощью крейта `strum`.

Вот как объявлен их `enum Chip` в исходном коде библиотеки:

```rust
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    clap::ValueEnum,
    strum::EnumIter,
    strum::Display,
    strum::EnumString,
)]
#[serde(rename_all = "kebab-case")]
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
```

Всего три макроса из `strum` перекрывают все потребности работы со строками и итерацией!

---

## Основные фишки Strum

### 1. Итерирование по вариантам enum (`strum::EnumIter`)

Макрос `EnumIter` в сочетании с типажом `strum::IntoEnumIterator` добавляет к `enum` метод `.iter()`:

```rust
use strum::IntoEnumIterator;

for chip in Chip::iter() {
    println!("Поддерживаемый микроконтроллер: {chip}");
}
```

Теперь при добавлении любого нового варианта (например, `Esp32p4`) он **автоматически** начнет участвовать во всех циклах программы без правки кода!

---

### 2. Преобразование в строку (`strum::Display`)

Макрос `Display` избавляет от необходимости вручную писать `impl std::fmt::Display for Chip`.

Атрибут `#[strum(serialize_all = "kebab-case")]` настраивает правильное имя для вывода:
* `Chip::Esp32c3` превращается в `"esp32c3"`.
* `Chip::Esp32s3` превращается в `"esp32s3"`.

```rust
println!("{}", Chip::Esp32c3); // Выведет: esp32c3
```

---

### 3. Парсинг строки в Enum (`strum::EnumString`)

Макрос `EnumString` реализует типаж `std::str::FromStr`. Это позволяет легко распарсить пользовательский ввод:

```rust
use std::str::FromStr;

let chip = Chip::from_str("esp32c3")?; // Ok(Chip::Esp32c3)
// Или стандартным методом parse():
let chip: Chip = "esp32c3".parse()?;
```

---

## Другие полезные макросы в strum

Крейт `strum` не ограничивается этой тройкой:

* **`EnumCount`**: добавляет константу (Например `Chip::COUNT`) показывающую общее количество вариантов `enum` во время компиляции.
* **`VariantNames`**: добавляет слайс  с именами всех вариантов (Например `Chip::VARIANTS` - `["esp32", "esp32c2"]`).
* **`EnumProperty` / `EnumMessage`**: позволяет прикреплять произвольные метаданные или понятные описания прямо к вариантам `enum` через атрибуты `#[strum(message = "...")]`.

---

## Полный пример кода

```rust
use strum::{EnumIter, EnumString, Display, IntoEnumIterator};
use std::str::FromStr;

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
    // 1. Итерируемся по всему enum
    println!("=== ESP Chips ===");
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
```

### Вывод программы:
```text
=== Поддерживаемые микроконтроллеры ESP ===
- esp32
- esp32c2
- esp32c3
- esp32c5
- esp32c6
- esp32c61
- esp32h2
- esp32s2
- esp32s3

Успешно преобразовали 'esp32c3' в: Esp32c3
```

---

## 🎯 Проверь свои знания!

Пройдите короткий тест, чтобы закрепить материал о работе с `strum`:


### 📝 Крейт Strum в Rust


**1. Какой макрос strum добавляет метод .iter() для enum?**

- [ ] strum::Display
- [ ] strum::EnumIter
- [ ] strum::EnumString
- [ ] strum::EnumCount

**2. Как атрибут #[strum(serialize_all = "kebab-case")] преобразует вариант Esp32c3 в строку?**

- [ ] Преобразует в ESP32C3
- [ ] Преобразует в esp32c3
- [ ] Преобразует в esp_32_c3
- [ ] Оставляет без изменений

**3. Какую роль выполняет derive-макрос strum::EnumString?**

- [ ] Выводит enum в консоль через std::fmt::Display
- [ ] Считает количество элементов в enum
- [ ] Реализует трейт std::str::FromStr для парсинга строк в варианты enum
- [ ] Создает строковый массив всех имен вариантов enum

**4. В чём главное преимущество strum::EnumIter перед ручным массивом variants: &[Chip]?**

- [ ] Работает только во время выполнения
- [ ] При добавлении нового варианта в enum он автоматически попадает во все циклы итерации
- [ ] Ускоряет компиляцию программы в 2 раза
- [ ] Позволяет хранить варианты в памяти в виде связного списка

---

## Как добавить в проект

Достаточно прописать в `Cargo.toml`:

```toml
[dependencies]
strum = { version = "0.28", features = ["derive"] }
```

🔗 **Репозиторий на GitHub:** [Petrochev/strum](https://github.com/Petrochev/strum)  
📦 **Crates.io:** [crates.io/crates/strum](https://crates.io/crates/strum)  
🦀 **Пример из esp-generate:** [esp-rs/esp-generate](https://github.com/esp-rs/esp-generate)

