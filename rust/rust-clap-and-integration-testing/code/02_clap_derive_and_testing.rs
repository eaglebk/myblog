// Шаг 1: Декларативный парсинг CLI аргументов через clap (Derive API)

use clap::Parser;

/// Высоконагруженная консольная утилита подсчета метрик файлов
#[derive(Parser, Debug, PartialEq)]
#[command(name = "stat_tool", version = "1.0", about = "Считает строки и слова")]
pub struct CliArgs {
    /// Включить режим подсчета слов вместо строк
    #[arg(short = 'w', long = "words")]
    pub words: bool,

    /// Путь к файлу для обработки
    #[arg(short = 'f', long = "file", default_value = "input.txt")]
    pub file_path: String,
}

fn main() {
    // В реальной утилите аргументы читаются вызовом CliArgs::parse()
    let mock_cli = vec!["stat_tool", "-w", "--file", "syslog.log"];
    let args = CliArgs::parse_from(mock_cli);

    println!("Парсинг CLI завершен:");
    println!("  Режим подсчета слов: {}", args.words);
    println!("  Целевой файл: {}", args.file_path);
}

// ! **1. Декларативный парсер clap**
// ! - Атрибут `#[derive(Parser)]` автоматически генерирует полный код парсинга CLI из полей структуры.
// ! - Документационные комментарии `///` автоматически становятся справочным текстом для флага `--help`.

// ---

// Шаг 2: Архитектура внешних интеграционных тестов (каталог tests/)

// Внешние интеграционные тесты создаются в папке `tests/integration_test.rs`
// Они компилируют проект и запускают готовый бинарник через утилиты `assert_cmd` и `predicates`:

pub fn run_business_pipeline(words_mode: bool, input: &str) -> String {
    if words_mode {
        format!("Слов: {}", input.split_whitespace().count())
    } else {
        format!("Строк: {}", input.lines().count())
    }
}

fn main() {
    let result = run_business_pipeline(true, "Hello Rust World");
    println!("{result}");
}

#[cfg(test)]
mod integration_mock_tests {
    use super::*;

    #[test]
    fn pipeline_counts_words_properly() {
        let input = "один два три четыре";
        let output = run_business_pipeline(true, input);
        assert!(output.contains("Слов: 4"));
    }
}

// ! **2. Интеграционное тестирование в tests/**
// ! - Модульные тесты (`#[cfg(test)]`) проверяют внутренние функции в `src/lib.rs`.
// ! - Интеграционные тесты в каталоге `tests/` тестируют бинарный файл снаружи, проверяя коды выхода и `stderr`/`stdout`.
