// Шаг 1: Макрос векторов my_vec!
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let numbers = my_vec![10, 20, 30, 40];
    println!("Созданный вектор: {:?}", numbers);
}

// ! **1. Базовые сопоставления и повторения**
// ! - `$x:expr` подставляет выразительный фрагмент кода (expression).
// ! - `$( ... ),*` указывает на повторение фрагментов через запятую 0 или более раз.
// ! - Двойные фигурные скобки `{ { ... } }` создают изолированную область видимости для тела макроса.

// ---

use std::collections::HashMap;

// Шаг 2: Макрос удобного создания хеш-таблицы map!
macro_rules! map {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {
        {
            let mut temp_map = HashMap::new();
            $(
                temp_map.insert($key, $val);
            )*
            temp_map
        }
    };
}

fn main() {
    let config = map! {
        "host" => "localhost",
        "port" => "8080",
        "protocol" => "https",
    };

    println!("Конфигурация сервера: {:?}", config);
}

// ! **2. Макрос инициализации Хеш-таблицы**
// ! - Синтаксис `$key:expr => $val:expr` позволяет определять пользовательские разделители (стрелка `=>`).
// ! - `$(,)?` разрешает ставить висячую запятую (trailing comma) в конце списка элементов.

// ---

use std::time::Instant;

// Шаг 3: Практический макрос замеров времени time_it!
macro_rules! time_it {
    ( $name:expr, $code:block ) => {
        {
            let start = Instant::now();
            let result = $code;
            let elapsed = start.elapsed();
            println!("[ТАЙМЕР] Блок '{}' выполнен за {:?}", $name, elapsed);
            result
        }
    };
}

fn main() {
    let total_sum = time_it!("Суммирование 100k элементов", {
        let mut sum = 0u64;
        for i in 1..=100_000 {
            sum += i;
        }
        sum
    });

    println!("Итоговая сумма: {total_sum}");
}

// ! **3. Измерение времени выполнения кода (time_it!)**
// ! - Фрагмент `$code:block` принимает целые блоки кода в фигурных скобках `{ ... }`.
// ! - Макрос прозрачно оборачивает код замером `Instant::now()` и возвращает итоговый результат выполнения блока.

// ---

// Шаг 4: Декларативный макрос строителя предметно-ориентированного языка (DSL)
macro_rules! html {
    ( h1 { $text:expr } ) => {
        format!("<h1>{}</h1>", $text)
    };
    ( p { $text:expr } ) => {
        format!("<p>{}</p>", $text)
    };
    ( div { $( $child:tt )* } ) => {
        format!("<div>{}</div>", html!( $( $child )* ))
    };
}

fn main() {
    let title = html!(h1 { "Привет, Rust!" });
    let paragraph = html!(p { "Макросы позволяют конструировать DSL прямо в коде." });

    println!("{title}");
    println!("{paragraph}");
}

// ! **4. Построение собственного DSL (Domain-Specific Language)**
// ! - Макросы могут содержать несколько сопоставлений шаблонов (pattern matching), подобно `match`.
// ! - Фрагмент `$child:tt` улавливает произвольный токен (Tree Token).
// ! - Макросы могут вызываться рекурсивно для построения сложных деревообразных структур.
