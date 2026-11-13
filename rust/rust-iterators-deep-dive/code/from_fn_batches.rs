fn main() {
    // Нарезаем диапазон ID [0..100) на батчи по 25 штук
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

    let ranges: Vec<_> = batches.collect();
    println!("Батчи: {:?}", ranges);
}

// ! **Генератор батчей через std::iter::from_fn**
// ! - Замыкание хранит текущую позицию `cursor` и при каждом вызове `.next()` возвращает очередной диапазон `Range<u64>`.

