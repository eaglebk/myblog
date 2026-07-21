use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Default)]
struct Node {
    name: String,
    next: Option<Rc<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Деструктор узла '{}' вызван!", self.name);
    }
}

fn main() {
    println!("--- Демонстрация сильных ссылок ---");
    let first = Rc::new(RefCell::new(Node {
        name: "Узел А".to_string(),
        next: None,
    }));
    
    let second = Rc::new(RefCell::new(Node {
        name: "Узел Б".to_string(),
        next: None,
    }));

    println!("Сильные ссылки 'first': {}", Rc::strong_count(&first));
    println!("Сильные ссылки 'second': {}", Rc::strong_count(&second));
    
    // Связываем узел А -> узел Б
    first.borrow_mut().next = Some(Rc::clone(&second));
    println!("После связывания 'second' strong_count: {}", Rc::strong_count(&second));
}

// ! **Подсчет сильных ссылок Rc**
// ! - `Rc::strong_count(&a)` позволяет узнать количество владельцев объекта в куче.
// ! - Каждый `Rc::clone` увеличивает `strong_count` на 1, а при выходе переменной из scope — уменьшает на 1.
// ! - Если узел А владеет узлом Б через `Rc`, счетчик ссылок Б становится равен 2.

// ---

use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Gadget {
    id: String,
    owner: Option<Weak<Owner>>, // Ослабленная ссылка на владельца предотвращает циклический вызов Drop!
}

struct Owner {
    name: String,
    gadgets: RefCell<Vec<Rc<RefCell<Gadget>>>>,
}

impl Drop for Gadget {
    fn drop(&mut self) {
        println!("Устройство '{}' удалено из памяти", self.id);
    }
}

impl Drop for Owner {
    fn drop(&mut self) {
        println!("Владелец '{}' удален из памяти", self.name);
    }
}

fn main() {
    println!("--- Безопасная связь через Weak ---");
    
    let owner = Rc::new(Owner {
        name: "Алексей".to_string(),
        gadgets: RefCell::new(Vec::new()),
    });

    let gadget = Rc::new(RefCell::new(Gadget {
        id: "Смартфон".to_string(),
        owner: Some(Rc::downgrade(&owner)), // Создаем ослабленную ссылку Weak<Owner>
    }));

    owner.gadgets.borrow_mut().push(Rc::clone(&gadget));

    println!("Сильные ссылки владельца: {}", Rc::strong_count(&owner));
    println!("Слабые ссылки владельца: {}", Rc::weak_count(&owner));

    // Для обращения к владельцу из устройства используем upgrade():
    if let Some(ref weak_owner) = gadget.borrow().owner {
        if let Some(upgraded_owner) = weak_owner.upgrade() {
            println!("Устройство принадлежит владельцу: {}", upgraded_owner.name);
        }
    }
}

// ! **Решение циклов через Weak ссылки (downgrade и upgrade)**
// ! - `Rc::downgrade(&rc)` создает слабую ссылку `Weak<T>`, увеличивая `weak_count`, но НЕ `strong_count`.
// ! - `Weak<T>` не препятствует деаллокации объекта, когда все сильные ссылки выходят из области видимости.
// ! - Для доступа к данным `Weak::upgrade()` возвращает `Option<Rc<T>>`. Если объект уже удален, вернется `None`.
