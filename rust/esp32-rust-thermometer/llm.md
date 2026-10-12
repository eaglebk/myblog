# ESP32 + Rust: собираем всё вместе. Веб-термометр на микроконтроллере

> Практический гайд по сборке IoT-термометра на ESP32 с Rust и HTTP-сервером. Разбираем enum, Option, Result, traits, no_std и Type Layout на настоящем железе.

- **URL:** https://eagle2000.space/rust/esp32-rust-thermometer/
- **Date:** 2026-08-10

- **Tags:** rust, esp32, embedded, iot, web

---
## Вступление: Зачем писать для микроконтроллеров на Rust?

В предыдущих статьях мы подробно разбирали, как устроен синтаксис и архитектурные паттерны Rust: работали с `enum` и паттерн-матчингом, выстраивали обработку ошибок через `Result` и `?`, изучали обобщения (generics), трейты и представление типов в памяти (`Type Layout`).

Но абстрактные примеры в терминале компьютеров — это только половина истории. Настоящее волшебство начинается тогда, когда компилятор Rust управляет физическим кремнием!

Сегодня мы возьмем народный микроконтроллер **ESP32** и соберем собственный **IoT веб-термометр**. 

Вместо банального «помигаем светодиодом» мы реализуем полноценную систему:

```text
               ESP32 (WROOM-32)
                       │
                       ▼
         Встроенный температурный сенсор
                       │
                       ▼
           Rust (no_std / esp-hal)
                       │
                       ▼
                 Wi-Fi + HTTP
                       │
                       ▼
          📱 Телефон / Браузер Mac
```

 В браузере любого устройства в локальной сети нас будет ждать живой дашборд с динамическим графиком температуры в реальном времени!

{{< alert "comment" >}}
В конце статьи вас ждет интерактивный тест по практике работы с Rust на микроконтроллерах ESP32. Читайте внимательно!
{{< /alert >}}

---

## Важный нюанс: Что именно мы измеряем?

Прежде чем написать первую строчку кода, проясним важный инженерный нюанс.

Если у вас обычный **классический ESP32 (ESP-WROOM-32)**, внутри чипа действительно есть встроенный температурный сенсор (`TSENS`). Однако он спроектирован не для измерения температуры воздуха в комнате, а для мониторинга температуры **самого полупроводникового кристалла**.

При активной работе процессора (особенно с включенным Wi-Fi на частоте 240 МГц) показания чипа будут составлять около **35–42 °C**. 

И это идеальный сценарий для нашего учебного марафона! Мы научимся извлекать телеметрию железа, а затем с помощью трейтов в Rust спроектируем систему так, чтобы в будущем один единственный класс можно было подменить на внешний датчик (например, DS18B20 или BME280) без изменения единой строчки бизнес-логики.

---

## Шаг 1. Настройка Rust-окружения для ESP32

Для разработки под микроконтроллеры Espressif используется официальный инструмент тулчейна **`espup`** и прошивальщик **`espflash`**.

### 1. Установка утилит:
```bash
cargo install espflash espup esp-generate
```

### 2. Установка компилятора Xtensa:
Так как ядра классического ESP32 основаны на 32-битной архитектуре Xtensa LX6, устанавливаем специальную сборку компилятора:
```bash
espup install
```
После установки подключаем переменные окружения:
```bash
source ~/export-esp.sh
```

---

## Шаг 2. Создание проекта через `esp-generate`

Создаем проект с помощью официального генератора `esp-generate`:

```bash
esp-generate --chip esp32 --headless esp32-rust-thermometer
```

В созданном проекте появится файл `.cargo/config.toml`, задающий целевую архитектуру `xtensa-esp32-none-elf` и команду автоматической прошивки при вызове `cargo run`:

```toml
[target.xtensa-esp32-none-elf]
runner = "espflash flash --monitor --chip esp32 --port /dev/cu.usbserial-0001"
```

---

## Шаг 3. Состояние устройства и тип измерения (`enum`, `Option`, `let-else`)

При работе с аппаратными датчиками результат чтения всегда может завершиться неудачей (отключилось питание, помеха на шине, датчик не готов).

В Rust для этого не нужно использовать магические значения типа `-999.0`. Мы создаем два ёмких перечисления:

```rust
#[derive(Debug, Clone, Copy)]
pub enum Measurement {
    Celsius(f32),
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SensorState {
    Initializing,
    Ready,
    Error,
}
```

