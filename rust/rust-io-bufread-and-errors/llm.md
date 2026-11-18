# Быстрый ввод-вывод в Rust: буферизация, тесты и ошибки

> Как сократить число системных вызовов с BufRead, тестировать потоки через std::io::Cursor, обрабатывать сбои .lines() и возвращать коды выхода в CLI.

- **URL:** https://eagle2000.space/rust/rust-io-bufread-and-errors/
- **Date:** 2026-11-18

- **Tags:** rust, dev, io, errors

---
## Введение

Стандартная библиотека Rust проектировалась так, чтобы ввод-вывод (I/O) не тратил лишние системные вызовы и аллокации на каждый байт и при этом оставался безопасным. Про устройство `BufReader` я уже рассказывал в [отдельной статье про ускорение ввода-вывода](../bufreader/), а сегодня покажу, как решать с его помощью повседневные задачи CLI.

Пока пишешь небольшие скрипты, `File::open` и `stdin().read_line()` работают без сюрпризов. Но стоит прогнать через утилиту лог на пару гигабайт, как она упирается в системные вызовы и аллокации, код на `stdin()` тяжело тестировать без временных файлов, а пайп вроде `my_tool | head -n 5` внезапно валится с паникой.

Дальше разберу, как навести здесь порядок средствами одного `std::io`.

---

## 1. Анатомия ввода-вывода в ОС и роли `Read` vs `BufRead`

### Накладные расходы системных вызовов (Syscalls)
На уровне операционной системы чтение байтов из файла или сетевого сокета выполняется через системный вызов ядра (например, `sys_read` в POSIX).

Каждый такой вызов требует **переключения контекста** (Context Switch) из пользовательского режима (User Mode) в режим ядра (Kernel Mode) и обратно. Если читать файл по одному байту напрямую через `std::fs::File`, то для файла объёмом 1 МБ программа сделает **1 048 576 системных вызовов**, и почти всё время уйдёт на походы в ядро, а не на полезную работу.

**1. Побайтовое чтение без буфера (~1 000 000 системных вызовов):**

{{< mermaid >}}
graph LR
    U1["User Code"] -- "sys_read()" --> K1["Kernel Mode"] -- "I/O" --> D1["Disk"]
{{< /mermaid >}}

**2. Чтение через BufReader (~128 системных вызовов):**

{{< mermaid >}}
graph LR
    U2["User Code"] -- "RAM Buffer (8 KB)" --> K2["Kernel Mode"] -- "I/O" --> D2["Disk"]
{{< /mermaid >}}

{{< flow-stepper src="flows/bufreader-flow.json" >}}

### Разница между `Read` и `BufRead`
В стандартной библиотеке Rust абстракции I/O разделены на два ключевых типажа:
1. **`std::io::Read`**: базовый трейт для чтения байтов в буфер с помощью метода `.read(&mut [u8])`. Не гарантирует наличия внутреннего буфера.
2. **`std::io::BufRead`**: расширенный трейт для типов, обладающих **внутренним промежуточным буфером в оперативной памяти** (по умолчанию 8 КБ в `BufReader<R>`). Он предоставляет высокоуровневые методы `.read_line()`, `.lines()`, `.read_until()` и `.fill_buf()`.

---

## 2. Подход «Магическая функция» и тестируемость с `impl BufRead`

