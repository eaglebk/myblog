use std::collections::HashMap;

// Шаг 1: Макрос хеш-таблицы map!
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

    println!("Конфигурация: {:?}", config);
}

// ! **Макрос для HashMap с висячей запятой**
// ! - Синтаксис `$key:expr => $val:expr` задает свой разделитель `=>`.
// ! - `$(,)?` поддерживает опциональную запятую в конце списка элементов.

// ---

use std::time::Instant;

// Шаг 2: Замер времени выполнения блока кода
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
    let sum = time_it!("Подсчет суммы", {
        (1..=50_000).sum::<u64>()
    });

    println!("Результат: {sum}");
}

// ! **Измерение времени с помощью $code:block**
// ! - Спецификатор `$code:block` принимает произвольный блок кода `{ ... }`.
// ! - Макрос возвращает результат выполнения блока не нарушая логики.

// ---

// Шаг 3: Макрос повторных попыток вызова retry!
macro_rules! retry {
    ( $attempts:expr, $code:expr ) => {
        {
            let mut result = None;
            for attempt in 1..=$attempts {
                let res = $code;
                if res.is_ok() {
                    result = Some(res);
                    break;
                }
                println!("Попытка {attempt} завершилась ошибкой, повторяем...");
            }
            result.unwrap_or_else(|| Err("Превышено количество попыток"))
        }
    };
}

fn main() {
    let mut attempts_counter = 0;
    
    let res: Result<&str, &str> = retry!(3, {
        attempts_counter += 1;
        if attempts_counter == 2 {
            Ok("Успешное соединение!")
        } else {
            Err("Ошибка сети")
        }
    });

    println!("Итоговое состояние: {:?}", res);
}

// ! **Макрос автоматических повторов (retry!)**
// ! - Позволяет оборачивать ненадежные вычисления в цикл с заданным количеством попыток.