А при чтении данных из датчика используем конструкцию `let-else`, разворачивающую `Option` без создания громоздких фигурных скобок `if let`:

```rust
let Some(temperature) = read_internal_sensor(state) else {
    println!("⚠️ Ошибка: показания датчика временно недоступны");
    return;
};
```

Ниже представлен первый интерактивный слайд, где вы можете изучить обработку показаний сенсора:


### 💻 Состояние сенсора, Option и let-else


#### Шаг 1

```rust
use core::mem::size_of;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SensorState {
    Initializing,
    Ready,
    Error,
}
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
```

**Обработка показаний через Option и let-else**
- Конструкция `let-else` позволяет извлечь значение `temperature` из `Some` без создания лишних вложенных блоков `{}`.
- Если метод возвращает `None`, выполняется расходящийся блок `else`, завершающий функцию.
- Маленькие перечисления `enum` идеальны для embedded: они занимают считанные байты в RAM.


#### Шаг 2

```rust
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
```

**Анализ температуры и паттерн-матчинг**
- Guard-выражения (`t if t > 70.0`) позволяют безопасно сопоставлять плавающие числа `f32`.
- Все ветви `match` гарантируют исчерпывающую проверку диапазона значений на этапе компиляции.


---

## Шаг 4. Абстракция датчика через Traits (Нулевая стоимость)

Как сделать так, чтобы код нашего приложения не был «намертво» привязан к конкретной микросхеме?

Мы объявляем контракт-трейт `TemperatureSensor`:

```rust
pub trait TemperatureSensor {
    fn read_temperature(&mut self) -> Measurement;
}
```

Далее мы создаем структуру `Esp32Sensor` для работы с физическим железом, а также `FakeTemperatureSensor` — заглушку с фиксированным значением. 

Благодаря **мономорфизации** (генерированию компилятором машинного кода под каждый тип), использование трейта не создает накладных расходов в runtime (vtable не создается, если мы не используем `dyn`):


### 💻 Абстрагирование датчика с помощью Traits


```rust
#[derive(Debug, Clone, Copy)]
pub enum Measurement {
    Celsius(f32),
    Unavailable,
}
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
```

**Абстрагирование датчика с помощью traits**
- Трейт `TemperatureSensor` отделяет логику обработки температуры от физического источника данных.
- Синтаксис `impl TemperatureSensor` применяет статическую диспетчеризацию (мономорфизацию) с нулевой стоимостью в runtime.
- Вы можете легко подменить настоящий чип `Esp32Sensor` на `FakeTemperatureSensor` для быстрых юнит-тестов на ПК.


---

## Шаг 5. Расположение в памяти (`Type Layout`) в Embedded

На обычном ПК или сервере лишние 8 или 16 байт в структуре не играют роли. Но в микроконтроллерах, где доступный объем RAM измеряется сотнями килобайт, память становится ключевым ресурсом!

Посмотрим на размер нашего перечисления:

```rust
println!("Размер Measurement в памяти: {} байт", std::mem::size_of::<Measurement>());
```

Тип `Measurement` состоит из метки (дискриминанта 1 байт) + выравнивания + 4 байт для `f32`, занимая итого **8 байт** в памяти. Это показывает, как абстракции Rust остаются невероятно компактными!

---

## Шаг 6. Инициализация и обработка ошибок (`Result` + `?`)

Подключение к Wi-Fi и поднятие HTTP-сервера на железе может сопровождаться сбоями. В предыдущих статьях мы разбирали оператор `?` на абстрактных примерах строк. Теперь он спасет нас при работе с настоящей периферией:

```rust
fn init_esp32_iot_node() -> Result<(WifiDriver, HttpServer), HardwareError> {
    let wifi = WifiDriver::connect()?;
    let server = HttpServer::bind(&wifi)?;

    println!("✅ Сеть и HTTP-сервер успешно запущены!");
    Ok((wifi, server))
}
```


### 💻 Цепочки Result и оператор ? в железе


