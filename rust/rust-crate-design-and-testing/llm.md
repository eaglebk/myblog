# Проектирование библиотек в Rust: Data-Oriented API, разделение lib/main и бебаггинг (Bebugging)

> Первая статья серии по разработке утилит и библиотек в Rust: архитектура Crate, Data-Oriented API, разделение lib.rs/main.rs, техника Bebugging и самодокументируемые тесты.

- **URL:** https://eagle2000.space/rust/rust-crate-design-and-testing/
- **Date:** 2026-08-12

- **Tags:** rust, dev, crate-architecture, data-oriented-api, unit-testing, bebugging, testdox

---
## Введение

Написание производственного кода на Rust требует не только знания синтаксиса и системы типов, но и глубокого понимания **архитектуры крейтов (Crates)** и стандартов инженерии надежности.

Эта статья открывает практическую серию по разработке инструментов и библиотек в Rust, созданную на основе материалов книги [**John Arundel *"The Secrets of Rust: Tools"***](https://bitfieldconsulting.com/books/rust-tools).

Мы системно разберем, как правильно разделять ответственность между библиотекой и бинарным приложением, почему публичный API обязан быть **Data-Oriented**, и как техника **Bebugging** предотвращает ошибки в юнит-тестах.

---

## 1. Архитектура Crate: разделение `src/lib.rs` и `src/main.rs`

Одно из главных правил архитекторов Rust — **разделение ответственности** между вычислениями и отображением:

1. **`src/lib.rs` (Библиотечный компонент)**: содержит всю бизнес-логику, агрегаторы данных, структуры и вычисления.
2. **`src/main.rs` (Бинарная точка входа)**: содержит минимальный код, который отвечает исключительно за чтение параметров из CLI или конфигураций и вывод готового результата пользователю.

### Главное правило Data-Oriented API
Библиотечные функции **категорически не должны** напрямую выводить данные в терминал через `println!`. Взамен они обязаны возвращать структуры данных или строки (`Data-Oriented API`).

Это гарантирует:
- **Переиспользуемость**: одну и ту же библиотеку можно подключить к CLI-утилите, Web/gRPC микросервису или GUI-интерфейсу.
- **Тестируемость**: состояние системы можно мгновенно проверить в тестах без перехвата стандартного потока вывода `stdout`.

Ниже представлен пошаговый пример проектирования библиотеки телеметрии и точки входа:


### 💻 Архитектура Crate и Data-Oriented API


#### Шаг 1

```rust
// Шаг 1: Проектирование чистой библиотеки (src/lib.rs) без побочных эффектов

#[derive(Debug, PartialEq)]
pub struct TelemetryReport {
    pub service_name: String,
    pub cpu_usage_pct: f64,
    pub active_connections: usize,
}

pub struct TelemetryAggregator {
    service_name: String,
}

impl TelemetryAggregator {
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
        }
    }

    // Главный принцип библиотеки: функция ВОЗВРАЩАЕТ структуру данных, а не выводит её в stdout через println!
    pub fn generate_report(&self, cpu_raw: u64, connections: usize) -> TelemetryReport {
        let cpu_usage_pct = (cpu_raw as f64) / 100.0;
        TelemetryReport {
            service_name: self.service_name.clone(),
            cpu_usage_pct,
            active_connections: connections,
        }
    }
}

fn main() {
    let aggregator = TelemetryAggregator::new("auth-service");
    let report = aggregator.generate_report(4550, 120);
    println!("Библиотечный модуль сгенерировал данные: {report:?}");
}
```

**1. Data-Oriented API в библиотеках**
- Публичный интерфейс библиотеки должен быть Data-Oriented: функции возвращают структуры, не вызывая `println!` внутри.
- Это обеспечивает идеальную тестируемость, возможность повторного использования в CLI, Web/gRPC и отсутствие побочных эффектов.


#### Шаг 2

```rust
// Шаг 2: Точка входа приложения (src/main.rs) отвечает за отображение

#[derive(Debug, PartialEq)]
pub struct TelemetryReport {
    pub service_name: String,
    pub cpu_usage_pct: f64,
    pub active_connections: usize,
}

pub struct TelemetryAggregator {
    service_name: String,
}

impl TelemetryAggregator {
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
        }
    }

    pub fn generate_report(&self, cpu_raw: u64, connections: usize) -> TelemetryReport {
        TelemetryReport {
            service_name: self.service_name.clone(),
            cpu_usage_pct: (cpu_raw as f64) / 100.0,
            active_connections: connections,
        }
    }
}
fn render_report_to_terminal(report: &TelemetryReport) {
    println!("=== ТЕЛЕМЕТРИЯ СЕРВИСА: {} ===", report.service_name);
    println!("Загрузка CPU: {:.2}%", report.cpu_usage_pct);
    println!("Активных соединений: {}", report.active_connections);
}

fn main() {
    // В src/main.rs логика вызова вынесена отдельно:
    let aggregator = TelemetryAggregator::new("payment-gateway");
    let report = aggregator.generate_report(8230, 450);

    // Только main() принимает решение, КАК отобразить данные пользователю:
    render_report_to_terminal(&report);
}
```

**2. Разделение ответственности (src/lib.rs и src/main.rs)**
- Библиотечный файл `src/lib.rs` отвечает за вычисления и типы данных.
- Точка входа `src/main.rs` держит минимум кода и управляет вводом/выводом в консоль или форматированием.


---

## 2. Юнит-тестирование, подробные утверждения и Bebugging

### Подмодуль `#[cfg(test)]`
Тесты в Rust размещаются в специальном подмодуле `#[cfg(test)] mod tests`. Благодаря атрибуту `#[cfg(test)]` весь код тестов и их вспомогательные структуры **полностью исключаются** из финального релизного бинарника (`cargo build --release`), не создавая никакого оверхеда.

### Информативные утверждения в `assert_eq!`
При написании утверждений не ограничивайтесь стандартным вызовом. Передавайте форматированное контекстное сообщение:

```rust
assert_eq!(
    got, want,
    "Индекс здоровья системы для load={load}: want '{want}', got '{got}'"
);
```

### Техника Bebugging (Тестирование самих тестов)
**Bebugging** — это инженерная практика намеренного внесения дефекта в реализацию функции. Если после внесения ошибки ваш тест всё равно остается «зеленым», значит тест написан неверно или имеет избыточную толерантность.

### Самодокументируемые тесты (`cargo-testdox`)
Именование функций-тестов в виде понятных полных предложений на английском языке (например `health_index_calculates_correctly_under_normal_load`) позволяет использовать сторонние утилиты вроде `cargo-testdox` для автоматической сборки спецификации требований прямо из кода.

Изучите пошаговый пример написания информативных юнит-тестов и проверки методом Bebugging:


### 💻 Юнит-тесты, Bebugging и самодокументируемый код


#### Шаг 1

```rust
// Шаг 1: Информативные утверждения в тесте с контекстными сообщениями

pub fn calculate_health_index(load: f64, errors: usize) -> f64 {
    if load > 100.0 {
        return 0.0;
    }
    let error_penalty = (errors as f64) * 2.5;
    (100.0 - load - error_penalty).max(0.0)
}

fn main() {
    let index = calculate_health_index(20.0, 2);
    println!("Вычисленный индекс здоровья системы: {index}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_index_calculates_correctly_under_normal_load() {
        let load = 20.0;
        let errors = 2;
        let want = 75.0; // 100 - 20 - 5.0
        let got = calculate_health_index(load, errors);

        // Информативное сообщение с подробными данными о несовпадении:
        assert_eq!(
            got, want,
            "Индекс здоровья рассчитан неверно для load={load}, errors={errors}: want '{want}', got '{got}'"
        );
    }
}
```

**1. Информативные юнит-тесты**
- Тесты размещаются в подмодуле `#[cfg(test)] mod tests`, который не компилируется в релизную сборку.
- Использование расширенных контекстных сообщений в `assert_eq!` позволяет мгновенно понять причину сбоя без дебаггера.


#### Шаг 2

```rust
// Шаг 2: Техника Bebugging и самодокументируемые тесты (cargo-testdox)

// В рамках техники Bebugging разработчик намеренно вносит ошибку в формулу
// для проверки того, что тесты действительно падают с понятным отчетом:
pub fn calculate_discount_price(original_price: f64, discount_pct: f64) -> f64 {
    // Намеренный баг для Bebugging: умножение вместо вычитания скидки
    // original_price * (1.0 - discount_pct / 100.0)
    original_price // ОШИБКА: вернет исходную цену без скидки!
}

fn main() {
    let price = calculate_discount_price(200.0, 10.0);
    println!("Итоговая цена: {price}");
}

#[cfg(test)]
mod bebugging_tests {
    use super::*;

    // Именование функций в стиле самодокументируемых предложений для cargo-testdox:
    #[test]
    fn discount_price_applies_percentage_deduction_properly() {
        let price = 200.0;
        let discount = 10.0;
        let want = 180.0;
        let got = calculate_discount_price(price, discount);

        assert_eq!(
            got, want,
            "Bebugging check: test successfully caught the error! want '{want}', got '{got}'"
        );
    }
}
```

**2. Bebugging и cargo-testdox**
- **Bebugging**: намеренная поломка кода для подтверждения того, что тесты чувствительны к багам.
- **cargo-testdox**: именование тестов полными предложениями генерирует спецификацию требований к системе.


---

## Проверь свои знания!

Пройдите короткий тест по архитектуре крейтов и модульному тестированию:


### 📝 Архитектура крейтов и тестирование


**1. Почему библиотечные модули (`src/lib.rs`) должны проектироваться как Data-Oriented API без прямых вызовов `println!`?**

- [ ] Возврат структур данных оставляет задачу отображения вызывающей стороне и позволяет использовать библиотеку в CLI, Web/gRPC и тестах без сайд-эффектов
- [ ] Прямой вызов `println!` блокирует компилятор на 60 секунд
- [ ] Функции внутри `src/lib.rs` не поддерживают работу с консолью

**2. В чем заключается ключевое преимущество блока `#[cfg(test)] mod tests`?**

- [ ] Тестовый код компилируется только при запуске `cargo test` и полностью исключается из релизной бинарной сборки
- [ ] Он принудительно включает многопоточный вывод данных
- [ ] Он автоматически исправляет синтаксические ошибки

**3. В чем заключается суть инженерной техники Bebugging?**

- [ ] В намеренном внесении багов в код для проверки того, что тесты действительно падают с информативными сообщениями
- [ ] В автоматическом поиске синтаксических ошибок через Miri
- [ ] В замене всех `assert_eq!` на `unwrap()`

**4. Зачем функции-тесты называют в виде полных предложений (техника `cargo-testdox`)?**

- [ ] Имена функций в виде предложений формируют понятную автодокументируемую спецификацию требований к системе
- [ ] Это требование компилятора `rustc` для релиза
- [ ] Это ускоряет сборку проекта

