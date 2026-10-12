# BufReader в Rust: ускоряем ввод-вывод

> BufReader в Rust - подробный разбор. Как ускорить чтение файлов, работать с вводом-выводом, использовать BufRead и Read. Примеры построчного чтения и оптимизации IO.

- **URL:** http://localhost:1314/rust/bufreader/
- **Date:** 2025-08-26

- **Tags:** rust, dev

---В этой статье разберём зачем нужен `BufReader`, как он работает и какие есть способы читать данные из файлов. Мы сравним разные подходы и посмотрим сколько они занимают времени.

> 🎙️ К статье прилагается подкаст-версия - лёгкий и образный рассказ с аналогиями. Включайте, если хотите услышать объяснение на слух.
{{< audio src="/audio/Rust_details_1.mp3" caption="Подкаст Rust в деталях. Выпуск 1" >}}

## Трейты и чтение данных

В Rust **трейты** описывают общее поведение. Если тип реализует трейт - он "умеет" делать определённые действия.

Например, трейт `Read` говорит: "этот объект можно читать". Под него попадают файлы (`File`), сетевые соединения (`TcpStream`), стандартный ввод (`stdin`).

Главный метод:

```rust
fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
````

Он записывает доступные байты в буфер и возвращает число реально прочитанных. Чтобы загрузить все данные - `.read()` вызывают многократно.

## Системные вызовы и их цена

Когда мы читаем файл или сокет через `Read`, каждый вызов `.read()` почти всегда приводит к системному вызову `read(2)` (на Linux/Unix) или его аналогу в других ОС.

⚙️ Разберём по шагам что же происходит когда в коде вызвается этот метод:
1. Программа вызывает метод .read().
2. Процессор переключается из пользовательского режима [(user mode)](https://ru.wikipedia.org/wiki/User-mode_Linux) в режим ядра [(kernel mode)](https://ru.wikipedia.org/wiki/%D0%A0%D0%B5%D0%B6%D0%B8%D0%BC_%D1%8F%D0%B4%D1%80%D0%B0).
3. ОС выполняет реальную операцию чтения: обращается к диску, сети или памяти.
4. Управление возвращается в пользовательское приложение.

Каждый такой переход - это сотни инструкций процессора. Поэтому чтение по 1 байту с диска будет катастрофически медленным.

## BufReader

Чтобы уменьшить число системных вызовов существует **BufReader**.

Он один раз считывает крупный блок (по умолчанию 8 КБ) во внутренний буфер, а дальше выдаёт данные уже из памяти. Так вместо тысячи вызовов вы получаете десятки.


### 💻 BufReader: чтение всего файла


#### Шаг 1

```rust
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Read, Write};
use std::path::Path;

fn ensure_file() -> io::Result<()> {
    if !Path::new("hello.txt").exists() {
        let mut f = OpenOptions::new().create(true).write(true).open("hello.txt")?;
        writeln!(f, "Знакомство с BufReader!")?;
    }
    Ok(())
}
fn main() -> io::Result<()> {
    // Проверяем, что файл существует (создаём при необходимости)
    ensure_file()?;
    
    // Открываем файл
    let file = File::open("hello.txt")?;
    
    // Оборачиваем его в BufReader
    let mut reader = BufReader::new(file);

    // Читаем содержимое в строку
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    println!("Содержимое файла: {}", contents);

    Ok(())
}
```

**Разберем подробнее:**
1. Функция `ensure_file` проверяет наличие файла `hello.txt`. 
Если его нет — создаём и записываем туда строку "Знакомство с BufReader!".
2. В `main` открываем файл и оборачиваем его в `BufReader`. 
Буферизованное чтение делает работу эффективной.
3. С помощью `read_to_string` считываем всё содержимое в строку.
4. Результат печатаем в консоль.
Это самый простой пример: мы просто читаем файл целиком.
Но ключевая идея уже видна - `BufReader` используется как оболочка
вокруг файла, которая делает операции ввода-вывода более эффективными.


#### Шаг 2

```rust
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufRead, Write};
use std::path::Path;


fn ensure_file() -> io::Result<()> {
    if !Path::new("app.log").exists() {
        let mut f = OpenOptions::new().create(true).write(true).open("app.log")?;
        writeln!(f, "[2025-08-27 10:00:01] INFO  — Приложение запущено")?;
        writeln!(f, "[2025-08-27 10:00:03] WARN  — Соединение нестабильно")?;
        writeln!(f, "[2025-08-27 10:00:05] ERROR — Ошибка чтения конфигурации")?;
        writeln!(f, "[2025-08-27 10:00:07] INFO  — Перезапуск сервиса")?;
    }
    Ok(())
}