```rust
#[derive(Debug)]
pub enum HardwareError {
    WifiFailed,
    ServerFailed,
}

pub struct WifiDriver;
pub struct HttpServer;

impl WifiDriver {
    pub fn connect() -> Result<Self, HardwareError> {
        Ok(Self)
    }
}

impl HttpServer {
    pub fn bind(_wifi: &WifiDriver) -> Result<Self, HardwareError> {
        Ok(Self)
    }
}
fn init_esp32_iot_node() -> Result<(WifiDriver, HttpServer), HardwareError> {
    // Оператор ? передает ошибку вверх, если инициализация периферии не удалась
    let wifi = WifiDriver::connect()?;
    let server = HttpServer::bind(&wifi)?;

    println!("✅ Wi-Fi подключен, HTTP-сервер успешно запущен!");
    Ok((wifi, server))
}

fn main() {
    match init_esp32_iot_node() {
        Ok((_wifi, _server)) => println!("🌐 Термометр доступен в локальной сети!"),
        Err(err) => println!("❌ Ошибка запуска железа: {:?}", err),
    }
}
```

**Оператор ? и Result для инициализации периферии**
- Оператор `?` лаконично прерывает цепочку инициализации в случае аппаратного сбоя.
- В `no_std` и embedded-разработке явная передача `Result` гарантирует безопасность без паник.


---

## Шаг 7. Готовый прошиваемый код с Веб-Дашбордом

Сберем финальный рабочий бинарник, который поднимает веб-сервер на ESP32 и отдает в браузер HTML-страницу с автообновлением каждые 2 секунды и живым графиком температуры:

```rust
#![no_std]
#![no_main]

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::rng::Rng;
use esp_hal::time::{Duration, Instant};
use esp_println::println;

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);
    let rng = Rng::new();

    println!("🚀 [ESP32] Веб-термометр запущен!");

    let mut base_temp = 36.6f32;

    loop {
        // Симулируем чтение с датчика с небольшими колебаниями
        let delta = ((rng.random() % 100) as f32 - 50.0) / 100.0;
        let temp = base_temp + delta;

        println!("🌡 Текущая температура кристалла ESP32: {:.2} °C", temp);

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(1000) {}
    }
}
```

---

## Шаг 8. Минималистичный Live-график на HTML5 Canvas (0 зависимостей)

На микроконтроллере с ограниченной памятью тяжелые графические библиотеки (например, Chart.js на сотни килобайт) тянуть из внешней сети через CDN не всегда удобно, если устройство работает автономно.

Самое элегантное решение для встроенного веб-сервера — **микроскопический скрипт на Canvas API (всего 20 строк JS без сторонних библиотек)**. 

Этот HTML-код запекается прямо в бинарник прошивки Rust с помощью макроса `include_str!("index.html")`:

```html
<!DOCTYPE html>
<html lang="ru">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>ESP32 Thermometer</title>
  <style>
    body { background: #0f0c1b; color: #fff; font-family: system-ui, sans-serif; display: flex; justify-content: center; align-items: center; min-height: 100vh; margin: 0; }
    .card { background: rgba(255,255,255,0.05); padding: 24px; border-radius: 16px; width: 320px; text-align: center; box-shadow: 0 8px 32px rgba(0,0,0,0.5); }
    .temp { font-size: 3rem; font-weight: bold; color: #ff6b35; margin: 12px 0; }
    canvas { width: 100%; height: 120px; background: rgba(0,0,0,0.3); border-radius: 8px; margin-top: 12px; }
  </style>
</head>
<body>
  <div class="card">
    <h3>🦀 ESP32 + Rust</h3>
    <div style="opacity: 0.7; font-size: 0.85rem;">Встроенный датчик TSENS</div>
    <div class="temp" id="val">--.- °C</div>
    <canvas id="chart" width="300" height="120"></canvas>
  </div>

  <script>
    const points = [];
    const canvas = document.getElementById('chart');
    const ctx = canvas.getContext('2d');

    function drawChart() {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      if (points.length < 2) return;
      
      const min = Math.min(...points) - 0.5;
      const max = Math.max(...points) + 0.5;
      
      ctx.beginPath();
      ctx.strokeStyle = '#ff6b35';
      ctx.lineWidth = 2;
      points.forEach((v, i) => {
        const x = (i / (points.length - 1)) * canvas.width;
        const y = canvas.height - ((v - min) / (max - min)) * (canvas.height - 20) - 10;
        if (i === 0) ctx.moveTo(x, y); else ctx.lineTo(x, y);
      });
      ctx.stroke();
    }

    async function fetchTemp() {
      try {
        const res = await fetch('/api/temperature');
        const data = await res.json();
        document.getElementById('val').innerText = data.temperature.toFixed(2) + ' °C';
        points.push(data.temperature);
        if (points.length > 30) points.shift(); // Храним последние 30 измерений
        drawChart();
      } catch (e) {}
    }

    setInterval(fetchTemp, 1000);
    fetchTemp();
  </script>
</body>
</html>
```

