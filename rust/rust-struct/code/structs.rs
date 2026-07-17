#[allow(dead_code)]
#[derive(Debug)]
struct Hub {
    name: String,
    id: String,
    location: String,
    sensors: Vec<Sensor>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Sensor {
    name: String,
    id: String,
    sensor_type: String,
    value: f32,
    unit: String,
    is_active: bool,
}

fn main() {
    let hub = Hub {
        name: "Main Hub".to_string(),
        id: String::from("HUB001"),
        location: "Room 101".to_string(),
        sensors: Vec::new(), // пустой список сенсоров
    };

    println!("{:#?}", hub);
}
// ! **Объявление и базовая инициализация структур**
// ! - Структуры определяются с помощью ключевого слова `struct`.
// ! - Атрибут `#[derive(Debug)]` позволяет выводить структуру в отладочном формате с помощью `{:#?}`.

// ---

#[allow(dead_code)]
#[derive(Debug)]
struct Hub {
    name: String,
    id: String,
    location: String,
    sensors: Vec<Sensor>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Sensor {
    name: String,
    id: String,
    sensor_type: String,
    value: f32,
    unit: String,
    is_active: bool,
}

fn main() {
    let sensor1 = Sensor {
        name: "Temperature Sensor".to_string(),
        id: "TEMP001".to_string(),
        sensor_type: "Temperature".to_string(),
        value: 25.3,
        unit: "°C".to_string(),
        is_active: true,
    };

    let sensor2 = Sensor {
        name: "Humidity Sensor".to_string(),
        id: "HUM001".to_string(),
        sensor_type: "Humidity".to_string(),
        value: 60.5,
        unit: "%".to_string(),
        is_active: true,
    };

    let hub = Hub {
        name: "Main Hub".to_string(),
        id: String::from("HUB001"),
        location: "Room 101".to_string(),
        sensors: vec![sensor1, sensor2],
    };

    println!("{:#?}", hub);
}
// ! **Структуры со вложенными коллекциями**
// ! - Мы создали экземпляры `Sensor` и передали их в поле `sensors` структуры `Hub`.
// ! - Макрос `vec!` используется для удобной инициализации вектора `Vec<Sensor>`.

// ---

#[derive(Debug, Default)]
struct Foo {
    a: u32,
    b: u8,
}

fn main() {
    let a = 42;
    let b = 13;

    // Сокращенный синтаксис инициализации (field init shorthand)
    // Имена переменных совпадают с именами полей
    let x = Foo { a, b };

    // Синтаксис перехода (struct update syntax ..)
    // Копирование (перенос) значений оставшихся полей из другой структуры
    let y = Foo { a: 50, ..x };

    // Инициализация структуры с дефолтными значениями, кроме поля a
    let z = Foo { a: 100, ..Default::default() };

    println!("x: {:?}", x);
    println!("y: {:?}", y);
    println!("z: {:?}", z);
}
// ! **Сокращенный синтаксис и синтаксис перехода**
// ! - Если имена переменных совпадают с именами полей, можно писать `a` вместо `a: a` (field init shorthand).
// ! - Синтаксис `..` позволяет скопировать остальные поля из другого экземпляра той же структуры.
// ! - Типаж `Default` позволяет инициализировать поля по умолчанию с помощью `..Default::default()`.

// ---

// Объявление структуры-кортежа
#[derive(Debug)]
struct Bar(u32, bool);

fn main() {
    // Создание экземпляра
    let x = Bar(42, true);

    // Доступ к полям по порядковому индексу
    let number = x.0;
    let flag = x.1;

    println!("x: {:?}", x);
    println!("Поля: number = {}, flag = {}", number, flag);

    // Неявное представление конструктора структуры-кортежа как функции
    let constructor: fn(u32, bool) -> Bar = Bar;
    let y = constructor(100, false);
    println!("y: {:?}", y);
}
// ! **Структуры-кортежи (Tuple Structs)**
// ! - Поля в структурах-кортежах не имеют названий, а определяются только своими типами.
// ! - Доступ к полям осуществляется по порядковому индексу через точку: `x.0`, `x.1`.
// ! - Имя структуры-кортежа выступает в роли функции-конструктора (например, `Bar` имеет тип `fn(u32, bool) -> Bar`).

// ---

// Объявление юнит-структуры
#[derive(Debug)]
struct Baz;

fn main() {
    // Корректное создание (без использования скобок)
    let x = Baz;

    println!("x: {:?}", x);
}
// ! **Юнит-структуры (Unit-like Structs)**
// ! - Объявляются вообще без использования скобок.
// ! - Создаются также без использования скобок: `let x = Baz;`.
// ! - Обычно используются как маркеры или для реализации типажей на них без хранения данных.

// ---

// Структура с полями разного уровня видимости
#[allow(dead_code)]
pub struct VisibleStruct {
    pub a: u32,          // Публичное поле (доступно везде)
    pub(crate) b: u8,    // Доступно только внутри текущего крейта
    c: u64,              // Приватное поле (доступно только внутри текущего модуля)
}

fn main() {
    let x = VisibleStruct { a: 1, b: 2, c: 3 };
    println!("a = {}, b = {}", x.a, x.b);
    // Доступ к x.c разрешен, так как main находится в том же модуле
    println!("c = {}", x.c);
}
// ! **Управление видимостью полей**
// ! - Поля структур приватны по умолчанию.
// ! - Модификатор `pub` делает поле видимым извне текущего модуля и крейта.
// ! - Модификатор `pub(crate)` делает поле видимым только внутри текущего крейта.
