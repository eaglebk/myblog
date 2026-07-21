use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
use std::thread;

// Элемент очереди ухода с приоритетом
#[derive(Eq, PartialEq)]
struct CareTask {
    priority: u32,
    description: String,
}

impl Ord for CareTask {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority) // Max-Heap по приоритету
    }
}

impl PartialOrd for CareTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    // Многопоточный хаб управления приютом/кормушкой через Arc<Mutex>
    let schedule = Arc::new(Mutex::new(BinaryHeap::new()));

    // Фоновый поток добавляет регулярные кормления
    let sched_clone = Arc::clone(&schedule);
    let handle = thread::spawn(move || {
        let mut lock = sched_clone.lock().unwrap();
        lock.push(CareTask { priority: 10, description: "Плановое кормление в 18:00".to_string() });
    });

    // Главный поток регистрирует срочный прием лекарства
    {
        let mut lock = schedule.lock().unwrap();
        lock.push(CareTask { priority: 100, description: "СРОЧНО: Выдать инсулин коту Мурзику".to_string() });
    }

    handle.join().unwrap();

    println!("--- Выполнение задач ухода по степени важности ---");
    let mut lock = schedule.lock().unwrap();
    while let Some(task) = lock.pop() {
        println!("Выполняется [Приоритет {}]: {}", task.priority, task.description);
    }
}

// ! **1. Многопоточный хаб и Очередь Приоритетов**
// ! - `Arc<Mutex<...>>` обеспечивает безопасный доступ к планировщику из параллельных потоков.
// ! - `BinaryHeap` гарантирует, что критические медицинские задачи (приоритет 100) выталкиваются раньше планового кормления.

// ---

struct MedicalAlert {
    pet_name: String,
    is_critical: bool,
}

// Пользовательский итератор для фильтрации критических уведомлений
struct CriticalAlertsIterator<I> {
    inner: I,
}

impl<I: Iterator<Item = MedicalAlert>> Iterator for CriticalAlertsIterator<I> {
    type Item = MedicalAlert;

    fn next(&mut self) -> Option<Self::Item> {
        // Пропускаем некритические предупреждения лениво
        self.inner.find(|alert| alert.is_critical)
    }
}

fn main() {
    let alerts = vec![
        MedicalAlert { pet_name: "Шарик".to_string(), is_critical: false },
        MedicalAlert { pet_name: "Васька".to_string(), is_critical: true },
        MedicalAlert { pet_name: "Чижик".to_string(), is_critical: true },
    ];

    let critical_iter = CriticalAlertsIterator { inner: alerts.into_iter() };

    println!("--- Ленивая фильтрация экстренных сигналов ---");
    for alert in critical_iter {
        println!("ЭКСТРЕННЫЙ ВЫЗОВ ВЕТЕРИНАРА: Питомец {}", alert.pet_name);
    }
}

// ! **2. Кастомный Итератор Экстренных Уведомлений**
// ! - Реализация собственного типажа `Iterator` производит отбор критических элементов лениво во время обхода.
