// ?hidden:start
struct GpioPin;
struct OutputConfig;
struct Output;

impl Output {
    fn new(_pin: GpioPin, _config: OutputConfig) -> Self {
        Output
    }
    fn set_high(&mut self) {}
    fn set_low(&mut self) {}
}
// ?hidden:end

fn configure_gpio() {
    let pin_gpio2 = GpioPin;
    let mut led = Output::new(pin_gpio2, OutputConfig);

    // Управляем уровнем напряжения на пине
    led.set_high(); // 3.3V (Светодиод горит)
    led.set_low();  // 0.0V (Светодиод гаснет)
}

fn main() {
    configure_gpio();
    println!("GPIO 2 сконфигурирован успешно!");
}

// ! **Конфигурирование GPIO в Rust**
// ! - **Захват пина**: Безопасность типов Rust гарантирует, что пин GPIO2 не может быть одновременно использован в двух разных местах программы.
// ! - **Level::High (3.3V)**: Подает высокий уровень напряжения на вывод.
// ! - **Level::Low (0.0V)**: Подает нулевой уровень (заземление).
