fn main() {
    let mut numbers = vec![10, 20, 30];

    println!("--- 1. .iter() — заимствование по &T ---");
    for num in numbers.iter() {
        println!("Элемент по ссылке: {num}");
    }

    println!("\n--- 2. .iter_mut() — изменение по &mut T ---");
    for num in numbers.iter_mut() {
        *num += 5;
    }
    println!("Модифицированный вектор: {:?}", numbers);

    println!("\n--- 3. .into_iter() — перемещение владения T ---");
    for num in numbers.into_iter() {
        println!("Поглощенное значение: {num}");
    }
    // numbers больше недоступен — владение перемещено в итератор!
}

// ! **Три вкуса заимствования при итерации**
// ! - `.iter()` выдает неизменяемые ссылки `&T`. Вектор остается доступен после цикла.
// ! - `.iter_mut()` выдает мутабельные ссылки `&mut T` для изменения элементов на месте.
// ! - `.into_iter()` поглощает коллекцию и выдает владение `T`. После вызова коллекция перемещается.

// ---

// Пользовательский итератор последовательности Фибоначчи
struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

// Реализуем ключевой типаж Iterator
impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

fn main() {
    println!("--- Первые 8 чисел Фибоначчи из нашего итератора ---");
    let fib_seq: Vec<u64> = Fibonacci::new().take(8).collect();
    println!("Результат: {:?}", fib_seq);
}

// ! **Реализация собственного типажа Iterator**
// ! - Для создания итератора достаточно задать ассоциированный тип `type Item` и метод `next(&mut self) -> Option<Item>`.
// ! - После реализации `next()`, объект автоматически бесплатно получает десятки методов-адаптеров (`take`, `map`, `filter`, `collect` и т.д.).

// ---

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("--- Цепочка адаптеров (Ленивые вычисления) ---");
    let iter = numbers.iter()
        .filter(|&&x| x % 2 == 0) // Оставляем четные
        .map(|&x| x * x);          // Возводим в квадрат

    // До вызова терминального метода (fold / collect) ни один элемент НЕ вычисляется!

    let sum_of_squares: i32 = iter.sum(); // Терминальный вызов производит вычисление
    println!("Сумма квадратов четных чисел: {sum_of_squares}");
}

// ! **Комбинаторы и Ленивые вычисления**
// ! - Адаптеры вроде `.map()` и `.filter()` являются ленивыми (lazy). Они создают новые итераторы, но не выполняют вычислений.
// ! - Вычисления запускаются только при вызове терминальных методов (`collect`, `sum`, `fold`, `for_each`).
// ! - Компилятор Rust объединяет цепочки адаптеров в один машинный цикл с нулевыми накладными расходами.