fn main() -> io::Result<()> {
    ensure_file()?;

    let file = File::open("app.log")?;
    let reader = BufReader::new(file);

    let mut line_count = 0;
    let mut error_count = 0;

    for line in reader.lines() {
        let line = line?;
        line_count += 1;
        if line.contains("ERROR") {
            error_count += 1;
        }
        println!(">> {}", line);
    }

    println!("\nВ логе {} строки. Из них {} с ошибками.", line_count, error_count);

    Ok(())
}
```

**Разберем подробнее что же тут происходит.**
_Мы читаем лог-файл с помощью `BufReader`:_
1. Функция `ensure_file` имитирует создание файла с логом приложения. Если файла `app.log` нет — создаём его и записываем несколько строк с метками времени, уровнями логов (`INFO`, `WARN`, `ERROR`) и сообщениями.
2. В `main` открываем файл и оборачиваем его в `BufReader` чтобы читать эффективно.
3. С помощью `reader.lines()` проходим построчно в цикле:
- считаем общее количество строк
- ищем вхождения слова `"ERROR"` и увеличиваем отдельный счётчик ошибок
- печатаем строку с префиксом `>>`
4. После цикла выводим статистику: сколько строк всего и сколько среди них сообщений об ошибках.
Такой приём реально полезен: BufReader позволяет работать даже с огромными логами построчно,
без необходимости загружать их целиком в память.


## BufRead против Read

BufReader реализует трейт `BufRead`, который расширяет возможности `Read`.

Зачем нужен отдельный трейт?
Потому что некоторые операции становятся эффективными только при наличии внутреннего буфера. 

Например, метод `.lines`() возвращает итератор по строкам:


### 💻 BufReader: построчное чтение


```rust
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufRead, Write};