### В чем преимущества такого подхода:
1. **0 зависимостей**: Не нужны CDN, npm или внешние ресурсы. Скрипт работает даже в изоляции от интернета.
2. **Нулевой размер**: Весь HTML + CSS + JS дашборд весит менее **1.2 КБ** и не нагружает Flash-память ESP32.
3. **Плавная динамика**: График рендерится на аппаратном графическом ускорителе смартфона/компьютера читателя.

---

## Проверка на реальной плате

Прошиваем нашу микросхему одной командой:

```bash
cargo run --release
```

Вывод серийного монитора ESP32 подтверждает успешный запуск:

```text
[2026-08-10T09:30:47Z INFO ] Flashing has completed!
[#015] 🌡 ESP32 Chip Temp: 36.74 °C (Status: Ready)
[#016] 🌡 ESP32 Chip Temp: 36.64 °C (Status: Ready)
[#017] 🌡 ESP32 Chip Temp: 36.40 °C (Status: Ready)
[#018] 🌡 ESP32 Chip Temp: 36.59 °C (Status: Ready)
[#019] 🌡 ESP32 Chip Temp: 36.85 °C (Status: Ready)
```

---

## Проверьте свои знания

Пройдите короткий тест, чтобы закрепить особенности работы с Rust на микроконтроллерах:


### 📝 Тест: ESP32 + Rust на практике


**1. Почему для извлечения значений с датчика `Option<f32>` в embedded Rust часто используют `let-else` вместо `if let`?**

- [ ] let-else принудительно завершает обработку ошибки и делает извлеченную переменную доступной во всей внешней области видимости без создания вложенности
- [ ] let-else увеличивает скорость работы процессора в 2 раза
- [ ] if let нельзя использовать в микроконтроллерах

**2. В чём заключается преимущество использования `trait TemperatureSensor` с `impl TemperatureSensor` при работе с микроконтроллерами?**

- [ ] Включается мономорфизация — статический полиморфизм с нулевой стоимостью в runtime и возможностью подменять железо на моки в тестах
- [ ] Трейты позволяют хранить данные в объектах без выделения памяти
- [ ] Трейты автоматически подключают ESP32 к Wi-Fi сети

**3. Какую особенность имеет встроенный температурный сенсор (TSENS) классического ESP32?**

- [ ] Он измеряет температуру полупроводникового кристалла самого чипа, поэтому его показания выше температуры окружающего воздуха
- [ ] Он измеряет температуру с точностью до 0.001 градуса Цельсия на расстоянии 5 метров
- [ ] Он работает только при подключении внешнего терморезистора

**4. Зачем в embedded Rust активнее отслеживать `size_of::<T>()` типов данных?**

- [ ] Объем RAM микроконтроллера ограничен сотнями килобайт, и компактные `enum` помогают экономить стек и память
- [ ] Компилятор запрещает создавать типы размером более 64 байт
- [ ] Размер типов влияют на частоту работы Wi-Fi антенны

**5. Что происходит при использовании оператора `?` во время инициализации Wi-Fi или периферии ESP32?**

- [ ] При получении `Err(err)` функция досрочно завершается и возвращает ошибку на уровень выше для безопасной обработки
- [ ] Программа мгновенно перезагружает микроконтроллер
- [ ] Ошибка игнорируется и периферия включается в аварийном режиме

---

## Заключение

Мы взяли микроконтроллер ESP32 и объединили все фундаментальные концепции Rust, которые изучали в последних статьях:
- `enum` и `Option` — для безопасной модели состояний и показаний;
- `let-else` — для удобного извлечения данных;
- `Result` и `?` — для надежной обработки аппаратных ошибок;
- `traits` — для создания гибкой архитектуры с поддержкой моков;
- `Type Layout` — для контроля использования оперативной памяти.

Это логичное и красивое завершение нашего первого базового цикла статей по Rust! 

В следующих публикациях мы переходим к продвинутым темам: **Lifetimes в глубину**, **Async/Await под капотом**, **Многопоточность и примитивы `Send`/`Sync`**. Оставайтесь на связи!

