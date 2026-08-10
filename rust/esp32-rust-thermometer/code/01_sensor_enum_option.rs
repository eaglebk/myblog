use core::mem::size_of;

// ?hidden:start
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SensorState {
    Initializing,
    Ready,
    Error,
}
// ?hidden:end

#[derive(Debug, Clone, Copy)]
pub enum Measurement {
    Celsius(f32),
    Unavailable,
}

fn read_internal_sensor(state: SensorState) -> Option<f32> {
    if state == SensorState::Ready {
        Some(36.75) // Показания встроенного датчика TSENS
    } else {
        None
    }
}

fn main() {
    let state = SensorState::Ready;

    // Читаем показания датчика
    let raw_reading = read_internal_sensor(state);

    // Извлекаем значение с помощью элегантной конструкции let-else
    let Some(temperature) = raw_reading else {
        println!("⚠️ Ошибка: показания датчика недоступны!");
        return;
    };

    let measurement = Measurement::Celsius(temperature);

    println!("🌡 Показания сенсора: {:.2} °C", temperature);
    println!("📦 Размер Measurement в памяти: {} байт", size_of::<Measurement>());
}

// ! **Обработка показаний через Option и let-else**
// ! - Конструкция `let-else` позволяет извлечь значение `temperature` из `Some` без создания лишних вложенных блоков `{}`.
// ! - Если метод возвращает `None`, выполняется расходящийся блок `else`, завершающий функцию.
// ! - Маленькие перечисления `enum` идеальны для embedded: они занимают считанные байты в RAM.

// ---

fn evaluate_chip_health(temp: f32) -> &'static str {
    // Внимание: числовые паттерны в match заменены на guard-условия (для корректной работы экстрактора)
    match temp {
        t if t > 70.0 => "🔥 Перегрев кристалла! Требуется охлаждение",
        t if t > 45.0 => "⚠️ Повышенный нагрев при нагрузке",
        _ => "✅ Нормальная рабочая температура",
    }
}

fn main() {
    let current_temp = 36.75;
    let status = evaluate_chip_health(current_temp);
    println!("Состояние чипа: {}", status);
}

// ! **Анализ температуры и паттерн-матчинг**
// ! - Guard-выражения (`t if t > 70.0`) позволяют безопасно сопоставлять плавающие числа `f32`.
// ! - Все ветви `match` гарантируют исчерпывающую проверку диапазона значений на этапе компиляции.
