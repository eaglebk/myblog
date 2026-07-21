use std::collections::VecDeque;

fn main() {
    // Двусторонняя очередь (Ring Buffer)
    let mut queue = VecDeque::with_capacity(5);

    // Добавление в конец очереди:
    queue.push_back("Задача 1");
    queue.push_back("Задача 2");

    // Срочное добавление в начало очереди:
    queue.push_front("Приоритетная задача!");

    println!("Элементы очереди: {:?}", queue);

    // Извлечение с противоположных концов за O(1):
    if let Some(first) = queue.pop_front() {
        println!("Обработана из начала: {first}");
    }

    if let Some(last) = queue.pop_back() {
        println!("Обработана с конца: {last}");
    }
}

// ! **Двусторонняя очередь: VecDeque**
// ! - `VecDeque<T>` реализован как кольцевой буфер (Ring Buffer).
// ! - Позволяет выполнять вставку и удаление за `O(1)` как с начала (`push_front`/`pop_front`), так и с конца (`push_back`/`pop_back`).
// ! - Идеальный выбор для реализаций FIFO-очередей задач или буферов последних сообщений.

// ---

use std::collections::BTreeMap;

fn main() {
    let mut sensor_data = BTreeMap::new();

    // Вставка ключей в случайном порядке
    sensor_data.insert(104, "Серверная");
    sensor_data.insert(101, "Прихожая");
    sensor_data.insert(103, "Кухня");
    sensor_data.insert(102, "Гостиная");

    println!("--- Итерация всегда строго отсортирована по ключу ---");
    for (id, room) in &sensor_data {
        println!("Датчик ID {id}: {room}");
    }

    println!("\n--- Срез по диапазону ключей (Range query 102..=103) ---");
    for (id, room) in sensor_data.range(102..=103) {
        println!("Диапазон: ID {id} -> {room}");
    }
}

// ! **Упорядоченный словарь: BTreeMap**
// ! - `BTreeMap<K, V>` устроен как B-дерево. Ключи всегда хранятся в отсортированном виде.
// ! - Гарантирует логарифмическую сложность `O(log N)` для поиска, вставки и удаления.
// ! - Позволяет выполнять быструю выборку по диапазонам ключей через метод `.range()`.

// ---

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Job {
    priority: u32,
    name: String,
}

// Реализуем Ord для сортировки очереди по приоритету
impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut heap = BinaryHeap::new();

    heap.push(Job { priority: 10, name: "Фоновая отгрузка логов".to_string() });
    heap.push(Job { priority: 90, name: "Аварийное отключение питания".to_string() });
    heap.push(Job { priority: 50, name: "Обновление конфигурации".to_string() });

    println!("--- Выборка задач в порядке приоритета (Max-Heap) ---");
    while let Some(job) = heap.pop() {
        println!("Выполняется [Приоритет {}]: {}", job.priority, job.name);
    }
}

// ! **Очередь с приоритетом: BinaryHeap**
// ! - `BinaryHeap<T>` — это Max-Куча. Метод `.pop()` всегда первым возвращает максимальный элемент.
// ! - Вставка `push()` и удаление `pop()` выполняются за `O(log N)`.
// ! - Требует от типа реализации типажа `Ord` и `Eq`. Идеально подходит для планировщиков задач.
