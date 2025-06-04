#[derive(Debug)]
struct City {
    name: String,
    history: String,
}

#[derive(Debug)]
struct CitySummary {
    headline: String,
    description: String,
}

fn main() {
    let history = String::from("Город основан в 1574 году");

    let city = City {
        name: String::from("Уфа"),
        history: history,
    };

    let summary = CitySummary {
        headline: String::from("Уфа"),
        description: history, // ❌ перемещено из-под city
    };

    println!("{:?}, {:?}", city, summary);
}
// ! **1. Ошибка: попытка поделиться строкой напрямую между структурами**  
// ! - Мы либо копируем строку, либо нарушаем правила владения, переместив значение. ⚠️ _**Данные не могут быть одновременно в двух структурах**_. 

// ---

use std::rc::Rc;

#[derive(Debug)]
struct City {
    name: Rc<String>,
    history: Rc<String>,
}

#[derive(Debug)]
struct CitySummary {
    headline: Rc<String>,
    description: Rc<String>,
}

fn main() {
    let name = Rc::new(String::from("Уфа"));
    let history = Rc::new(String::from("Основан в 1574 году"));

    let city = City {
        name: Rc::clone(&name),
        history: Rc::clone(&history),
    };

    let summary = CitySummary {
        headline: Rc::clone(&name),
        description: Rc::clone(&history),
    };

    println!("Город: {:?} Сводка: {:?}", city, summary);
    println!("Ссылок на описание: {}", Rc::strong_count(&history));
}

// ! **2. Решение: использование Rc для разделения строк**
// ! - С Rc<T> можно безопасно использовать одни и те же данные в нескольких структурах. Владение разделено, а данные освобождаются когда последний владелец исчезает.