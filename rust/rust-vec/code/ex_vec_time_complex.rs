use std::time::Instant;

fn main() {
    // Симулируем поток данных от датчиков
    let incoming: Vec<String> = (1..=20_000)
        .map(|i| format!("Прибор #{i:05}"))
        .collect();

    let start = Instant::now();
    let mut manifest: Vec<String> = Vec::new();

    let mut last_cap = manifest.capacity();
    let mut reallocs = 0;

    for item in incoming {
        manifest.push(item); // добавляем по одному
        if manifest.capacity() != last_cap {
            reallocs += 1;
            last_cap = manifest.capacity();
        }
    }

    println!(
        "Загрузка завершена: len={}, capacity={}, reallocs={}, elapsed={:?}",
        manifest.len(),
        manifest.capacity(),
        reallocs,
        start.elapsed()
    );
}
// ! **Что происходит:**
// ! - `Vec` начинает с минимальной ёмкости.
// ! - При каждом переполнении выделяется новый блок памяти → копирование старых данных.
// ! - На больших объёмах видно множество реаллокаций.

// ---

// ?hidden:start
use std::time::Instant;

fn main() {
    let incoming: Vec<String> = (1..=20_000)
        .map(|i| format!("Прибор #{i:05}"))
        .collect();
    let mut reallocs = 0;
    let start = Instant::now();

// ?hidden:end

    // Предвыделяем память под все элементы заранее
    let mut manifest: Vec<String> = Vec::with_capacity(incoming.len());

    // Массовая вставка
    manifest.extend(incoming);
    reallocs += 1;

    println!(
        "Загрузка завершена: len={}, capacity={}, reallocs={}, elapsed={:?}",
        manifest.len(),
        manifest.capacity(),
        reallocs,
        start.elapsed()
    );
}
// ! **Исправление капитана — предвыделение памяти)**
// ! - Капитан объясняет, что теперь:
// ! - нет скачков ёмкости,
// ! - данные загружаются быстрее,
// ! - память выделяется ровно один раз.

