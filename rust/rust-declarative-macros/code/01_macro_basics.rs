// Шаг 1: Инициализатор вектора my_vec!
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
    println!("Вектор из макроса: {:?}", numbers);
}

// ! **Базовый макрос инициализации**
// ! - Спецификатор `$x:expr` сопоставляется с любым выражением (expression).
// ! - Конструкция `$( ... ),*` разворачивает аргументы через запятую.

// ---

// Шаг 2: Использование различных спецификаторов ($i:ident, $t:ty, $e:expr)
macro_rules! make_struct {
    ( $name:ident, $field:ident: $type:ty ) => {
        struct $name {
            $field: $type,
        }
    };
}

make_struct!(User, age: u32);

fn main() {
    let u = User { age: 25 };
    println!("Создана структура с возрастом: {}", u.age);
}

// ! **Спецификаторы типов токенов**
// ! - `$name:ident` захватывает имя структуры или переменной (идентификатор).
// ! - `$type:ty` захватывает тип данных Rust (`u32`, `String` и т.д.).
