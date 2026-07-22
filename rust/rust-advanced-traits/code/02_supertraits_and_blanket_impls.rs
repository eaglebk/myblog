// Шаг 1: Супертрейты (Supertraits)
use std::fmt::Display;

// Супертрейт Loggable требует, чтобы любой реализующий его тип ТАКЖЕ реализовывал Display
trait Loggable: Display {
    fn log_with_prefix(&self, prefix: &str) {
        // Мы можем использовать форматирование {}, так как Self: Display!
        println!("[{prefix}] {self}");
    }
}

struct User {
    username: String,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Пользователь({})", self.username)
    }
}

// Теперь мы можем реализовать Loggable для User:
impl Loggable for User {}

fn main() {
    let u = User { username: String::from("Алексей") };
    u.log_with_prefix("INFO");
}

// ! **1. Супертрейты (Supertraits)**
// ! - Синтаксис `trait Loggable: Display` накладывает условие: тип должен реализовывать базовый типаж `Display`.
// ! - Позволяет использовать методы базового типажа внутри реализации супертрейта.

// ---

// Шаг 2: Blanket-реализации (Ковровые реализации)
use std::fmt::Display as StandardDisplay;

trait Summary {
    fn print_summary(&self);
}

// Blanket Implementation: реализуем Summary сразу для ВСЕХ типов T, у которых есть Display!
impl<T: StandardDisplay> Summary for T {
    fn print_summary(&self) {
        println!("Автоматический вывод для типа: {self}");
    }
}

fn main() {
    // Вся стандартная библиотека и пользовательские типы с Display получили метод print_summary():
    let num = 42;
    num.print_summary();
    "Привет, Rust!".print_summary();
    true.print_summary();
}

// ! **2. Blanket Implementations**
// ! - Синтаксис `impl<T: TraitA> TraitB for T` автоматически наделяет типажем `TraitB` ВСЕ типы, удовлетворяющие `TraitA`.
// ! - Используется в стандартной библиотеке (например, `ToString` реализован для всех типов с `Display`).
