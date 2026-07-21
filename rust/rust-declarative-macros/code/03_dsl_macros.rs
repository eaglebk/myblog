// Шаг 1: Макрос генерации HTML-разметки
macro_rules! html {
    ( h1 { $text:expr } ) => {
        format!("<h1>{}</h1>", $text)
    };
    ( p { $text:expr } ) => {
        format!("<p>{}</p>", $text)
    };
}

fn main() {
    let header = html!(h1 { "Заголовок статьи" });
    let paragraph = html!(p { "Текст первого параграфа." });

    println!("{header}");
    println!("{paragraph}");
}

// ! **Предметно-ориентированный язык (DSL)**
// ! - Макросы могут сопоставлять фиксированные ключевые слова (`h1`, `p`) с генерацией пользовательского кода.

// ---

// Шаг 2: Макрос генерации мини-JSON строк json_lite!
macro_rules! json_lite {
    ( $( $key:literal : $val:expr ),* $(,)? ) => {
        {
            let mut pairs = Vec::new();
            $(
                pairs.push(format!("\"{}\": \"{}\"", $key, $val));
            )*
            format!("{{ {} }}", pairs.join(", "))
        }
    };
}

fn main() {
    let user_json = json_lite! {
        "name": "Алексей",
        "role": "Разработчик",
        "city": "Москва",
    };

    println!("Сгенерированный JSON: {user_json}");
}

// ! **Генератор структуры JSON**
// ! - Спецификатор `$key:literal` сопоставляется со строковыми или числовыми литералами.
// ! - Макрос объединяет элементы через джойнер `, `.