### Идея «обратной разработки» (Magic Function)
Я познакомился с этим приёмом в [отличной книге «The Secrets of Rust: Tools»](https://bitfieldconsulting.com/books/rust-tools) и после первых же глав пересмотрел свой подход к CLI. Вот как это работает на практике. Вы проектируете код с конца: сначала пишете в `main()` идеальный вызов, словно нужная функция с идеальной сигнатурой уже существует:

```rust
// Придуманный идеальный вызов в main():
let metrics = analyze_log_stream(input)?;
```

Только после этого вы подбираете минимально нужные типажи для аргумента `input`.

### Опасность жёсткой привязки к конкретным типам
Если написать функцию, принимающую конкретный `&mut File` или `StdinLock`:

```rust
// Плохой подход: жёсткая привязка к файлам
pub fn analyze_file(file: &mut std::fs::File) -> std::io::Result<usize> { ... }
```

то написать для неё быстрый юнит-тест станет проблемой: придётся создавать временный файл на диске, удалять его после теста и бороться с конфликтами при параллельном запуске тестов.

### Универсальное решение: `impl BufRead` и `std::io::Cursor`
Принимая обобщённый параметр `impl BufRead`, ваша функция перестаёт зависеть от источника данных:

- В **реальном коде** она принимает `std::fs::File` (обёрнутый в `BufReader` - для больших файлов можно увеличить буфер через `BufReader::with_capacity(64 * 1024, file)`) или заблокированный `stdin().lock()`.
- В **юнит-тестах** ей передаётся `std::io::Cursor::new("строка 1\nстрока 2")` - поток прямо из среза байтов или строки в памяти без обращения к диску.

{{< alert "comment" >}}
**Что такое `std::io::Cursor`?** Если коротко, то это обёртка над обычными данными в памяти (`&str`, `&[u8]` или `Vec<u8>`), которая хранит внутри числовую позицию (курсор чтения/записи). Благодаря этому буфер в RAM начинает вести себя как открытый файл: для `Cursor` реализованы типажи `Read`, `BufRead`, `Seek` и `Write` (для мутабельных буферов). При каждом `.read()` или `.read_line()` курсор просто сдвигает внутренний индекс вперёд, не обращаясь к диску.
{{< /alert >}}

Вот пошаговый пример `impl BufRead`, тестирования через `Cursor` и применения `stdin().lock()`:


### 💻 BufRead, Cursor в тестах и stdin().lock()


#### Шаг 1

```rust
// Шаг 1: Подход Magic Function и идеальная тестируемость с `impl BufRead` и `Cursor`

use std::io::{BufRead, Cursor, Result};

pub struct LogMetrics {
    pub total_lines: usize,
    pub error_lines: usize,
}

// Принимаем `impl BufRead` вместо жёстко привязанного `StdinLock` или `BufReader<File>`:
pub fn analyze_log_stream(input: impl BufRead) -> Result<LogMetrics> {
    let mut total_lines = 0;
    let mut error_lines = 0;

    for line in input.lines() {
        let text = line?; // Обработка возможных ошибок чтения каждого блока
        total_lines += 1;
        if text.contains("[ERROR]") || text.contains("[FATAL]") {
            error_lines += 1;
        }
    }

    Ok(LogMetrics {
        total_lines,
        error_lines,
    })
}

fn main() {
    // В тестах поток ввода подменяется через std::io::Cursor без обращения к диску:
    let fake_log_data = "[INFO] System start\n[ERROR] Connection timeout\n[INFO] Retrying\n";
    let cursor = Cursor::new(fake_log_data);

    let metrics = analyze_log_stream(cursor).expect("Ошибка анализа лог-потока");
    assert_eq!(metrics.total_lines, 3);
    assert_eq!(metrics.error_lines, 1);
    println!(
        "Проанализировано строк: {}, ошибок найдено: {}",
        metrics.total_lines, metrics.error_lines
    );
}
```

**1. Подход Magic Function и BufRead**
- Сначала проектируем функцию, подбирая минимально требуемый и подходящий типаж `impl BufRead`.
- Использование `std::io::Cursor` позволяет тестировать потоки чтения прямо в оперативной памяти без обращения к файловой системе.


#### Шаг 2

```rust
// Шаг 2: Эффективная блокировка stdin().lock() для высокопроизводительного ввода
// ?ignore

use std::io::{self, stdin, BufRead};

fn process_user_stream(input: impl BufRead) -> io::Result<usize> {
    let mut count = 0;
    for line in input.lines() {
        let _ = line?; // Явно пробрасываем ошибки чтения и UTF-8, не теряя их молча
        count += 1;
    }
    Ok(count)
}

fn main() -> io::Result<()> {
    // Вызов stdin().lock() блокирует стандартный ввод один раз на всё время работы,
    // избавляя от повторных захватов Mutex при чтении каждой строки:
    let locked_stdin = stdin().lock();

    // Передаём заблокированный stdin в ту же универсальную функцию:
    let count = process_user_stream(locked_stdin)?;
    println!("Получено строк из стандартного ввода: {count}");
    Ok(())
}
```

**2. Захват блокировки через `stdin().lock()`**
- Стандартный `stdin()` защищен внутренним Mutex. Повторный вызов `.read_line()` на `stdin()` захватывает lock каждый раз.
- Метод `stdin().lock()` возвращает `StdinLock`, который реализует `BufRead` и кардинально ускоряет чтение.


---

## 3. Секреты `stdin().lock()` и внутренний мьютекс

### Как устроен `std::io::stdin()` под капотом?
Стандартный ввод `stdin` - общий ресурс для всех потоков программы, к которому они могут обращаться одновременно. Чтобы избежать повреждения данных и состояния гонки, реализация `std::io::Stdin` в стандартной библиотеке Rust защищена глобальным мьютексом. Но не простым, а `ReentrantMutex`.

{{< alert "comment" >}}
**Зачем здесь `ReentrantMutex` и чем он отличается от обычного `Mutex`?** Обычный `std::sync::Mutex` при повторном вызове `.lock()` в том же потоке приводит к вечному зависанию (deadlock): поток начинает ждать сам себя. Реентерабельный мьютекс (`ReentrantMutex` / `ReentrantLock` во внутренностях `std`) запоминает поток-владелец и счётчик вложенных блокировок. Если поток уже держит `stdin().lock()` и в глубине стека вызовет `stdin()` ещё раз, программа не зависнет - просто увеличится счётчик, а другие потоки по-прежнему будут ждать освобождения всех гардов.

Из-за того что в одном потоке можно получить несколько одновременных блокировок, такой мьютекс выдаёт только разделяемую ссылку `&T` вместо `&mut T` (внутри `Stdin` мутабельность обеспечивается через `RefCell` - проверку правил заимствования в рантайме), поэтому в прикладном коде обычно применяют стандартный `Mutex`.
{{< /alert >}}

Когда вы вызываете методы прямо на объекте `stdin()` без предварительной блокировки:

```rust
// Медленный вариант:
let mut input = std::io::stdin();
for _ in 0..1000 {
    let mut line = String::new();
    input.read_line(&mut line)?; // Захват и освобождение Mutex на КАЖДОМ шаге!
}
```

на каждой итерации цикла программа:
1. Захватывает мьютекс `stdin`.
2. Читает одну строку.
3. Освобождает мьютекс `stdin`.

### Решение: однократный захват через `.lock()` и переиспользование буфера
Метод `stdin().lock()` один раз закрепляет стандартный ввод за текущим потоком (`thread`) и возвращает обёртку `StdinLock<'static>`, которая реализует типаж `BufRead`. Если вдобавок переиспользовать один буфер `String` через `buf.clear()`, убираются и повторные блокировки мьютекса, и лишние аллокации памяти:

{{< code-before-after before_label="Медленно (lock + alloc в цикле)" after_label="Быстро (stdin.lock + buf.clear)" badge="0 extra locks" caption="Сравнение: один вызов stdin().lock() перед циклом и переиспользование буфера убирают лишние блокировки и аллокации" >}}
```rust
let input = std::io::stdin();
for _ in 0..1000 {
    let mut line = String::new();
    input.read_line(&mut line)?;
    process(&line);
}
```
===
```rust
let stdin = std::io::stdin();
let mut handle = stdin.lock();
let mut line = String::new();
while handle.read_line(&mut line)? > 0 {
    process(&line);
    line.clear();
}
```
===
Блокировка ;; let input = std::io::stdin(); ;; let mut handle = stdin.lock(); ;; Вместо захвата и освобождения мьютекса на каждой итерации блокируем stdin один раз до цикла.
Аллокация ;; let mut line = String::new(); ;; let mut line = String::new();||line.clear(); ;; Выделяем память под буфер String один раз перед циклом и сбрасываем текущую длину строки (len = 0) через line.clear(), сохраняя выделенную в куче ёмкость (capacity).
Чтение строки ;; input.read_line(&mut line)?; ;; handle.read_line(&mut line)? ;; Заблокированный StdinLock реализует BufRead и читает строки напрямую из буфера без лишней синхронизации.
{{< /code-before-after >}}

В результате накладные расходы на синхронизацию падают до нуля, а чтение ускоряется в разы.

---

## 4. Обработка ошибок в `.lines()` и интеграция с UNIX Shell

### Почему `.lines()` возвращает `Result<String>`?
В отличие от итерирования по коллекциям в памяти (например, `Vec<T>`), чтение из потока I/O может сорваться в любой момент:
- сбой диска, отключение накопителя или обрыв сетевого сокета (`EIO` / `ErrorKind::ConnectionReset`);
- нехватка прав на чтение файла (`ErrorKind::PermissionDenied`);
- невалидная байтовая последовательность UTF-8 во входном потоке (`ErrorKind::InvalidData`).

*(Примечание: прерывание системного вызова сигналом `EINTR` / `ErrorKind::Interrupted` стандартная библиотека внутри `BufRead::read_until` повторяет автоматически, поэтому из `.lines()` оно наружу не выходит.)*

Именно поэтому элемент итератора `.lines()` имеет тип `Result<String, std::io::Error>`. Каждую полученную строку обязательно проверяют оператором `?` или `match`.

### Оптимизация памяти: `lines()` vs `read_line()`
Как видно из сравнения в предыдущем разделе, разница в аллокациях огромна:
- **`reader.lines()`**: на каждой итерации создаёт **новую аллокацию `String`** (10 000 000 строк = 10 000 000 вызовов аллокатора).
- **`reader.read_line(&mut buf)`**: переиспользует **один и тот же буфер `String`** через `buf.clear()`, сводя число аллокаций к минимуму.

> [!WARNING]
> **Подвох при замене `.lines()` на `read_line()`:** итератор `.lines()` автоматически отрезает `\n` и `\r\n` в конце строки, а `read_line(&mut buf)` **оставляет** их внутри буфера. Перед парсингом не забывайте срезать перевод строки через `buf.trim_end_matches(&['\r', '\n'])`.
>
> Для необработанных системных логов, где может попасться битый UTF-8, вместо `read_line` надёжнее читать байты через `reader.read_until(b'\n', &mut byte_buf)` и декодировать их через `String::from_utf8_lossy`. А если файл приходит извне и в нём может не оказаться `\n` (например, гигабайтный файл в одну строку), ограничивайте чтение через `reader.by_ref().take(MAX_LINE_LEN)`, чтобы не упереться в нехватку памяти (Out of Memory, OOM).

### Разделение `stdout` / `stderr` и `ExitCode` вместо `process::exit`
Консольные утилиты часто запускают внутри UNIX-конвейеров (Unix Pipes):

```bash
cat server.log | my_rust_tool | grep "CRITICAL" > result.txt
```

Чтобы такая цепочка не сломалась, нужно соблюдать два правила:

1. **Ошибки пишутся строго в `stderr` через `eprintln!`**: если выводить ошибки через обычный `println!`, сообщения об ошибках попадут в общий поток `stdout` и окажутся внутри файла `result.txt`, испортив данные.
2. **Системные коды завершения (`std::process::ExitCode`)**:
   - `0` (`ExitCode::SUCCESS`) - программа завершилась успешно.
   - `1` (`ExitCode::FAILURE`) - общая ошибка выполнения программы или работы с данными.
   - `2` (`ExitCode::from(2)`) - некорректное использование утилиты или неверные CLI-аргументы.
   - В продакшен-утилитах лучше возвращать `std::process::ExitCode` прямо из `main()`, а не вызывать `std::process::exit(code)`. Вызов `process::exit` обрывает процесс мгновенно и **не запускает деструкторы (`Drop`)** - из-за этого не сбрасываются на диск буферы `BufWriter<File>` и не удаляются временные файлы.

### Буферизованный вывод и `BrokenPipe` как в `ripgrep` и `bat`
Обычный `println!` в цикле создаёт сразу две проблемы: под капотом `std::io::stdout()` использует `LineWriter` (блокирует `stdout` и сбрасывает буфер на каждом `\n`), а внутри макроса вызывается `.unwrap()`. При досрочном закрытии пайпа - например, в `my_tool big.log | head -n 5` - Rust переводит сигнал `SIGPIPE` (`SIG_IGN`) в ошибку `ErrorKind::BrokenPipe`, и `println!` **паникует** со стектрейсом.

Поэтому на практике `stdout().lock()` оборачивают в `BufWriter`, пишут через `writeln!`, явно вызывают `out.flush()?` (так как ошибки автосброса в `Drop` игнорируются) и обрабатывают `ErrorKind::BrokenPipe` на уровне возврата из `try_main()` с кодом `ExitCode::SUCCESS`. В CI/CD-скриптах под `set -o pipefail` это особенно важно: утилита завершается чисто с кодом `0` вместо паники или сигнала `141` - так устроены [`BurntSushi/ripgrep`](https://github.com/BurntSushi/ripgrep/blob/master/crates/core/main.rs) ([issue #200](https://github.com/BurntSushi/ripgrep/issues/200)) и `sharkdp/bat`.

А чтобы в одном CLI принимать и путь к файлу, и `-`/пайп из `stdin`, на границе программы возвращают трейт-объект `Box<dyn BufRead>`: в ветках `match` типы `BufReader<File>` и `StdinLock` разные, поэтому `impl BufRead` не скомпилируется, а внутри бизнес-логики по-прежнему остаётся `impl BufRead`.

Ниже - пример обработки ошибок `.lines()` и `ExitCode`:


### 💻 Обработка ошибок I/O, stderr и ExitCode


#### Шаг 1

```rust
// Шаг 1: Ранний возврат ошибок через оператор `?` в `.lines()`

use std::io::{self, BufRead, Cursor, ErrorKind, Result};

pub fn parse_config_values(reader: impl BufRead) -> Result<Vec<u32>> {
    let mut values = Vec::new();

    for line_result in reader.lines() {
        // Оператор `?` моментально прекращает выполнение при сбое чтения или битом UTF-8:
        let line = line_result?;
        let trimmed = line.trim();

        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            let num = trimmed
                .parse::<u32>()
                .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
            values.push(num);
        }
    }

    Ok(values)
}

fn main() {
    let raw_config = "100\n# comment\n200\n300\n";
    let cursor = Cursor::new(raw_config);

    match parse_config_values(cursor) {
        Ok(parsed) => println!("Разобраны числа: {:?}", parsed),
        Err(err) => eprintln!("Ошибка при чтении: {err}"),
    }
}
```

**1. Безопасный обход `.lines()`**
- Метод `.lines()` возвращает `Result<String>`, а не сырую строку `String`, так как чтение потока может сорваться в любой момент.
- Оператор `?` обеспечивает безопасный проброс ошибки наверх без паники приложения.


#### Шаг 2

```rust
// Шаг 2: Системный поток ошибок (`stderr`) и `ExitCode` вместо `process::exit`

use std::io::{self, BufRead, Cursor, ErrorKind, Result};

pub fn parse_config_values(reader: impl BufRead) -> Result<Vec<u32>> {
    let mut values = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            let num = trimmed
                .parse::<u32>()
                .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;
            values.push(num);
        }
    }
    Ok(values)
}
use std::process::ExitCode;

fn run_cli_pipeline() -> Result<()> {
    let sample = Cursor::new("42\n84\n");
    let results = parse_config_values(sample)?;
    println!("Обработано элементов: {}", results.len());
    Ok(())
}

fn main() -> ExitCode {
    // Ошибки всегда пишутся в `stderr` через `eprintln!`,
    // а из `main()` возвращается `ExitCode`, чтобы отработали все деструкторы `Drop`:
    match run_cli_pipeline() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Критическая ошибка выполнения утилиты: {err}");
            ExitCode::FAILURE
        }
    }
}
```

**2. Разделение `stdout`/`stderr` и возврат `ExitCode`**
- Вывод ошибок в `eprintln!` защищает пайплайны Shell (`cat data | my_tool > output.txt`): ошибки не попадут в результирующий файл.
- Возврат `ExitCode::FAILURE` из `main()` сигнализирует о сбое скриптам и CI/CD, но при этом корректно сбрасывает буферы и вызывает `Drop`.



### 💻 Продакшен-паттерны CLI: файл/stdin, BufWriter и BrokenPipe


#### Шаг 1

```rust
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
```

**1. Единый вход как в `bat` и `xsv`**
- `Box<dyn BufRead>` скрывает разницу между `BufReader<File>` и `StdinLock`.
- `read_line` оставляет `\n` в конце строки, поэтому перед проверкой обрезаем его через `trim_end_matches`.
- `StdinLock` не реализует `Send` (и держит блокировку `stdin` на время жизни `Box<dyn BufRead>`): для передачи ридера в другой поток используют `io::stdin()` и `Box<dyn BufRead + Send>`.


#### Шаг 2

```rust
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
```

**2. Буферизованный вывод вместо `println!` в цикле**
- `stdout().lock()` захватывает мьютекс один раз, а `BufWriter` копит строки в памяти перед системным вызовом.
- `out.flush()?` в конце обязателен: ошибки при автоматическом сбросе в `Drop` игнорируются.


#### Шаг 3

```rust
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
```

**3. Работа с UNIX-пайпами и `ExitCode`**
- Возврат `ExitCode` из `main()` корректно вызывает `Drop` для всех открытых файлов и буферов (в отличие от `process::exit`).
- При `ErrorKind::BrokenPipe` утилита завершается с `ExitCode::SUCCESS`, как это делают `ripgrep` и `bat`.


---

## Заключение

Три правила для надёжного ввода-вывода в Rust:
1. **Эффективность**: `BufRead`, `stdin().lock()` и `BufWriter<StdoutLock>` убирают лишние системные вызовы и повторные захваты мьютексов.
2. **Тестируемость**: параметр `impl BufRead` (и `Box<dyn BufRead>` на границе CLI) позволяет проверять I/O-логику в памяти через `std::io::Cursor` без временных файлов.
3. **Совместимость с UNIX**: вывод ошибок в `eprintln!`, возврат `ExitCode` из `main()` и корректная обработка `BrokenPipe` встраивают утилиту в любые скрипты и CI-пайплайны.

---

## Проверь свои знания!

Пройдите короткий интерактивный тест по работе с I/O в Rust:


### 📝 Потоки ввода-вывода и обработка ошибок


**1. Зачем функциям потоковой обработки принимать обобщённый параметр `impl BufRead` вместо `&mut File` или `StdinLock`?**

- [ ] Чтобы компилятор автоматически сжал входной буфер в памяти
- [ ] Чтобы не зависеть от источника данных и легко тестировать функцию через `std::io::Cursor` в памяти
- [ ] Чтобы отказаться от обработки ошибок при чтении из файла

**2. Что даёт предварительный вызов `std::io::stdin().lock()` перед циклом чтения?**

- [ ] Автоматически преобразует входные байты в `Vec<u8>` без проверки UTF-8
- [ ] Отключает буферизацию ядра, чтобы читать строки быстрее
- [ ] Один раз захватывает мьютекс стандартного ввода, убирая повторную синхронизацию на каждой строке

**3. Почему итератор `.lines()` возвращает `Result<String>` вместо `String` на каждом шаге?**

- [ ] Чтение из внешнего потока может сорваться в любой момент - например, из-за сбоя диска или невалидного UTF-8
- [ ] Потому что `BufRead` всегда читает файл в обратном порядке
- [ ] Чтобы компилятор мог распараллелить цикл по потокам

**4. Почему в CLI-утилитах ошибки выводят через `eprintln!`, а код завершения возвращают через `std::process::ExitCode` из `main()` вместо `process::exit(1)`?**

- [ ] Потому что `process::exit(1)` нельзя вызывать внутри блока `match`
- [ ] Ошибки уходят в `stderr`, не портя `stdout`-пайпы, а возврат `ExitCode` корректно вызывает деструкторы `Drop` и сбрасывает буферы
- [ ] Потому что `eprintln!` автоматически завершает процесс с кодом `1`

**5. О чём важно помнить при замене `.lines()` на цикл с `read_line(&mut buf)` и `buf.clear()`?**

- [ ] `read_line()` сам удаляет все пробелы в начале и конце строки
- [ ] `read_line()` игнорирует ошибки невалидного UTF-8
- [ ] `read_line()` сохраняет завершающий `\n` / `\r\n` в конце строки, поэтому перед разбором его нужно обрезать

**6. Зачем в конце вывода через `BufWriter<StdoutLock>` вызывают `out.flush()?` явно и как правильно реагировать на `ErrorKind::BrokenPipe`?**

- [ ] Чтобы обнулить счётчик `StdinLock`, а при `BrokenPipe` - завершить программу паникой
- [ ] Ошибки автосброса в `Drop` игнорируются, поэтому `flush()?` нужен для их отлова; при `BrokenPipe` (например, `| head -n 5`) программа должна завершиться с `ExitCode::SUCCESS` без паники
- [ ] Явный `flush()` очищает экран терминала, а `BrokenPipe` возникает только при переполнении диска

