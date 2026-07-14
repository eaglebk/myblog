fn main() {
    let mut telemetry: Vec<i32> = Vec::new();

    // Симулируем поток данных
    for i in 0..200 {
        telemetry.push(i);
    }

    println!("Собрано {} значений", telemetry.len());
}

// ! **Проблема:**
// ! - буфер растёт бесконечно,
// ! - через некоторое время память будет исчерпана.

// ---

fn main() {
    let mut telemetry: Vec<i32> = Vec::new();
    let max_len = 100;

    for i in 0..200 {
        telemetry.push(i);

        // если размер превысил лимит, убираем старые записи
        if telemetry.len() > max_len {
            telemetry.remove(0); // ❌ неэффективно, сдвигает элементы
        }
    }

    println!("Буфер телеметрии: {:?}", telemetry);
}
// ! **Исправление капитана — ограниченный буфер**
// ! - Работает, но `remove(0)` сдвигает все элементы - на больших объёмах это дорого.


// ---

fn main() {
    let mut telemetry: Vec<i32> = Vec::new();
    let max_len = 100;

    for i in 0..200 {
        telemetry.push(i);

        if telemetry.len() > max_len {
            let overflow = telemetry.len() - max_len;
            telemetry.drain(0..overflow);
        }
    }

    println!("Буфер телеметрии: {:?}", telemetry);
}

// ! **Ещё лучше — `drain`**
// ! - `drain(0..overflow)` удаляет сразу несколько элементов с начала.
// ! - В буфере всегда остаются последние `max_len` записей.


// ---
// ?hidden:start
fn main() {
    let mut telemetry: Vec<i32> = Vec::new();
    let max_len = 100;

    for i in 0..200 {
        telemetry.push(i);
    }
// ?hidden:end

    let last100: Vec<i32> = telemetry.iter().rev().take(100).cloned().collect();


    println!("Буфер телеметрии: {:?}", last100);
}
// ! **Альтернативное решение — срез последних элементов**
// ! - Иногда проще просто брать «хвост» вектора:

