use std::cell::Cell;

struct Logger {
    log_count: Cell<usize>,
}

impl Logger {
    fn new() -> Self {
        Logger {
            log_count: Cell::new(0),
        }
    }

    // Обратите внимание: метод принимает &self (неизменяемую ссылку!), но меняет log_count!
    fn log(&self, message: &str) {
        println!("[LOG]: {message}");
        let current = self.log_count.get();
        self.log_count.set(current + 1);
    }
}

fn main() {
    let logger = Logger::new();

    logger.log("Первое сообщение системы");
    logger.log("Второе сообщение системы");

    println!("Всего записей в логе: {}", logger.log_count.get());

    // Метод replace заменяет значение и возвращает старое
    let old_val = logger.log_count.replace(100);
    println!("Старый счетчик: {old_val}, новый счетчик: {}", logger.log_count.get());
}

// ! **Внутренняя мутабельность с Cell<T>**
// ! - `Cell<T>` позволяет изменять значение внутри объекта, имея только неизменяемую ссылку `&self`.
// ! - Запись `set()` и чтение `get()` работают путем копирования или перемещения значения без создания ссылок `&T`.
// ! - Идеально подходит для простых типов (`Copy`) или флагов и счетчиков.

// ---

use std::cell::RefCell;

struct UserCache {
    cache: RefCell<Vec<String>>,
}

impl UserCache {
    fn new() -> Self {
        UserCache {
            cache: RefCell::new(Vec::new()),
        }
    }

    fn add_user(&self, name: String) {
        // Запрашиваем изменяемое заимствование во время выполнения
        self.cache.borrow_mut().push(name);
    }

    fn print_users(&self) {
        // Запрашиваем неизменяемое заимствование
        let users = self.cache.borrow();
        println!("Пользователи в кэше: {:?}", *users);
    }
}

fn main() {
    let cache = UserCache::new();

    cache.add_user("Алиса".to_string());
    cache.add_user("Борис".to_string());

    cache.print_users();

    // Проверка размера кэша
    println!("Количество пользователей: {}", cache.cache.borrow().len());
}

// ! **Динамическое заимствование с RefCell<T>**
// ! - `RefCell<T>` позволяет изменять сложные структуры в куче через `borrow_mut()` по ссылкам `&self`.
// ! - В отличие от `Cell`, `RefCell` выдает временные ссылки `Ref` и `RefMut`.
// ! - Проверки заимствований переносятся на runtime. Попытка открыть два `borrow_mut()` одновременно вызовет панику.
