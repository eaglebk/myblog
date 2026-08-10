// ?hidden:start
#[derive(Debug, Clone, Copy)]
pub enum Measurement {
    Celsius(f32),
    Unavailable,
}
// ?hidden:end

pub trait TemperatureSensor {
    fn read_temperature(&mut self) -> Measurement;
}

pub struct Esp32Sensor {
    chip_id: &'static str,
}

impl Esp32Sensor {
    pub fn new(chip_id: &'static str) -> Self {
        Self { chip_id }
    }
}

impl TemperatureSensor for Esp32Sensor {
    fn read_temperature(&mut self) -> Measurement {
        // Чтение встроенного датчика TSENS чипа ESP32
        Measurement::Celsius(36.8)
    }
}

pub struct FakeTemperatureSensor {
    simulated_temp: f32,
}

impl TemperatureSensor for FakeTemperatureSensor {
    fn read_temperature(&mut self) -> Measurement {
        // Идеально для unit-тестирования без настоящего железа
        Measurement::Celsius(self.simulated_temp)
    }
}

fn print_sensor_data(sensor: &mut impl TemperatureSensor) {
    let Measurement::Celsius(val) = sensor.read_temperature() else {
        println!("Ошибка датчика");
        return;
    };
    println!("Температура в градусах Цельсия: {:.2} °C", val);
}

fn main() {
    let mut esp_hw = Esp32Sensor::new("ESP32-WROOM-32");
    let mut mock_hw = FakeTemperatureSensor { simulated_temp: 22.5 };

    print_sensor_data(&mut esp_hw);
    print_sensor_data(&mut mock_hw);
}

// ! **Абстрагирование датчика с помощью traits**
// ! - Трейт `TemperatureSensor` отделяет логику обработки температуры от физического источника данных.
// ! - Синтаксис `impl TemperatureSensor` применяет статическую диспетчеризацию (мономорфизацию) с нулевой стоимостью в runtime.
// ! - Вы можете легко подменить настоящий чип `Esp32Sensor` на `FakeTemperatureSensor` для быстрых юнит-тестов на ПК.
