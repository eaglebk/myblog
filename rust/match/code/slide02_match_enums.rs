// ?hidden:start
#[allow(dead_code)]
// ?hidden:end
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);

    match color {
        Color::Red   => println!("Красный"),
        Color::Blue  => println!("Синий"),
        Color::Green => println!("Зелёный"),
        Color::RGB(r, g, b) => println!("RGB: {}, {}, {}", r, g, b),
    }
}
// ! **Match с перечислениями (Enums)**
// ! - `match` — основной инструмент для работы с перечислениями (enum) в Rust.
// ! - Каждая ветка соответствует одному из вариантов enum. Компилятор проверяет, что все варианты обработаны.
// ! - Вариант `Color::RGB(r, g, b)` извлекает (деструктурирует) числа из данных, хранящихся внутри этого варианта enum.

// ---

// ?hidden:start
#[allow(dead_code)]
// ?hidden:end
enum Exercise {
    Run { distance: f64 },
    Swim { duration: u32 },
    Yoga,
}

fn process_exercise(exercise: Exercise) {
    match exercise {
        Exercise::Run { distance } => {
            println!("Ты пробежал {} км. Отлично!", distance);
        }
        Exercise::Swim { duration } => {
            println!("Ты плавал {} минут. Продолжай в том же духе!", duration);
        }
        Exercise::Yoga => {
            println!("Отличная работа — норма йоги на сегодня выполнена!");
        }
    }
}

fn main() {
    process_exercise(Exercise::Run { distance: 10.5 });
    process_exercise(Exercise::Swim { duration: 45 });
    process_exercise(Exercise::Yoga);
}
// ! **Match с богатыми данными внутри Enum**
// ! - Варианты enum могут хранить именованные поля (как в структуре).
// ! - В шаблоне `Exercise::Run { distance }` мы одновременно проверяем вариант и извлекаем поле `distance`.
// ! - Это позволяет безопасно и элегантно обработать разные «формы» одного типа.
