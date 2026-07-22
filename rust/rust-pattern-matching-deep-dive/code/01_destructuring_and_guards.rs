// Шаг 1: Глубокая деструктуризация структур и связывающие шаблоны (@)
struct Address {
    street: String,
    city: String,
}

struct Person {
    name: String,
    address: Address,
}

fn main() {
    let person = Person {
        name: String::from("Алексей"),
        address: Address {
            street: String::from("Ленина"),
            city: String::from("Москва"),
        },
    };

    // 1. Извлечение полей из вложенной структуры за одно действие:
    let Person { name, address: Address { street, city } } = person;
    println!("Имя: {name}, Город: {city}, Улица: {street}");

    // 2. Использование @-связывания для сохранения значения при проверке диапазона:
    let val = 15;
    match val {
        num @ 1..=10 => println!("Число {num} в диапазоне от 1 до 10"),
        num @ 11..=20 => println!("Число {num} в диапазоне от 11 до 20"),
        _ => println!("Другое число"),
    }
}

// ! **1. Деструктуризация и @-связывание**
// ! - Шаблоны позволяют разбирать вложенные структуры и перечисления за один шаг без временных переменных.
// ! - Оператор `@` сохраняет привязанное значение в переменную одновременно с проверкой диапазона или шаблона.

// ---

enum Signal {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

// Шаг 2: Охранные выражения (Match Guards) в перечислениях
fn main() {
    let signal = Signal::Move { x: 20, y: 20 };

    match signal {
        Signal::Quit => println!("Команда на выход"),
        // Match Guard с условием равенства координат:
        Signal::Move { x, y } if x == y => {
            println!("Диагональное перемещение в точку ({x}, {y})");
        }
        Signal::Move { x, y } => {
            println!("Обычное смещение в точку ({x}, {y})");
        }
        Signal::Write(text) => println!("Текст сигнала: {text}"),
    }
}

// ! **2. Охранные выражения (Match Guards)**
// ! - Match Guard (`if CONDITION`) добавляет дополнительное динамическое условие к ветке `match`.
// ! - Компилятор учитывает порядок веток: ветку с guard следует ставить перед общей веткой.
