// Модуль с публичными и приватными элементами
mod example {
    // Публичная константа
    pub const ANSWER: i32 = 42;
    
    // Статическая переменная
    static mut COUNTER: i32 = 0;
    
    // Публичная структура
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }
    
    // Перечисление
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    
    // Типаж
    pub trait Drawable {
        fn draw(&self);
    }
    
    // Реализация типажа для Point
    impl Drawable for Point {
        fn draw(&self) {
            println!("Drawing point at ({}, {})", self.x, self.y);
        }
    }
    
    // Псевдоним типа
    pub type Coord = i32;
    
    // Публичная функция
    pub fn create_point(x: Coord, y: Coord) -> Point {
        Point { x, y }
    }
    
    // Функция с использованием нескольких ключевых слов
    pub fn process_direction(dir: Direction) {
        // Сопоставление с образцом
        match dir {
            Direction::Up => println!("Moving up"),
            Direction::Down => println!("Moving down"),
            Direction::Left => println!("Moving left"),
            Direction::Right => println!("Moving right"),
        }
        
        // Изменяемая переменная
        let mut temp = 5;
        
        // Цикл for
        for i in 0..3 {
            temp += i;
        }
        
        // Условный оператор
        if temp > 7 {
            println!("Temp is big");
        } else {
            println!("Temp is small");
        }
        
        // Бесконечный цикл с break
        loop {
            temp -= 1;
            if temp == 0 {
                break;
            }
        }
        
        // Цикл while
        while temp < 5 {
            temp += 1;
            if temp == 3 {
                continue; // Пропуск итерации
            }
        }
        
        // Приведение типов
        let float_temp = temp as f32;
    }
}

// Главная функция
fn main() {
    // Использование модуля
    use example::{Point, Direction, Drawable, process_direction};
    
    // Объявление переменной
    let point = Point { x: 10, y: 20 };
    let dir = Direction::Up;
    
    // Вызов функций
    point.draw();
    process_direction(dir);
    
    // Использование Self
    impl Point {
        fn new() -> Self {
            Point { x: 0, y: 0 }
        }
    }
    
    // Где-клауза (where clause)
    fn print_it<T>(value: T) 
    where
        T: std::fmt::Display,
    {
        println!("Value: {}", value);
    }
    
    print_it(42);
    
    // Возврат значения
    fn answer() -> i32 {
        return example::ANSWER;
    }
    
    // Макрос для вывода
    println!("The answer is {}", answer());
}

// ! **Базовый пример**  
// ! - Вывод текста  
// ! - Точка входа в программу

// ---
fn main() {
    let name = "Rust";
    println!("Hello, {}!", name);
}
// ! **Форматированный вывод**  
// ! - Переменная `name`  
// ! - Плейсхолдер `{}`

// ---
fn main() {
    let x = 5;
    let y = 10;
    println!("x + y = {}", x + y);
}
// ! **Математика**  
// ! - Сложение переменных  
// ! - Автоматический вывод типов

// ---
{
    let light_sensor = 20
    println!("Value of light sensor is {light_sensor}");
}
    println!("Value of light sensor is {light_sensor}"); // ERROR!!!
// ! **Математика**  
// ! - Сложение переменных  
// ! - Автоматический вывод типов
