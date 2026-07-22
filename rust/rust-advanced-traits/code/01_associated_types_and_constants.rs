// Шаг 1: Ассоциированные типы (Associated Types) и ассоциированные константы

trait Container {
    // Ассоциированный тип определяемый реализацией:
    type Item;

    // Ассоциированная константа:
    const CAPACITY_LIMIT: usize = 100;

    fn add(&mut self, item: Self::Item);
    fn count(&self) -> usize;
}

struct IntStack {
    items: Vec<i32>,
}

impl Container for IntStack {
    type Item = i32;

    fn add(&mut self, item: Self::Item) {
        if self.items.len() < Self::CAPACITY_LIMIT {
            self.items.push(item);
        }
    }

    fn count(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    let mut stack = IntStack { items: vec![] };
    stack.add(42);
    println!("Размер стека: {}, Макс. лимит: {}", stack.count(), IntStack::CAPACITY_LIMIT);
}

// ! **1. Ассоциированные типы и константы**
// ! - `type Item;` связывает внутренний тип данных с типажем, не засоряя сигнатуры универсальными типами `trait Container<T>`.
// ! - `const CAPACITY_LIMIT: usize` определяет константу, связанную с пространством имен типажа.

// ---

// Шаг 2: Полностью уточненный синтаксис (Fully Qualified Syntax) для устранения неоднозначностей
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Пилот: поднять самолет в воздух!");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Волшебник: лететь на метле!");
    }
}

impl Human {
    fn fly(&self) {
        println!("Человек: махать руками на земле.");
    }
}

fn main() {
    let person = Human;

    // 1. Вызов собственного метода структуры:
    person.fly();

    // 2. Использование Fully Qualified Syntax для вызова конкретного метода типажа:
    Pilot::fly(&person);
    Wizard::fly(&person);
    <Human as Pilot>::fly(&person);
}

// ! **2. Fully Qualified Syntax (<Type as Trait>::method)**
// ! - Разрешает конфликты, если тип реализует несколько типажей с одинаковыми именами методов.
// ! - Синтаксис `<Type as Trait>::function(...)` однозначно указывает компилятору конкретную реализацию.
