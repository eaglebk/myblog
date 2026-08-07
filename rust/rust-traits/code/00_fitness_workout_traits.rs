// ?compile_fail
struct Pushups {
    reps: u32,
}

impl Pushups {
    fn calories_burned(&self) -> u32 {
        self.reps * 1
    }
}

// Попытка написать обобщенную функцию БЕЗ ограничения трейтом T
fn total_calories<T>(exercises: &[T]) -> u32 {
    exercises.iter().map(|e| e.calories_burned()).sum()
}

fn main() {
    let pushups = vec![Pushups { reps: 20 }, Pushups { reps: 30 }];
    let total = total_calories(&pushups);
    println!("Всего калорий: {}", total);
}

// ! **Шаг 1: Ошибка компиляции без ограничения трейта**
// ! - Попытка вызвать метод `calories_burned()` у обобщенного типа `T` без объявления контракта.
// ! - Компилятор не знает, есть ли у типа `T` такой метод, и блокирует сборку.

// ---

pub trait Workout {
    fn calories_burned(&self) -> u32;
}

struct Pushups {
    reps: u32,
}

struct Plank {
    duration_sec: u32,
}

impl Workout for Pushups {
    fn calories_burned(&self) -> u32 {
        self.reps * 1
    }
}

impl Workout for Plank {
    fn calories_burned(&self) -> u32 {
        self.duration_sec * 2
    }
}

// Единая обобщенная функция с ограничением Trait Bound (T: Workout)
fn total_calories<T: Workout>(exercises: &[T]) -> u32 {
    exercises.iter().map(|e| e.calories_burned()).sum()
}

fn main() {
    let pushups = vec![Pushups { reps: 20 }, Pushups { reps: 30 }];
    let planks = vec![Plank { duration_sec: 60 }, Plank { duration_sec: 45 }];

    println!("Калории от отжиманий: {}", total_calories(&pushups));
    println!("Калории от планки: {}", total_calories(&planks));
}

// ! **Шаг 2: Решение с явным трейтом Workout**
// ! - Трейт `Workout` декларирует обязательный контракт: наличие метода `calories_burned`.
// ! - Структуры `Pushups` и `Plank` реализуют типаж через `impl Workout for ...`.
// ! - Функция `total_calories` принимает любой тип `T: Workout`, а компилятор генерирует оптимизированный код под каждый тип без задержек в рантайме.
