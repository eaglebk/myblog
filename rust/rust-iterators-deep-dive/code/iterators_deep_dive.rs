fn main() {
    let mut numbers = vec![10, 20, 30];

    // 1. Заимствование по &T (эквивалентно for num in &numbers)
    for num in numbers.iter() {
        println!("По ссылке (&T): {num}");
    }

    // 2. Изменение на месте по &mut T (эквивалентно for num in &mut numbers)
    for num in numbers.iter_mut() {
        *num += 5;
    }
    println!("После iter_mut: {:?}", numbers);

    // 3. Поглощение коллекции: for num in numbers делает то же самое
    for num in numbers.into_iter() {
        println!("Во владение (T): {num}");
    }
    // Здесь numbers уже перемещён, обратиться к нему нельзя
}

// ! **Три режима итерации и IntoIterator**
// ! - `.iter()` (или `for x in &vec`) выдаёт `&T` и оставляет коллекцию нетронутой.
// ! - `.iter_mut()` (или `for x in &mut vec`) выдаёт `&mut T` для изменения элементов на месте.
// ! - `.into_iter()` (или `for x in vec`) забирает владение коллекцией и отдаёт значения `T`.

// ---

fn main() {
    let raw_inputs = ["10", "20", "bad_input", "40"];

    // Цепочка ленива: пока нет терминального метода, замыкания не запускаются
    let lazy_pipeline = raw_inputs
        .iter()
        .inspect(|s| println!("Читаю: {s}"))
        .map(|s| s.parse::<i32>());

    println!("Пайплайн создан, но ещё ничего не вычислено!");

    // collect::<Result<Vec<_>, _>>() остановится на первой ошибке и не дойдёт до "40"
    let parsed: Result<Vec<i32>, _> = lazy_pipeline.collect();
    println!("Результат разбора: {:?}", parsed);

    // .copied() убирает лишний уровень ссылки (&i32 -> i32)
    let nums = [1, -2, 3, 4];
    let positive_sum: i32 = nums.iter().copied().filter(|&x| x > 0).sum();
    println!("Сумма положительных: {positive_sum}");
}

// ! **Ленивые вычисления и short-circuiting**
// ! - Вызовы `.map()`, `.inspect()`, `.filter()` только оборачивают итератор в новую структуру.
// ! - Работа начинается только при вызове терминального метода (`collect`, `sum`, `find`).
// ! - `collect::<Result<Vec<_>, _>>()` прерывает обход сразу на первом `Err`, не тратя время на хвост коллекции.

// ---

use std::iter::FusedIterator;
use std::time::Duration;

struct RetryBackoff {
    current: Duration,
    max_delay: Duration,
    remaining: usize,
}

impl RetryBackoff {
    fn new(initial: Duration, max_delay: Duration, attempts: usize) -> Self {
        Self { current: initial, max_delay, remaining: attempts }
    }
}

impl Iterator for RetryBackoff {
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let delay = self.current.min(self.max_delay);
        self.current = self.current.saturating_mul(2).min(self.max_delay);
        Some(delay)
    }
}

// Гарантируем, что после исчерпания попыток итератор всегда будет возвращать None
impl FusedIterator for RetryBackoff {}

fn main() {
    let attempts = 5;
    let schedule: Vec<Duration> =
        RetryBackoff::new(Duration::from_millis(100), Duration::from_millis(500), attempts)
            .collect();
    println!("План задержек retry: {:?}", schedule);
}

// ! **Собственный Iterator на примере Exponential Backoff**
// ! - Для превращения структуры в итератор достаточно задать `type Item` и метод `next`.
// ! - `saturating_mul` и потолок `max_delay` защищают от переполнения, а `remaining == 0` позволяет реализовать `FusedIterator`.

// ---

use std::collections::VecDeque;
use std::iter::FusedIterator;

// Имитация удалённого API: отдаёт по 2 элемента на страницу
fn fetch_page(page: usize) -> Vec<String> {
    match page {
        p if p == 0 => vec!["user_1".into(), "user_2".into()],
        p if p == 1 => vec!["user_3".into(), "user_4".into()],
        p if p == 2 => vec!["user_5".into()],
        _ => vec![],
    }
}

struct PageIter {
    next_page: usize,
    buffer: VecDeque<String>,
    done: bool,
}

impl PageIter {
    fn new() -> Self {
        Self { next_page: 0, buffer: VecDeque::new(), done: false }
    }
}

impl Iterator for PageIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while self.buffer.is_empty() && !self.done {
            println!("-> Запрашиваю страницу {}", self.next_page);
            let items = fetch_page(self.next_page);
            if items.is_empty() {
                self.done = true;
            } else {
                self.next_page += 1;
                self.buffer.extend(items);
            }
        }
        self.buffer.pop_front()
    }
}

impl FusedIterator for PageIter {}

fn main() {
    // Загружаем только первые 3 записи: страница 2 даже не будет запрошена!
    let first_three: Vec<String> = PageIter::new().take(3).collect();
    println!("Получено записей: {:?}", first_three);
}

// ! **Практика: ленивый итератор пагинации**
// ! - Итератор запрашивает следующую страницу только тогда, когда локальный буфер `VecDeque` опустел.
// ! - Благодаря `.take(3)` лишние сетевые или дисковые запросы для оставшихся страниц не выполняются вообще.

// ---

// Возвращаем ленивый итератор без создания промежуточного Vec
fn non_empty_lines(text: &str) -> impl Iterator<Item = &str> + '_ {
    text.lines().map(str::trim).filter(|line| !line.is_empty())
}

fn main() {
    // 1. std::iter::from_fn - нарезка ID на батчи без отдельной структуры
    let mut cursor = Some(0u64);
    let batches = std::iter::from_fn(move || {
        let start = cursor?;
        if start >= 100 {
            cursor = None;
            return None;
        }
        let end = (start + 25).min(100);
        cursor = Some(end);
        Some(start..end)
    });
    println!("Размер обёртки from_fn на стеке: {} байт", std::mem::size_of_val(&batches));
    println!("Батчи ID: {:?}", batches.collect::<Vec<_>>());

    // 2. Ленивый парсинг строк без аллокаций
    let log = "\n  INFO start \n\n  WARN disk 85% \n";
    let lines: Vec<_> = non_empty_lines(log).collect();
    println!("Строки лога: {:?}", lines);

    // 3. Скользящее окно windows(2) вместо ручного slice[i + 1]
    let temps = [18, 21, 19, 24];
    let diffs: Vec<i32> = temps.windows(2).map(|w| w[1] - w[0]).collect();
    println!("Дельты температур: {:?}", diffs);
}

// ! **Идиомы стандартной библиотеки: from_fn, impl Iterator и windows**
// ! - `std::iter::from_fn` заменяет отдельный `struct + impl Iterator` для компактных генераторов.
// ! - `-> impl Iterator<Item = &str> + '_` возвращает ленивый поток, не выделяя `Vec` в куче.
// ! - `slice.windows(2)` заменяет ручную индексацию `slice[i + 1]` без проверок выхода за границы.