// Создадим файл
fn ensure_file() -> io::Result<()> {
    if !std::path::Path::new("data.txt").exists() {
        let mut f = OpenOptions::new().create(true).write(true).open("data.txt")?;
        writeln!(f, "Первая строка")?;
        writeln!(f, "Вторая строка")?;
        writeln!(f, "Третья строка")?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    ensure_file()?;

    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
```

**Разберем подробнее:**
1. BufReader читает файл большими блоками и хранит их в своём буфере.
2. Трейт BufRead добавляет методы, которые работают не с файловым дескриптором напрямую, а с этим буфером в памяти.
3. Метод .lines() ищет символ перевода строки \n в буфере и возвращает готовые строки.
Если бы у нас был только Read, то для поиска конца строки пришлось бы каждый раз вызывать .read() → это означало бы больше системных вызовов и ненужных копирований.



### Другие полезные методы BufRead:

- `.read_line(&mut String)` — читает одну строку целиком в переданную строку.
- `.split(u8)` — возвращает итератор, разделяющий поток по заданному байту.
- `.fill_buf()` — даёт прямой доступ к текущему содержимому буфера (без копирования).
- `.consume(n)` — указывает сколько байт "съесть" из буфера после обработки.

## Четыре способа читать файл построчно

Чтобы увидеть разницу, сравним четыре подхода.



### 💻 Примеры для 4 подходов


#### Шаг 1

```rust
use std::fs::{self, OpenOptions};
use std::io::{self, Write, Read};
use std::path::Path;
use std::fs::File;


/// Создаём файл big.txt, если его ещё нет.
/// 100_0 строк с текстом и номерами.
fn ensure_file() -> io::Result<()> {
    if Path::new("big.txt").exists() {
        return Ok(());
    }
    println!("Создается файл big.txt...");
    let mut f = OpenOptions::new().create(true).write(true).open("big.txt")?;
    for i in 0..100_0 {
        writeln!(f, "Строка номер {i:06} — пример содержимого файла для BufReader")?;
    }
    println!("Файл создан!");
    Ok(())
}
fn main() -> io::Result<()> {
    ensure_file()?; // создаём файл

    let start = std::time::Instant::now();
    let mut file = File::open("big.txt")?;
    let mut buffer = [0; 1];
    let mut line = String::new();

    while file.read(&mut buffer)? > 0 {
        let ch = buffer[0] as char;
        if ch == '\n' {
            line.clear();
        } else {
            line.push(ch);
        }
    }
    let end = std::time::Instant::now();
    println!("Время выполнения: {:?}", end - start);

    Ok(())
}
```

1. **Небуферизованное чтение по одному символу**
❌ Очень медленно {{< badge >}} ~123 миллисекунды {{< /badge >}}
Здесь каждая операция read идёт напрямую к ОС.
Удобно для понимания, но не для реальной работы.


#### Шаг 2

```rust
use std::fs::{self, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;
use std::fs::File;

/// Создаём файл big.txt, если его ещё нет.
/// 100_0 строк с текстом и номерами.
fn ensure_file() -> io::Result<()> {
    if Path::new("big.txt").exists() {
        return Ok(());
    }
    println!("Создается файл big.txt...");
    let mut f = OpenOptions::new().create(true).write(true).open("big.txt")?;
    for i in 0..100_0 {
        writeln!(f, "Строка номер {i:06}")?;
    }
    println!("Файл создан!");
    Ok(())
}
fn main() -> io::Result<()> {
    ensure_file()?; // создаём файл
    let start = std::time::Instant::now();
    let mut lines_count = 0;

    let file = File::open("big.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        lines_count += 1;
    }

    let end = std::time::Instant::now();
    println!("Время выполнения: {:?}", end - start);

    Ok(())
}
```

2. **BufReader + .lines()**
⚡ Гораздо быстрее {{< badge >}} ~500µs {{< /badge >}}
Это стандартный способ, удобный и быстрый.


#### Шаг 3

```rust
use std::fs::{self, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;
use std::fs::File;



/// Создаём файл big.txt, если его ещё нет.
/// 100_0 строк с текстом и номерами.
fn ensure_file() -> io::Result<()> {
    if Path::new("big.txt").exists() {
        return Ok(());
    }
    println!("Создается файл big.txt...");
    let mut f = OpenOptions::new().create(true).write(true).open("big.txt")?;
    for i in 0..100_0 {
        writeln!(f, "Строка номер {i:06}")?;
    }
    println!("Файл создан!");
    Ok(())
}
fn main() -> io::Result<()> {
    ensure_file()?; // создаём файл

    let start = std::time::Instant::now();
    let mut lines_count = 0;
    let file = File::open("big.txt")?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        lines_count += 1;
        line.clear(); // строка очищается, но не выделяется заново
    }

    let end = std::time::Instant::now();
    println!("Время выполнения: {:?}", end - start);

    Ok(())
}
```

3. *BufReader + переиспользование строки*
⚡ Ещё быстрее {{< badge >}} ~360µs {{< /badge >}}
read_line записывает прямо в существующую строку.
Нет лишних аллокаций памяти, работает эффективнее.


#### Шаг 4

```rust
use std::fs::{self, OpenOptions};
use std::io::{self, Write, Read};
use std::path::Path;
use std::fs::File;

/// Создаём файл big.txt, если его ещё нет.
/// 100_0 строк с текстом и номерами.
fn ensure_file() -> io::Result<()> {
    if Path::new("big.txt").exists() {
        return Ok(());
    }
    println!("Создается файл big.txt...");
    let mut f = OpenOptions::new().create(true).write(true).open("big.txt")?;
    for i in 0..100_0 {
        writeln!(f, "Строка номер {i:06}")?;
    }
    println!("Файл создан!");
    Ok(())
}
fn main() -> io::Result<()> {

    ensure_file()?; // создаём файл
    let start = std::time::Instant::now();

    let mut lines_count = 0;
    let mut file = File::open("big.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    for line in contents.lines() {
        lines_count += 1;
    }

    let end = std::time::Instant::now();
    println!("Время выполнения: {:?}", end - start);

    Ok(())
}
```

*4. Еще один вариант*
⚡ {{< badge >}} ~270µs {{< /badge >}}
Всё содержимое читается за раз, а потом разбивается на строки. 
Время выполнения у вас может отличаться от представленного здесь.
Минус: выделяется объём памяти равный размеру файла.


## Когда BufReader не нужен

Хотя `BufReader` полезен в большинстве сценариев, есть ситуации, когда он не только не ускорит программу, но и будет лишним.

### 1. Данные уже в памяти

Если у вас есть `Vec<u8>`, `String` или срез `&[u8]`, никакого дополнительного системного вызова не происходит. Данные уже находятся в оперативной памяти, а значит никакой выгоды от дополнительного буфера не будет.

### 2. Чтение крупными блоками

Если вы сразу читаете данные большими кусками (десятки или сотни килобайт), то накладные расходы на системные вызовы становятся несущественными.

### 3. Ограниченная память

Если у вас тысячи одновременных соединений (например, работаете с сервером), каждый `BufReader` создаёт внутренний буфер `(по умолчанию 8 КБ)`. При тысячах соединений это может означать десятки мегабайт памяти, которые не всегда оправданы.

В асинхронном коде часто используют другие подходы: читают ровно столько - сколько пришло в сокете и сами управляют буферами чтобы экономить память.

____

## Вывод

* `Read` — низкоуровневый контракт для чтения потоков байтов.
* Каждый вызов `.read()` - это системный вызов и он дорогой.
* `BufReader` сокращает число системных вызовов и делает чтение в разы быстрее.

На практике BufReader с повторным использованием строки даёт лучший баланс между скоростью и памятью.

