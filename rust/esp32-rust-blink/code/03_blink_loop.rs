// ?hidden:start
struct Led {
    is_on: bool,
}

impl Led {
    fn toggle(&mut self) {
        self.is_on = !self.is_on;
    }
}
// ?hidden:end

fn blink_simulation() {
    let mut led = Led { is_on: false };

    for step in 1..=4 {
        led.toggle();
        let status = if led.is_on { "ВКЛ (HIGH 💡)" } else { "ВЫКЛ (LOW 🌑)" };
        println!("[Шаг {}] Состояние светодиода: {}", step, status);
    }
}

fn main() {
    blink_simulation();
}

// ! **Бесконечный цикл и переключение состояния**
// ! - **led.toggle()**: Атомарно меняет состояние выхода с High на Low или наоборот.
// ! - **Delay**: Использование аппаратного таймера отсчитывает ровно 1000 мс паузы.
// ! - **loop**: Бесконечный цикл гарантирует, что прошивка продолжает работу всё время, пока подается питание.
