trait Task {
    fn execute(&self) -> String;
}

struct NetworkTask;
impl Task for NetworkTask {
    fn execute(&self) -> String { "Сеть".to_string() }
}

struct DbTask;
impl Task for DbTask {
    fn execute(&self) -> String { "База данных".to_string() }
}

fn main() {
    // Вектор содержит разные типы данных под общим трейт-объектом
    let tasks: Vec<Box<dyn Task>> = vec![
        Box::new(NetworkTask),
        Box::new(DbTask),
    ];
    for t in tasks {
        println!("Выполняю задачу: {}", t.execute());
    }
}

// ! **Динамический полиморфизм и dyn Trait**
// ! - Трейт-объект `dyn Task` позволяет работать с гетерогенными коллекциями, где типы известны только в рантайме.
// ! - Для этого объекты обязательно помещаются за указатель (например, `Box<dyn Task>` или `&dyn Task`).
// ! - Определение вызываемого метода происходит во время выполнения через таблицу виртуальных методов (vtable).

// ---

// По умолчанию все дженерики имеют ограничение T: Sized.
// Мы отключаем его с помощью ?Sized, чтобы функция могла принимать слайсы или dyn Trait.
fn print_debug<T: ?Sized + std::fmt::Debug>(val: &T) {
    println!("Значение: {:?}", val);
}

fn main() {
    let s: &str = "Привет, мир!"; // str — это DST (Dynamically Sized Type)
    print_debug(s);
}

// ! **Неявный bound Sized и ?Sized**
// ! - Большинство типов имеют размер, известный при компиляции (Sized), и могут создаваться на стеке.
// ! - Динамические типы (DST), такие как `str`, `[T]` или `dyn Trait`, не имеют фиксированного размера.
// ! - Синтаксис `?Sized` отменяет неявное требование фиксированного размера, позволяя передавать DST по ссылке.

// ---

trait Fly {
    fn fly(&self) -> String;
}

trait Swim {
    fn swim(&self) -> String;
}

// Rust запрещает &(dyn Fly + Swim) напрямую.
// Создаем общий трейт-субститут:
trait Duck: Fly + Swim {}

// И пишем Blanket-реализацию для всех подходящих типов:
impl<T: Fly + Swim> Duck for T {}

struct Mallard;
impl Fly for Mallard {
    fn fly(&self) -> String { "Летит".to_string() }
}
impl Swim for Mallard {
    fn swim(&self) -> String { "Плывет".to_string() }
}

fn handle_duck(duck: &dyn Duck) {
    println!("Утка: {}, {}", duck.fly(), duck.swim());
}

fn main() {
    let d = Mallard;
    handle_duck(&d);
}

// ! **Двойные трейт-объекты**
// ! - В Rust нельзя напрямую составить трейт-объект из нескольких независимых трейтов: `dyn Trait1 + Trait2` не скомпилируется.
// ! - Для обхода создается собирающий трейт `Duck`, наследующий от нужных супертрейтов.
// ! - Blanket implementation автоматически делает любой подходящий тип наследником `Duck`.

// ---

// Трейт содержит метод `copy_by_val`, который возвращает Self по значению.
// Это ломает Object Safety (динамическую совместимость).
// Но мы изолируем его с помощью where Self: Sized!
trait SafeTrait {
    fn name(&self) -> String;
    
    // Этот метод не будет включен в vtable и не сломает dyn SafeTrait
    fn copy_by_val(self) -> Self where Self: Sized {
        self
    }
}

struct Item;
impl SafeTrait for Item {
    fn name(&self) -> String { "Предмет".to_string() }
}

fn print_name(obj: &dyn SafeTrait) {
    println!("Имя: {}", obj.name());
    // obj.copy_by_val(); // Ошибка! Метод недоступен для трейт-объекта.
}

fn main() {
    let item = Item;
    print_name(&item);
}

// ! **Динамическая совместимость (Object Safety)**
// ! - Трейт-объект можно создать только для Object Safe трейтов (нет Self по значению, нет дженерик-методов и т.д.).
// ! - Конструкция `where Self: Sized` позволяет исключить проблемные методы из vtable.
// ! - Трейт остается динамически совместимым, а «опасные» методы можно вызывать только у статических типов.
