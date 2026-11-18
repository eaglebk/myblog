// Шаг 1: Один интерфейс для файла, stdin и тестов (Box<dyn BufRead>)

use std::fs::File;
use std::io::{self, BufRead, BufReader, Cursor};

// На границе CLI выбираем источник: "-" или None -> stdin, иначе файл.
#[allow(dead_code)]
fn open_input(path: Option<&str>) -> io::Result<Box<dyn BufRead>> {
    match path {
        Some("-") | None => Ok(Box::new(io::stdin().lock())),
        Some(p) => Ok(Box::new(BufReader::new(File::open(p)?))),
    }
}

// Вся бизнес-логика остаётся простой: принимает любой impl BufRead.
fn count_matching(mut reader: impl BufRead, pattern: &str) -> io::Result<usize> {
    let mut count = 0;
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        let clean = line.trim_end_matches(&['\r', '\n']);
        if clean.contains(pattern) {
            count += 1;
        }
        line.clear();
    }
    Ok(count)
}

fn main() -> io::Result<()> {
    // В тесте или Playground подменяем файл/stdin обычным Cursor в памяти:
    let fake_input = Cursor::new("INFO ok\nERROR disk full\nERROR timeout\n");
    let errors = count_matching(fake_input, "ERROR")?;
    assert_eq!(errors, 2);
    println!("Найдено ошибок: {errors}");

    // В реальном CLI вызов выглядел бы так:
    // let reader = open_input(std::env::args().nth(1).as_deref())?;
    Ok(())
}

// ! **1. Единый вход как в `bat` и `xsv`**
// ! - `Box<dyn BufRead>` скрывает разницу между `BufReader<File>` и `StdinLock`.
// ! - `read_line` оставляет `\n` в конце строки, поэтому перед проверкой обрезаем его через `trim_end_matches`.
// ! - `StdinLock` не реализует `Send` (и держит блокировку `stdin` на время жизни `Box<dyn BufRead>`): для передачи ридера в другой поток используют `io::stdin()` и `Box<dyn BufRead + Send>`.

// ---

// Шаг 2: Быстрый вывод через BufWriter<StdoutLock> и переиспользование буфера

use std::io::{self, BufRead, BufWriter, Cursor, Write};

fn filter_logs(mut reader: impl BufRead, mut out: impl Write, pattern: &str) -> io::Result<()> {
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        let clean = line.trim_end_matches(&['\r', '\n']);
        if clean.contains(pattern) {
            // Пишем в буфер, а не напрямую через println! (нет лишних flush и блокировок)
            writeln!(out, "{clean}")?;
        }
        line.clear();
    }

    // Явный flush сбрасывает остаток буфера и позволяет поймать ошибку записи
    out.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let input = Cursor::new("INFO start\nERROR db\nINFO done\n");
    let stdout = io::stdout();
    let out = BufWriter::new(stdout.lock());

    filter_logs(input, out, "ERROR")?;
    Ok(())
}

// ! **2. Буферизованный вывод вместо `println!` в цикле**
// ! - `stdout().lock()` захватывает мьютекс один раз, а `BufWriter` копит строки в памяти перед системным вызовом.
// ! - `out.flush()?` в конце обязателен: ошибки при автоматическом сбросе в `Drop` игнорируются.

// ---

// Шаг 3: Завершение через ExitCode и корректная обработка BrokenPipe (как в ripgrep)

use std::io::{self, BufWriter, ErrorKind, Write};
use std::process::ExitCode;

fn run_stream(mut out: impl Write) -> io::Result<()> {
    for i in 0..5 {
        writeln!(out, "Строка лога #{i}")?;
    }
    out.flush()?;
    Ok(())
}

fn handle_result(res: io::Result<()>) -> ExitCode {
    match res {
        Ok(()) => ExitCode::SUCCESS,
        // Если пайп закрыли раньше времени (например, `my_tool | head -n 5`), выходим чисто с кодом 0:
        Err(err) if err.kind() == ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Ошибка выполнения: {err}");
            ExitCode::FAILURE
        }
    }
}

fn main() -> ExitCode {
    let stdout = io::stdout();
    let out = BufWriter::new(stdout.lock());

    // Проверяем обработку BrokenPipe и запускаем основной поток вывода:
    assert_eq!(
        handle_result(Err(io::Error::new(ErrorKind::BrokenPipe, "pipe closed"))),
        ExitCode::SUCCESS
    );
    println!("BrokenPipe обработан чисто (ExitCode::SUCCESS)");
    handle_result(run_stream(out))
}

// ! **3. Работа с UNIX-пайпами и `ExitCode`**
// ! - Возврат `ExitCode` из `main()` корректно вызывает `Drop` для всех открытых файлов и буферов (в отличие от `process::exit`).
// ! - При `ErrorKind::BrokenPipe` утилита завершается с `ExitCode::SUCCESS`, как это делают `ripgrep` и `bat`.

