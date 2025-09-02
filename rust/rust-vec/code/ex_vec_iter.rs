fn main() {
    let mut names = vec![
        String::from("Капитан Нова"),
        String::from("Инженер Спаркс"),
        String::from("Доктор Люмен"),
        String::from("Биолог Рост"),
        String::from("Пилот Вега"),
        String::from("Связист Хекс"),
    ];

    // ❌ Ошибочный подход: iter() даёт неизменяемые ссылки (&String)
    for (i, name) in names.iter().enumerate() {
        *name = format!("R-{:03} {}", i + 1, name);
        println!("Подготовка кода для: {name} (#{})", i + 1);
    }
}

// ! **Почему не компилируется**
// ! - `names.iter()` возвращает неизменяемые ссылки `&String`.
// ! - Изменять значения по таким ссылкам запрещено.
// ! - Чтобы изменять элементы вектора, нужны изменяемые ссылки `&mut String.`

// ---

// ?hidden:start
fn main() {

    let mut names = vec![
        String::from("Капитан Нова"),
        String::from("Инженер Спаркс"),
        String::from("Доктор Люмен"),
        String::from("Биолог Рост"),
        String::from("Пилот Вега"),
        String::from("Связист Хекс"),
    ];
// ?hidden:end

    for (i, name) in names.iter_mut().enumerate() {
        *name = format!("R-{:03} {}", i + 1, *name);
    }

    println!("Коды присвоены:\n{:#?}", names);
}
// ! **Исправление капитана — iter_mut()**
// ! - Капитан объясняет:
// ! Теперь тип name стал &mut String и значение по ссылке можно изменять