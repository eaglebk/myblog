// ?compile_fail

struct Button {
    label: String,
    click_count: u32,
}

impl Button {
    fn on_click(&self) {
        println!("Кнопка '{}' нажата!", self.label);
        self.click_count += 1;
    }
}

fn main() {
    let btn = Button {
        label: String::from("Отправить"),
        click_count: 0,
    };
    btn.on_click();
}

// ! **Без Cell: Ошибка компиляции**
// ! - Компилятор запрещает изменять поле `click_count` по неизменяемой ссылке `&self`.
// ! - Сделать метод `&mut self` в UI-компонентах часто нельзя, так как кнопка доступна одновременно из разных частей приложения.

// ---

use std::cell::Cell;

struct Button {
    label: String,
    click_count: Cell<u32>,
}

impl Button {
    fn on_click(&self) {
        println!("Кнопка '{}' нажата!", self.label);
        let count = self.click_count.get();
        self.click_count.set(count + 1);
    }
}

fn main() {
    let btn = Button {
        label: String::from("Отправить"),
        click_count: Cell::new(0),
    };

    btn.on_click();
    btn.on_click();
    println!("Всего кликов по кнопке: {}", btn.click_count.get());
}

// ! **Решение с Cell: Код компилируется**
// ! - Орачиваем счетчик в `Cell<u32>`.
// ! - Метод `on_click(&self)` успешно обновляет значение через `.set()`, сохраняя ссылку `&self`.
