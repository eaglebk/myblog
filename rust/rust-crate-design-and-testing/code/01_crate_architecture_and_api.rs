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

// ! **1. Data-Oriented API в библиотеках**
// ! - Публичный интерфейс библиотеки должен быть Data-Oriented: функции возвращают структуры, не вызывая `println!` внутри.
// ! - Это обеспечивает идеальную тестируемость, возможность повторного использования в CLI, Web/gRPC и отсутствие побочных эффектов.

// ---

// Шаг 2: Точка входа приложения (src/main.rs) отвечает за отображение

// ?hidden:start
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
// ?hidden:end

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

// ! **2. Разделение ответственности (src/lib.rs и src/main.rs)**
// ! - Библиотечный файл `src/lib.rs` отвечает за вычисления и типы данных.
// ! - Точка входа `src/main.rs` держит минимум кода и управляет вводом/выводом в консоль или форматированием.
